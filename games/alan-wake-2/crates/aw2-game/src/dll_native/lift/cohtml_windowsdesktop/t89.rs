#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a0ed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000697150; const P1: u64 = 0x70005a4d1a; const P2: u64 = 0x700062e2c0; const P3: u64 = 0x70005a1330;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = (rdx) as u64; crate::simd_ops::wu64(rdx, (rax) as u64); rsi = (rcx) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rax) as u64); rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = ((rdi.wrapping_mul(8)) as u32) as u32 as u64; r8 = (0x18u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (rax) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rbp = (rax) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = (0x18u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx, (rbp) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; rbp = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru64(P2)) as u64) as u128; bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32) as u128; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rsi = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm0); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm7 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm0); xmm6 = crate::simd_ops::vcvtps2pd(xmm0); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm6 as u64)) as u64); fl_l = (rcx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64 as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), (rax) as u64); bb = 10; continue; }
            9 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rax, ((xmm6 as u64)) as u64); bb = 10; continue; }
            10 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rdi = ((rdi).wrapping_add(0x38u64)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = (rdi) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 11; continue; }
            11 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 12; continue; }
            12 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a1020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            3 => { r8 = ((r8).wrapping_add(0x10u64)) as u64; fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a1040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a14a0; const P1: u64 = 0x700062e490; const P2: u64 = 0x700062e338; const P3: u64 = 0x70005a4d32; const P4: u64 = 0x7000697150; const P5: u64 = 0x70005a1530; const P6: u64 = 0x700062dba0; const P7: u64 = 0x7000237500;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x3cu64))) as u32 as u64; rsi = (r9) as u64; rbp = (r8) as u64; r14 = (rcx) as u64; fl_l = (r9) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (r9) as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (r12) as u64); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); r12 = (0x8000000000000000u64) as u64; xmm6 = ((crate::simd_ops::ru64(P1)) as u64) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru64(P2)) as u64) as u128; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru32(r14.wrapping_add(0x3cu64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x38u64))) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = (((rcx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; bb = 10; continue; }
            6 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rbx) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm7 as u64)).to_bits() as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm6 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 9; continue; } bb = 7; continue; }
            7 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm6 as u64)).to_bits() as u128); fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm6 as u64)) as u64; if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (r12) as u64; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm0 as u64)))) as u64; rdx = ((rbx.wrapping_add(0x1u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rcx) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rdx }) as u64; rcx = ((r14.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x38u64))) as u32 as u64; rbx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x30u64)))) as u64; rdx = (0x30u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); r8 = ((rdx.wrapping_sub(0x18u64)) as u32) as u32 as u64; xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = ((rbp).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rax, (0x1u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (r15) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (r15) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (r15) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), (r15) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (r15) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (r15) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (r15) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x30u64), ((r15 as u8)) as u8); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x38u64), (1) as u32); fl_l = (rsi) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 11; continue; }
            11 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x38u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; r9 = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = (0x4924924924924925u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r8 = (rdx) as u64; rdx = ((r9.wrapping_add(rcx)) as u64) as u64; r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64; r8 = ((((r8) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (r8) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (r15) as u64; fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 23; continue; } bb = 13; continue; }
            13 => { rbp = (r15) as u64; r12 = ((P6) as u64) as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbp).wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = (((rax) & (0xfffffffffffffffeu64))) as u64; rcx = ((rbx.wrapping_add(0x10u64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { r8 = (0x2u32) as u32 as u64; rdx = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((r9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (r15) as u64); bb = 19; continue; }
            19 => { rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rsi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = (r12) as u64; fl_l = (((rax) | (0x1u64))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rax) as u64); bb = 22; continue; }
            21 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (r15) as u64); bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x38u64))) as u32 as u64; rsi = ((rsi).wrapping_add(1)) as u64; rbp = ((rbp).wrapping_add(0x38u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a1280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x6c; const P1: u64 = 0x10; const P2: u64 = 0x58;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a1290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x20; const P4: u64 = 0x28;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = (r9) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = (r8) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; r10 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru64(rdx)) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = (rsi) as u64; rdx = (rbp) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = (rdx) as u64; bb = 4; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a1330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x700062e338; const P3: u64 = 0x70005a4d32; const P4: u64 = 0x700062e490; const P5: u64 = 0x7000697150; const P6: u64 = 0x1; const P7: u64 = 0x70005a4d1a; const P8: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = (rcx) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { xmm0 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rdi) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P2))).to_bits() as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = ((crate::simd_ops::ru64(P4)) as u64) as u128; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 5; continue; } bb = 3; continue; }
            3 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm1 as u64)).to_bits() as u128); fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (0x8000000000000000u64) as u64; rax = (rcx) as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rsi = ((rdi.wrapping_add(P6)) as u64) as u64; rdx = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm0 as u64)))) as u64; r8 = (0x18u32) as u32 as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rdx) as u64 as u64; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { (rdx as u32) } else { (rsi as u32) }) as u32 as u64; rdx = ((rsi.wrapping_mul(8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rcx = (rax) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rdi = (rax) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P5)) as u64; r8 = (0x18u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(P8))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx, (rdi) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a1420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x700059f7a0; const P2: u64 = 0xc; const P3: u64 = 0x7000697150; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; rdi = (rcx) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = (rsi) as u64; rdx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = ((rbx).wrapping_add(0x38u64)) as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = (0x18u32) as u32 as u64; rdx = (rsi) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rdi.wrapping_add(P2), (0x0u32) as u32); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a14a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000697150; const P1: u64 = 0x8; const P2: u64 = 0x70005a17a0; const P3: u64 = 0x70005a1420; const P4: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (rcx) as u64; r15 = (rdx) as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = (0x18u32) as u32 as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul((0x38u32) as i32) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(r14)) as u64; rbp = (rax) as u64; r8 = (crate::simd_ops::ru32(r14.wrapping_add(P1))) as u32 as u64; rsi = (((r8) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdi) as u64); rdi = (rax) as u64; bb = 2; continue; }
            2 => { rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rbx).wrapping_add(0x38u64)) as u64; rdi = ((rdi).wrapping_add(0x38u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 4; continue; }
            4 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(r14, (rbp) as u64); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(r14.wrapping_add(P4), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a1530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a1be0; const P1: u64 = 0x70005a1530; const P2: u64 = 0x70005a1830; const P3: u64 = 0x7000598750; const P4: u64 = 0x70005a4d1a; const P5: u64 = 0x70005a2140; const P6: u64 = 0x7000697150; const P7: u64 = 0x700024f700; const P8: u64 = 0x70005a1a10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (rdx) as u64; rbx = (((r9 as u8)) as u32) as u32 as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rdi = (r8) as u64; rbp = (rdx) as u64; rsi = (rcx) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x738u64) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 1; continue; }
            1 => { r12 = (0x4924924924924925u64) as u64; bb = 2; continue; }
            2 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { r9 = (((rbx as u8)) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; r8 = (rbp) as u64; rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (rdi) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (r12) as u64; rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; r9 = (((rbx as u8)) as u32) as u32 as u64; rdi = ((((rdi) as i64).wrapping_shr(1u32) as u64)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; rcx = (rbp) as u64; rcx = ((rcx).wrapping_sub(r14)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = (r15) as u64; r8 = (rdx) as u64; rcx = ((rcx).wrapping_sub(rsi)) as u64; r8 = ((((r8) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (r8) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rax = (r12) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; r8 = (rdi) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (r15) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (r14) as u64; bb = 6; continue; }
            5 => { rdx = (rbp) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = (r15) as u64; bb = 6; continue; }
            6 => { rax = (rbp) as u64; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x738u64) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 7; continue; }
            7 => { rdx = (rbp) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = (r12) as u64; rcx = (rbp) as u64; rcx = ((rcx).wrapping_sub(rsi)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r12 = (rdx) as u64; r12 = ((((r12) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (r12) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; r12 = ((r12).wrapping_add(rax)) as u64; r15 = (r12) as u64; r15 = ((((r15) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 24; continue; } bb = 10; continue; }
            10 => { rdi = (((r15) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), (r13) as u64); rdi = ((rdi).wrapping_add(0xfffffffffffffff0u64)) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x10u64))) as u64; xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_sub(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_sub(0x28u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 13; continue; }
            13 => { r14 = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (r13) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (r13) as u64); fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((r14 as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rdi)) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_sub(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((r14 as u32)) as u32); bb = 15; continue; }
            15 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u32) as u32 as u64; rdx = ((r15.wrapping_sub(0x1u64)) as u64) as u64; r9 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rax as u8)) as u8); r8 = (r12) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rbx as u8)) as u8); rcx = (rsi) as u64; r15 = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (((rax) & (0xfffffffffffffffeu64))) as u64; rcx = ((r14.wrapping_add(0x10u64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = (0x2u32) as u32 as u64; rdx = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((r9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            19 => { crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (r13) as u64); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x6cu64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(P6)) as u64; r8 = (0x18u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((r13 as u32)) as u32); bb = 22; continue; }
            22 => { rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rdi).wrapping_sub(0x38u64)) as u64; fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 11; continue; } bb = 23; continue; }
            23 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 24; continue; }
            24 => { r8 = (((rbx as u8)) as u32) as u32 as u64; rdx = (rbp) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a17a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x7000598750; const P5: u64 = 0x70005a4d1a; const P6: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rdi = (rdx) as u64; rbx = (rcx) as u64; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P2), (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P3), (rax) as u64); rbp = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rbp as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x20u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rbp as u32)) as u32); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P6))) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P6), ((rax as u8)) as u8); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a1830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000598750; const P1: u64 = 0x70005a4d1a; const P2: u64 = 0x70005a1de0; const P3: u64 = 0x7000697150; const P4: u64 = 0x700024f700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; r15 = (rdx) as u64; rbp = (rcx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rdi) as u64); rdi = ((rcx.wrapping_add(0x38u64)) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (r12) as u64); r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), (r14) as u64); r14 = ((rdi.wrapping_add(0x28u64)) as u64) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r14.wrapping_sub(0x10u64))) as u64; rsi = (rdi) as u64; xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r12) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rbx as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(r14)) as u32 as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_sub(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rbx as u32)) as u32); bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32) as u128; xmm6 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rax as u8)) as u8); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rbx = (rdi) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rcx = (rbx) as u64; rbx = ((rbx).wrapping_sub(0x38u64)) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rsi = (rbp) as u64; bb = 15; continue; }
            12 => { xmm0 = ((crate::simd_ops::ru32(r14.wrapping_sub(0x50u64))) as u32) as u128; rbx = ((r14.wrapping_sub(0x60u64)) as u64) as u64; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rdx = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_sub(0x28u64))) as u32) as u128; rsi = (rbx) as u64; rbx = ((rbx).wrapping_sub(0x38u64)) as u64; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (((rax) & (0xfffffffffffffffeu64))) as u64; rcx = ((rbx.wrapping_add(0x10u64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = (0x2u32) as u32 as u64; rdx = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((r9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (r12) as u64); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = (0x18u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((r12 as u32)) as u32); bb = 22; continue; }
            22 => { rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rdi).wrapping_add(0x38u64)) as u64; r14 = ((r14).wrapping_add(0x38u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 23; continue; }
            23 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 24; continue; }
            24 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = (r15) as u64; rsp = ((rsp).wrapping_add(0x78u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { rax = (r15) as u64; rsp = ((rsp).wrapping_add(0x78u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a1a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000598750; const P1: u64 = 0x70005a4d1a; const P2: u64 = 0x70005a1de0; const P3: u64 = 0x70005a2140; const P4: u64 = 0x7000697150; const P5: u64 = 0x700024f700;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x88u64)) as u64; rax = (rdx) as u64; rbx = (((r8 as u8)) as u32) as u32 as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r15 = (rcx) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x70u64) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 17; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (rdi) as u64); rax = (0x4924924924924925u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), (r12) as u64); rdi = ((rdx.wrapping_sub(0x10u64)) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), (r13) as u64); r12 = (0xffffffffffffffd8u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), (r14) as u64); r12 = ((r12).wrapping_sub(rcx)) as u64; r14 = (0x10u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rbp) as u64); r14 = ((r14).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), (rsi) as u64); r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rcx = ((r14.wrapping_add(rdi)) as u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 15; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x10u64))) as u64; xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_sub(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_sub(0x28u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r13) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r13) as u64); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rsi as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rdi)) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_sub(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rsi as u32)) as u32); bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u32) as u32 as u64; rcx = ((rdi.wrapping_sub(0x28u64)) as u64) as u64; rdx = (r15) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), ((rax as u8)) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((r12.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rbx as u8)) as u8); rax = (0x4924924924924925u64) as u64; r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = (r15) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; r8 = (rdx) as u64; r8 = ((r8).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (((rax) & (0xfffffffffffffffeu64))) as u64; rcx = ((rsi.wrapping_add(0x10u64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = (0x2u32) as u32 as u64; rdx = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((r9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), (r13) as u64); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x5cu64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(P4)) as u64; r8 = (0x18u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((r13 as u32)) as u32); bb = 14; continue; }
            14 => { rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            15 => { rdi = ((rdi).wrapping_sub(0x38u64)) as u64; rax = ((r14.wrapping_add(rdi)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x70u64) as u64 as u64; rax = (0x4924924924924925u64) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 16; continue; }
            16 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 17; continue; }
            17 => { rsp = ((rsp).wrapping_add(0x88u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a1be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a2250; const P1: u64 = 0x70005a2440;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r12 = (rdx) as u64; r10 = (r8) as u64; r10 = ((r10).wrapping_sub(rdx)) as u64; rax = (0x4924924924924925u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r15 = (r8) as u64; rcx = (r12) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; r8 = ((r8).wrapping_add(0xffffffffffffffc8u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; rbx = (((rdx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(r12)) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = ((rbx.wrapping_add(0x38u64)) as u64) as u64; fl_l = (r12) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_sub(0x28u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_sub(0x38u64)) as u64; fl_l = (r12) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { fl_l = (rsi) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32) as u128; bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rsi = ((rsi).wrapping_add(0x38u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rdi = (rsi) as u64; r14 = (rbx) as u64; bb = 11; continue; }
            11 => { fl_l = (rdi) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (rsi) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = (rdi) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { fl_l = ((rsi).wrapping_add(0x38u64)) as u64 as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rdi = ((rdi).wrapping_add(0x38u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 19; continue; }
            19 => { r13 = (rdi) as u64; fl_l = (r12) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 20; continue; }
            20 => { rbp = ((r14.wrapping_sub(0x38u64)) as u64) as u64; bb = 21; continue; }
            21 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 25; continue; } bb = 22; continue; }
            22 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 26; continue; } bb = 23; continue; }
            23 => { rbx = ((rbx).wrapping_sub(0x38u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = (rbp) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 25; continue; }
            25 => { r14 = ((r14).wrapping_sub(0x38u64)) as u64; rbp = ((rbp).wrapping_sub(0x38u64)) as u64; fl_l = (r12) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 26; continue; }
            26 => { rbp = (rbx) as u64; fl_l = (r14) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 27; continue; }
            27 => { fl_l = (rdi) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 28; continue; }
            28 => { fl_l = (rsi) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdx = (rsi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            30 => { rsi = ((rsi).wrapping_add(0x38u64)) as u64; rbx = ((rbx).wrapping_add(0x38u64)) as u64; rdi = ((rdi).wrapping_add(0x38u64)) as u64; rdx = (r13) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            31 => { r14 = ((r14).wrapping_add(0xffffffffffffffc8u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 32; continue; }
            32 => { rbx = ((rbx).wrapping_sub(0x38u64)) as u64; fl_l = (r14) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdx = (rbx) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 34; continue; }
            34 => { rsi = ((rsi).wrapping_sub(0x38u64)) as u64; rcx = (rbx) as u64; rdx = (rsi) as u64; rbp = (rbx) as u64; r13 = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            35 => { rbp = (r13) as u64; rdi = ((rdi).wrapping_add(0x38u64)) as u64; rcx = (rbp) as u64; rdx = (r14) as u64; r13 = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            36 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax, (rbx) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rsi) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a1de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000697150; const P1: u64 = 0x700062e338; const P2: u64 = 0x70005a4d32; const P3: u64 = 0x700062e490; const P4: u64 = 0x7000598750; const P5: u64 = 0x70005a4d1a;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbp = (rdx) as u64; rdi = (rcx) as u64; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (rax) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (((rax) & (0xfffffffffffffffeu64))) as u64; rcx = ((rbx.wrapping_add(0x10u64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = (0x2u32) as u32 as u64; rdx = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((r9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (r15) as u64); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = (0x18u32) as u32 as u64; rdx = (rbx) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rsi = ((rbp.wrapping_add(0x20u64)) as u64) as u64; rbx = ((rdi.wrapping_add(0x20u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r14) as u64); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = (0x18u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((r15 as u32)) as u32); bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r15 as u32)) as u32); r15 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; fl_l = (r14) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((r14) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = ((crate::simd_ops::ru64(P3)) as u64) as u128; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 18; continue; } bb = 16; continue; }
            16 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm1 as u64)).to_bits() as u128); fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (0x8000000000000000u64) as u64; rcx = (rax) as u64; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm0 as u64)))) as u64; rdx = ((r14.wrapping_add(r15)) as u64) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (rbx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); bb = 20; continue; }
            20 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x30u64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x30u64), ((rax as u8)) as u8); rax = (rdi) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a1f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x30; const P2: u64 = 0x28; const P3: u64 = 0x2c; const P4: u64 = 0x20; const P5: u64 = 0x7000598750; const P6: u64 = 0x70005a4c62; const P7: u64 = 0x70005a4d14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (r9) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = (r8) as u64; rsi = (rcx) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; crate::simd_ops::wu8(rax.wrapping_add(rdx).wrapping_add(P1), (0x0u8) as u8); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); rdi = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (rbx) as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rcx = ((rcx).wrapping_add(r8)) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rbx as u32)) as u32); bb = 9; continue; }
            9 => { r9 = (((crate::simd_ops::ru64(rsi.wrapping_add(P0))) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = (rbx) as u64; rdx = (rbp) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(r9).wrapping_add(P4))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a20a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64;
    r10 = (rcx) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(r10)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a2140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a1de0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((r8.wrapping_sub(0x1u64)) as u64) as u64; rbp = (r9) as u64; rsi = ((((rsi) as i64).wrapping_shr(1u32) as u64)) as u64; r15 = (r8) as u64; r10 = (rdx) as u64; rdi = (rcx) as u64; r14 = (rdx) as u64; rbx = (rdx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(rbx)) as u64) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (((rcx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(rdi).wrapping_add(0x48u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(rdi).wrapping_add(0x80u64))) as u32 as u64; rbx = (rbx & !0xffu64) | (((if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = ((rcx).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; rcx = (((r10) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rdx = (((rbx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r10 = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 3; continue; }
            3 => { fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (((r15) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rcx = (((r10) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0xffffffffffffffc8u64)) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r10 = ((r15.wrapping_sub(0x1u64)) as u64) as u64; bb = 6; continue; }
            6 => { fl_l = (r14) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32) as u128; rbx = ((r10.wrapping_sub(0x1u64)) as u64) as u64; rbx = ((((rbx) as i64).wrapping_shr(1u32) as u64)) as u64; rdx = (((rbx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (((r10) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r10 = (rbx) as u64; fl_l = (r14) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 10; continue; }
            10 => { rcx = (((r10) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rdx = (rbp) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a2250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a2440;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32) as u128; rbx = (rdx) as u64; r9 = (r8) as u64; rax = (0x4924924924924925u64) as u64; r9 = ((r9).wrapping_sub(rcx)) as u64; rbp = (r8) as u64; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = (rcx) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x28u64) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 23; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rsi) as u64); rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r12) as u64); rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r14) as u64); r12 = (((rax) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r15) as u64); r15 = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(rcx).wrapping_add(0x10u64))) as u32 as u64; r14 = ((r12.wrapping_add(rcx)) as u64) as u64; rsi = ((r15.wrapping_add(rcx)) as u64) as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (rcx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (rsi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (rdi) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { rdi = (rbx) as u64; r14 = ((r15.wrapping_add(rbx)) as u64) as u64; rdi = ((rdi).wrapping_sub(r15)) as u64; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = (rbx) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { r14 = (rbp) as u64; rdi = (rbp) as u64; r14 = ((r14).wrapping_sub(r12)) as u64; rdi = ((rdi).wrapping_sub(r15)) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; xmm0 = ((crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = (r14) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rdx = (rdi) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = ((crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = (r14) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = (rsi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = (rsi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            21 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 22; continue; }
            22 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 25; continue; }
            25 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 26; continue; }
            26 => { rdx = (rbx) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 27; continue; }
            27 => { rdx = (rdi) as u64; rcx = (rbx) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a2440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000598750; const P1: u64 = 0x70005a4d1a; const P2: u64 = 0x70005a1de0; const P3: u64 = 0x7000697150; const P4: u64 = 0x700024f700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rsi = (rdx) as u64; xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64) as u128; rbx = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rbp) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdi as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x30u64))) as u32) as u32 as u64; rdx = (rsi) as u64; rcx = (rbx) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rax as u8)) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (((rax) & (0xfffffffffffffffeu64))) as u64; rcx = ((rbx.wrapping_add(0x10u64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = (0x2u32) as u32 as u64; rdx = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((r9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rbp) as u64); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = (0x18u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a2550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000598750; const P1: u64 = 0x70005a2a60; const P2: u64 = 0x70005a4d1a; const P3: u64 = 0x70005a26b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbp = (rdx) as u64; crate::simd_ops::wu64(rcx, (rbx) as u64); r14 = (rcx) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rbx) as u64); rax = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rbx) as u64); rsi = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rbx) as u64); bb = 1; continue; }
            1 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(rdx))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rdx = ((rsi.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rdx = ((rsi as u32)) as u32 as u64; rcx = ((r14.wrapping_add(0x18u64)) as u64) as u64; crate::simd_ops::wu32(r14.wrapping_add(0x20u64), ((rbx as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; r8 = (rsi) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x18u64)))) as u64; rdx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (((rcx as u32)).wrapping_add((rsi as u32))) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rbx as u8)) as u8); crate::simd_ops::wu32(r14.wrapping_add(0x20u64), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = ((rcx as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (0x1u32) as u32 as u64; rcx = ((r14.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = (rdx) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(rax).wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(rdx), ((rax as u8)) as u8); bb = 8; continue; }
            7 => { rdx = (0x1u32) as u32 as u64; rcx = ((r14.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rax, (0x0u8) as u8); bb = 8; continue; }
            8 => { crate::simd_ops::lk_add_u32(r14.wrapping_add(0x20u64), (1) as u32); rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(r14))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rdx = ((((rcx as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax), (0x0u8) as u8); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r9 = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (((rdx as u32)).wrapping_add(crate::simd_ops::ru32(rax))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; r8 = ((((rdx as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(r8.wrapping_add(rcx), (0x0u8) as u8); bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; rdx = ((((rcx as u32)) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rcx), (0x0u8) as u8); bb = 15; continue; }
            15 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (r14) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a26b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a3940; const P1: u64 = 0x70005929c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r12 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r15 = (rcx) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; r8 = (r8 & !0xffu64) | (((0x3bu8) as u8 as u64)); rbp = ((rbp).wrapping_add(r12)) as u64; rcx = (r12) as u64; rdx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 99; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = (rax) as u64; rcx = ((rcx).wrapping_sub(r12)) as u64; rdx = ((r8.wrapping_sub(0x1u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 9; continue; }
            9 => { fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; bb = 10; continue; }
            10 => { if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { rdx = (((rdx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(r15.wrapping_add(0x18u64)))) as u32 as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(32u32)) as u64; fl_l = (((rcx) | (rax))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 19; continue; }
            16 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 17; continue; }
            17 => { rdx = ((rdx).wrapping_sub(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 18; continue; }
            18 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rdi = ((rdi).wrapping_add(1)) as u64; crate::simd_ops::wu64(r15, (rcx) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 98; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); bb = 21; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x3du8) as u8 as u64)); rdx = (rbp) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 97; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_sub(r12)) as u64; rdx = ((rcx.wrapping_sub(0x1u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rcx.wrapping_add(rdi)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 30; continue; } bb = 23; continue; }
            23 => { rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 24; continue; }
            24 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 27; continue; } bb = 25; continue; }
            25 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 29; continue; } bb = 26; continue; }
            26 => { bb = 28; continue; }
            27 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 29; continue; }
            29 => { fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; bb = 30; continue; }
            30 => { if (((fl_l as u64)) < ((fl_r as u64))) { bb = 38; continue; } bb = 31; continue; }
            31 => { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 37; continue; } bb = 32; continue; }
            32 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 36; continue; } bb = 33; continue; }
            33 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 37; continue; } bb = 34; continue; }
            34 => { fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 38; continue; } bb = 35; continue; }
            35 => { rdx = (((rdx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub(crate::simd_ops::ru32(r15.wrapping_add(0x18u64)))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; r14 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; r14 = ((r14).wrapping_shl(32u32)) as u64; fl_l = (((r14) | (rax))) as u64 as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 39; continue; }
            36 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 37; continue; }
            37 => { rdx = ((rdx).wrapping_sub(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 31; continue; } bb = 38; continue; }
            38 => { r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 39; continue; }
            39 => { rbx = ((rsi.wrapping_add(0x1u64)) as u64) as u64; r8 = (r8 & !0xffu64) | (((0x22u8) as u8 as u64)); rcx = (rbx) as u64; rdx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 45; continue; } bb = 40; continue; }
            40 => { rcx = ((rax.wrapping_add(0x1u64)) as u64) as u64; r8 = (r8 & !0xffu64) | (((0x22u8) as u8 as u64)); rdx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 49; continue; } bb = 41; continue; }
            41 => { bb = 42; continue; }
            42 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 48; continue; } bb = 43; continue; }
            43 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 47; continue; } bb = 44; continue; }
            44 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 48; continue; } bb = 45; continue; }
            45 => { rax = ((rsi.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 71; continue; } bb = 46; continue; }
            46 => { rcx = ((rsi.wrapping_add(0x2u64)) as u64) as u64; r8 = (r8 & !0xffu64) | (((0x3bu8) as u8 as u64)); rdx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; bb = 72; continue; }
            47 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 45; continue; } bb = 48; continue; }
            48 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 42; continue; } bb = 49; continue; }
            49 => { fl_l = (rax) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 45; continue; } bb = 50; continue; }
            50 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_sub(r12)) as u64; rdx = ((rcx.wrapping_sub(0x1u64)) as u64) as u64; r8 = ((rcx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((r8).wrapping_add(rdi)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 58; continue; } bb = 51; continue; }
            51 => { rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 56; continue; } bb = 52; continue; }
            52 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 55; continue; } bb = 53; continue; }
            53 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 57; continue; } bb = 54; continue; }
            54 => { bb = 56; continue; }
            55 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 57; continue; } bb = 56; continue; }
            56 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 51; continue; } bb = 57; continue; }
            57 => { fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; bb = 58; continue; }
            58 => { if (((fl_l as u64)) < ((fl_r as u64))) { bb = 67; continue; } bb = 59; continue; }
            59 => { rcx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 66; continue; } bb = 60; continue; }
            60 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 65; continue; } bb = 61; continue; }
            61 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 66; continue; } bb = 62; continue; }
            62 => { r9 = (rax) as u64; bb = 63; continue; }
            63 => { rax = (r9) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 67; continue; } bb = 64; continue; }
            64 => { rdx = (((rdx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub(crate::simd_ops::ru32(r15.wrapping_add(0x18u64)))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rbx = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rbx = ((rbx).wrapping_shl(32u32)) as u64; fl_l = (((rbx) | (rax))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rax = (r9) as u64; bb = 68; continue; }
            65 => { r9 = (rax) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 63; continue; } bb = 66; continue; }
            66 => { rdx = ((rdx).wrapping_sub(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 59; continue; } bb = 67; continue; }
            67 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 68; continue; }
            68 => { rcx = ((rax.wrapping_add(0x1u64)) as u64) as u64; r8 = (r8 & !0xffu64) | (((0x3bu8) as u8 as u64)); rdx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 70; continue; } bb = 69; continue; }
            69 => { rdi = (rbp) as u64; bb = 92; continue; }
            70 => { rdi = ((rax.wrapping_add(0x1u64)) as u64) as u64; bb = 92; continue; }
            71 => { rdi = (rbp) as u64; bb = 72; continue; }
            72 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rax = ((rax).wrapping_sub(r12)) as u64; rcx = ((rax.wrapping_sub(0x1u64)) as u64) as u64; rdx = ((rax.wrapping_add(0x1u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 80; continue; } bb = 73; continue; }
            73 => { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 78; continue; } bb = 74; continue; }
            74 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 77; continue; } bb = 75; continue; }
            75 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 79; continue; } bb = 76; continue; }
            76 => { bb = 78; continue; }
            77 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 79; continue; } bb = 78; continue; }
            78 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 73; continue; } bb = 79; continue; }
            79 => { fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; bb = 80; continue; }
            80 => { if (((fl_l as u64)) < ((fl_r as u64))) { bb = 88; continue; } bb = 81; continue; }
            81 => { rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 87; continue; } bb = 82; continue; }
            82 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 86; continue; } bb = 83; continue; }
            83 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 87; continue; } bb = 84; continue; }
            84 => { fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 88; continue; } bb = 85; continue; }
            85 => { rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(r15.wrapping_add(0x18u64)))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rbx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rbx = ((rbx).wrapping_shl(32u32)) as u64; fl_l = (((rbx) | (rax))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 89; continue; }
            86 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 84; continue; } bb = 87; continue; }
            87 => { rcx = ((rcx).wrapping_sub(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 81; continue; } bb = 88; continue; }
            88 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 89; continue; }
            89 => { fl_l = (rdi) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 91; continue; } bb = 90; continue; }
            90 => { rdi = (rbp) as u64; bb = 92; continue; }
            91 => { fl_l = ((rdi).wrapping_add(1)) as u64 as u64; rdi = ((fl_l as u64)) as u64; bb = 92; continue; }
            92 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 95; continue; } bb = 93; continue; }
            93 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 95; continue; } bb = 94; continue; }
            94 => { rcx = ((r15.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; fl_l = ((rax).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rdx))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128); bb = 96; continue; }
            95 => { rcx = ((r15.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); bb = 96; continue; }
            96 => { crate::simd_ops::lk_add_u32(r15.wrapping_add(0x10u64), (1) as u32); fl_l = (rdi) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 97; continue; }
            97 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 98; continue; }
            98 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            99 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x4u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a2b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a2c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x780u32) as u32);
    rax = (rdx) as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x438u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0x18u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x18u32) as u32);
    crate::simd_ops::wu32(rdx, (0x780u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), (0x438u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a2d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000697150; const P1: u64 = 0x70005a4d1a; const P2: u64 = 0x70005a31f0; const P3: u64 = 0x700019ea00; const P4: u64 = 0x700062dbe8; const P5: u64 = 0x70005a86b0; const P6: u64 = 0x700062dbf8; const P7: u64 = 0x700062dc00; const P8: u64 = 0x70005a86a8; const P9: u64 = 0x700019e990; const P10: u64 = 0x70005a4c62; const P11: u64 = 0x700062dc20; const P12: u64 = 0x70005a86a0; const P13: u64 = 0x70005a8698; const P14: u64 = 0x70005a8690; const P15: u64 = 0x70005f6800; const P16: u64 = 0x700062dc48; const P17: u64 = 0x70005a8688; const P18: u64 = 0x70005a8680; const P19: u64 = 0x70005a31e0; const P20: u64 = 0x70005a31d0; const P21: u64 = 0x70005a31c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rbp = ((rax.wrapping_sub(0x18u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x108u64)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), (rdi) as u64); rbx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), (r13) as u64); rdi = (rcx) as u64; bb = 1; continue; }
            1 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x100u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r13) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r13) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = ((r13.wrapping_add(0x2u64)) as u32) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = (rax) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = ((r13.wrapping_add(0x2u64)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rsi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((rbx as u32)) as u32); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), (r12) as u64); rdx = ((rbx.wrapping_add(rdi)) as u64) as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), (r14) as u64); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 44; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru32(rdx)) as u32 as u64; rcx = ((r14.wrapping_add(0x4u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 44; continue; } bb = 8; continue; }
            8 => { rdi = ((rdx.wrapping_add(0x4u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), (r15) as u64); rsi = ((r14.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (r13) as u64); rbx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (r13) as u64); rbx = (((rbx as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rdx = ((rbx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r15 = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; r8 = ((r14 as u32)) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = (rdi) as u64; rcx = ((r12.wrapping_add(r15)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; r15 = (((r15 as u32)).wrapping_add((rbx as u32))) as u32 as u64; rbx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((r15 as u32)) as u32); rbx = (((rbx as u32)).wrapping_sub((rsi as u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r13) as u64); rdx = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r13) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; rdi = ((rdi).wrapping_sub(rsi)) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r8 = (rdi) as u64; rdx = (rsi) as u64; rcx = ((r15.wrapping_add(r14)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = (((r14 as u32)).wrapping_add((rbx as u32))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), (r13) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((r14 as u32)) as u32); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((P4) as u64) as u64; rcx = ((rbp.wrapping_sub(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 39; continue; }
            10 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; rdx = ((P6) as u64) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (r13) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 37; continue; }
            12 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; rax = ((rbp.wrapping_add(0x28u64)) as u64) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (r13) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); r8 = ((rbp.wrapping_add(0x20u64)) as u64) as u64; rdx = ((P7) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((rdi as u32)) as u32); rcx = (r14) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), ((rdi as u32)) as u32); r9 = ((rdi.wrapping_add(0x4u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 35; continue; }
            14 => { rbx = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; rsi = (rdi) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), (rdi) as u64); r15 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (rdi) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = ((rbx as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x70u64))) as u32 as u64; r8 = (rsi) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r15 = ((rbx as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rax = ((rbp.wrapping_add(0x28u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); r9 = (0x4u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r8 = ((rbp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x30u64), ((rdi as u32)) as u32); rdx = ((P11) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 32; continue; }
            18 => { rbx = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (r12) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = ((rbx as u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x80u64))) as u32 as u64; r8 = (rdi) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r12 = ((rbx as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_sub(0x58u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); r9 = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); r8 = (rsi) as u64; rcx = (r14) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 29; continue; } bb = 21; continue; }
            21 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; rcx = (r15) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 27; continue; } bb = 22; continue; }
            22 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = ((r12 as u32)) as u32 as u64; rdx = (rdi) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P14)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); rax = ((P15) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x11bu32) as u32); r9 = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; r8 = ((P16) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rbx) as u64); rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P17)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); rdx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; r9 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x2u32) as u32); r8 = (rdi) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = (r13) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P18)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_shr(31u32)) as u32 as u64; rbx = (rbx & !0xffu64) | ((((((rbx as u8)) ^ (0x1u8))) as u8 as u64)); fl_l = (r13) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 26; continue; }
            26 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; bb = 27; continue; }
            27 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x7cu64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = (0x2u32) as u32 as u64; rdx = (rdi) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 31; continue; }
            31 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x6cu64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = (0x2u32) as u32 as u64; rdx = (rsi) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 34; continue; }
            34 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 35; continue; }
            35 => { fl_l = (r13) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 37; continue; }
            37 => { fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x5cu64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = (0x2u32) as u32 as u64; rdx = (r15) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x6cu64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = (0x2u32) as u32 as u64; rdx = (r12) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 43; continue; }
            43 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 45; continue; }
            44 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = (0x2u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 47; continue; }
            47 => { rax = (((rbx as u8)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x108u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a31c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a86c8; const P1: u64 = 0x70005a31c9; const P2: u64 = 0x70005a31ca; const P3: u64 = 0x70005a31cb; const P4: u64 = 0x70005a31cc; const P5: u64 = 0x70005a31cd; const P6: u64 = 0x70005a31ce; const P7: u64 = 0x70005a31cf; const P8: u64 = 0x70005a86c0; const P9: u64 = 0x70005a31d7; const P10: u64 = 0x70005a31d8; const P11: u64 = 0x70005a31d9; const P12: u64 = 0x70005a31da; const P13: u64 = 0x70005a31db; const P14: u64 = 0x70005a31dc; const P15: u64 = 0x70005a31dd; const P16: u64 = 0x70005a31de; const P17: u64 = 0x70005a31df; const P18: u64 = 0x70005a86b8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { crate::lift_ops::trap(0x80000003, P1, rcx, 0); return rax; }
            2 => { crate::lift_ops::trap(0x80000003, P2, rcx, 0); return rax; }
            3 => { crate::lift_ops::trap(0x80000003, P3, rcx, 0); return rax; }
            4 => { crate::lift_ops::trap(0x80000003, P4, rcx, 0); return rax; }
            5 => { crate::lift_ops::trap(0x80000003, P5, rcx, 0); return rax; }
            6 => { crate::lift_ops::trap(0x80000003, P6, rcx, 0); return rax; }
            7 => { crate::lift_ops::trap(0x80000003, P7, rcx, 0); return rax; }
            8 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { crate::lift_ops::trap(0x80000003, P9, rcx, 0); return rax; }
            10 => { crate::lift_ops::trap(0x80000003, P10, rcx, 0); return rax; }
            11 => { crate::lift_ops::trap(0x80000003, P11, rcx, 0); return rax; }
            12 => { crate::lift_ops::trap(0x80000003, P12, rcx, 0); return rax; }
            13 => { crate::lift_ops::trap(0x80000003, P13, rcx, 0); return rax; }
            14 => { crate::lift_ops::trap(0x80000003, P14, rcx, 0); return rax; }
            15 => { crate::lift_ops::trap(0x80000003, P15, rcx, 0); return rax; }
            16 => { crate::lift_ops::trap(0x80000003, P16, rcx, 0); return rax; }
            17 => { crate::lift_ops::trap(0x80000003, P17, rcx, 0); return rax; }
            18 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P18)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a31f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a8640; const P1: u64 = 0x700019ea00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (r15) as u64); rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rdi) as u64); rbx = (r8) as u64; r13 = (rdx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (r12) as u64); r14 = (rcx) as u64; rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r15 = ((rbp.wrapping_add(0x8u64)) as u32) as u32 as u64; bb = 2; continue; }
            2 => { rcx = (((crate::simd_ops::ru8(r14)) as i8 as i32) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 25; continue; } bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; rdi = ((rcx.wrapping_sub(0x41u64)) as u32) as u32 as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = ((rcx.wrapping_sub(0x61u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdi = ((rcx.wrapping_sub(0x47u64)) as u32) as u32 as u64; bb = 8; continue; }
            6 => { rax = ((rcx.wrapping_sub(0x30u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdi = ((rcx.wrapping_add(0x4u64)) as u32) as u32 as u64; bb = 8; continue; }
            8 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 9; continue; }
            9 => { bb = 25; continue; }
            10 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2bu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdi = (rdi & !0xffu64) | (((0x3eu8) as u8 as u64)); bb = 14; continue; }
            12 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2fu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 13; continue; }
            13 => { rdi = (rdi & !0xffu64) | (((0x3fu8) as u8 as u64)); bb = 14; continue; }
            14 => { rdx = ((rbp.wrapping_add(0x6u64)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x8u32) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (r15 as u32) } else { (rax as u32) }) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rbp as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x6u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 15; continue; }
            15 => { rcx = ((r15 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rbp as u32))) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_add(0xfau8)) as u8 as u64)); rdi = (rdi & !0xffu64) | (((crate::lift_ops::shl8((rdi as u8), ((rcx as u32) & 31))) as u8 as u64)); rsi = (rsi & !0xffu64) | ((((((rsi as u8)) | ((rdi as u8)))) as u8 as u64)); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x8u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 16; continue; }
            16 => { rbp = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rsi as u8)) as u8); rdi = ((rbx.wrapping_add(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rax = ((rax).wrapping_add(rbp)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = (0x1u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rbx)) as u64; rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = (rdx) as u64; rax = ((rax).wrapping_sub(rbp)) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(rax).wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(rdx), ((rax as u8)) as u8); crate::simd_ops::lk_add_u32(rdi, (1) as u32); rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 25; continue; }
            19 => { rdx = (0x1u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rax, ((rsi as u8)) as u8); rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::lk_add_u32(rdi, (1) as u32); rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 25; continue; }
            20 => { r12 = (crate::simd_ops::ru64(rbx)) as u64; rbp = (0x6u32) as u32 as u64; rbp = (((rbp as u32)).wrapping_sub((rax as u32))) as u32 as u64; r15 = (((rdi as u8)) as u32) as u32 as u64; rcx = (((rbp as u8)) as u32) as u32 as u64; rax = ((rsp.wrapping_add(0x21u64)) as u64) as u64; r15 = (r15 & !0xffu64) | (((crate::lift_ops::sar8((r15 as u8), ((rcx as u32) & 31))) as u8 as u64)); r15 = (r15 & !0xffu64) | ((((((r15 as u8)) | ((rsi as u8)))) as u8 as u64)); rsi = ((rbx.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x21u64), ((r15 as u8)) as u8); fl_l = (rax) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = ((rsp.wrapping_add(0x21u64)) as u64) as u64; rax = ((rax).wrapping_add(r12)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = (0x1u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = (rdx) as u64; fl_l = (rax) as u64 as u64; fl_r = (r12) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(rax).wrapping_add(0x21u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(rdx), ((rax as u8)) as u8); bb = 24; continue; }
            23 => { rdx = (0x1u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rax, ((r15 as u8)) as u8); bb = 24; continue; }
            24 => { crate::simd_ops::lk_add_u32(rsi, (1) as u32); r15 = (0x8u32) as u32 as u64; rcx = ((r15 as u32)) as u32 as u64; rsi = (((rdi as u8)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rbp as u32))) as u32 as u64; fl_l = (crate::lift_ops::shl8((rsi as u8), ((rcx as u32) & 31))) as u8 as u64; rsi = (rsi & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 25; continue; }
            25 => { r14 = ((r14).wrapping_add(1)) as u64; fl_l = (r14) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 26; continue; }
            26 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 27; continue; }
            27 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a35b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x700062dee0; const P1: u64 = 0x70005a8a60; const P2: u64 = 0x70005a8a58; const P3: u64 = 0x70005a8a50; const P4: u64 = 0x70005a8a48; const P5: u64 = 0x70005a8a40; const P6: u64 = 0x70005a8a38; const P7: u64 = 0x70005a8a30; const P8: u64 = 0x70005a8a28; const P9: u64 = 0x70005a8a20; const P10: u64 = 0x70005a8a18; const P11: u64 = 0x70005a8a10; const P12: u64 = 0x70005a8a08; const P13: u64 = 0x70005a8a00; const P14: u64 = 0x70005a89f8; const P15: u64 = 0x70005a89f0; const P16: u64 = 0x70005a89e8; const P17: u64 = 0x70005a89e0; const P18: u64 = 0x70005a89d8; const P19: u64 = 0x70005a89d0; const P20: u64 = 0x70005a89c8; const P21: u64 = 0x70005a89c0; const P22: u64 = 0x70005a89b8; const P23: u64 = 0x70005a89b0; const P24: u64 = 0x70005a8b78; const P25: u64 = 0x70005a8b70; const P26: u64 = 0x70005a8b68; const P27: u64 = 0x70005a8700; const P28: u64 = 0x70005a8708; const P29: u64 = 0x70005a8e78;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); return rax; }
            1 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            3 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            10 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            11 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            12 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            13 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            14 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P14)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            15 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P15)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            16 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P16)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            17 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P17)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            18 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P18)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            19 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P19)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            20 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P20)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P21)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            22 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P22)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            23 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P23)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            24 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P24)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            25 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P25)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            26 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P26)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            27 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P27)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            28 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P28)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            29 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P29)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x700068f014; const P1: u64 = 0x70005a96e0; const P2: u64 = 0x70005a9700; const P3: u64 = 0x700062e9f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); r11 = (rdx) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdi) as u64); rbx = (((r8 as u8)) as u32) as u32 as u64; rsi = (0x8000000000000000u64) as u64; r9 = (rcx) as u64; r10 = (rsi) as u64; rdi = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { xmm2 = (crate::simd_ops::ru128(rcx)) as u128; rdx = (((rdx) & (0xfffffffffffffff0u64))) as u64; rax = ((rbx as u32)) as u32 as u64; r8 = ((P1) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); xmm6 = (crate::simd_ops::ru128(P2)) as u128; rax = ((rax).wrapping_add(rax)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; xmm3 = 0u128; rcx = ((rcx).wrapping_add(0x10u64)) as u64; xmm4 = (xmm6) as u128; xmm5 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; xmm2 = crate::simd_ops::vpsubq(xmm2, xmm5); fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { xmm1 = (crate::simd_ops::ru128(rcx)) as u128; rcx = ((rcx).wrapping_add(0x10u64)) as u64; xmm1 = crate::simd_ops::vpsubq(xmm1, xmm5); xmm0 = (xmm1) as u128; xmm0 = crate::simd_ops::vpcmpgtq(xmm0, xmm2); xmm3 = crate::simd_ops::vpblendvb(xmm3, xmm4, xmm0); xmm4 = crate::simd_ops::vpaddq(xmm4, xmm6); xmm2 = crate::simd_ops::vpblendvb(xmm2, xmm1, xmm0); fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; rdx = ((xmm2 as u64)) as u64; xmm0 = (xmm2) as u128; xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); rax = ((xmm0 as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; rdx = (if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { rax } else { rdx }) as u64; xmm0 = ((rdx) as u64) as u128; xmm0 = crate::simd_ops::vunpcklpd(xmm0, xmm0); rax = ((xmm0 as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm1 = (crate::simd_ops::ru128(P3)) as u128; r10 = (rax) as u64; xmm0 = crate::simd_ops::vpcmpeqq(xmm0, xmm2); r8 = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; xmm1 = crate::simd_ops::vpblendvb(xmm1, xmm3, xmm0); rdx = ((xmm1 as u64)) as u64; xmm0 = (xmm1) as u128; xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); rax = ((xmm0 as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; crate::simd_ops::wu128(rsp, (xmm3) as u128); rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; xmm0 = ((rdx) as u64) as u128; xmm0 = crate::simd_ops::vunpcklpd(xmm0, xmm0); xmm0 = crate::simd_ops::vpcmpeqq(xmm0, xmm1); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; rax = ((((rax as u32)) & ((r8 as u32)))) as u32 as u64; let bit_in: u32 = (rax as u32); if bit_in != 0 { rdx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = ((rdx as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(rax.wrapping_mul(8)))) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; fl_l = ((r9).wrapping_add(rdi)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (rcx) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rax) as u64 as u64; r9 = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { rcx } else { r9 }) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (r10) as u64 as u64; fl_r = (rax) as u64 as u64; rax = (if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { r10 } else { rax }) as u64; r10 = (rax) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rax = (r9) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rax = ((r10.wrapping_add(rsi)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; r9 = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { r9 }) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; rdx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rax } else { rdx }) as u64; rax = (rdx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rax = (r9) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x700068f014;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdi) as u64); r11 = (crate::simd_ops::ru32(P0)) as u32 as u64; r10 = (rdx) as u64; r10 = ((r10).wrapping_sub(rcx)) as u64; rdi = ((((r8 as u8)) as i8 as i32) as u32) as u32 as u64; rbx = (rdx) as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (r10) as u64; fl_l = (((rdx) & (0xffffffffffffffe0u64))) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = ((r11 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(5u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { xmm2 = (((rdi as u32)) as u32) as u128; ymm2_hi = 0; xmm0 = 0u128; xmm2 = crate::simd_ops::vpshufb(xmm2, xmm0); ymm2_hi = 0; let ins_v: u128 = xmm2; xmm2 = xmm2; ymm2_hi = ins_v; r8 = ((rdx.wrapping_add(rcx)) as u64) as u64; bb = 3; continue; }
            3 => { let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpcmpeqb(xmm2, core::ptr::read_unaligned((rcx) as *const u128)), crate::simd_ops::vpcmpeqb(ymm2_hi, core::ptr::read_unaligned((rcx).wrapping_add(16) as *const u128))); xmm1 = y_lo; ymm1_hi = y_hi; rax = ((crate::simd_ops::vpmovmskb(xmm1) | (crate::simd_ops::vpmovmskb(ymm1_hi) << 16)) as u32) as u32 as u64; let bit_in: u32 = (rax as u32); rax = ((bit_in.count_ones() as u32)) as u32 as u64; fl_l = (rax as u32) as u64; rax = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x20u64)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = ((((r10 as u32)) & (0x1fu32))) as u32 as u64; fl_r = fl_l; r10 = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            5 => { fl_l = (((r10) & (0xfffffffffffffff0u64))) as u64 as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { r11 = (((r11 as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = ((r11 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { xmm1 = (((rdi as u32)) as u32) as u128; ymm1_hi = 0; r8 = ((r10.wrapping_add(rcx)) as u64) as u64; xmm1 = crate::simd_ops::vpunpcklbw(xmm1, xmm1); ymm1_hi = 0; xmm1 = crate::simd_ops::vpunpcklwd(xmm1, xmm1); ymm1_hi = 0; xmm1 = crate::simd_ops::vpshufd(xmm1, 0x0u8); ymm1_hi = 0; bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rcx = ((rcx).wrapping_add(0x10u64)) as u64; xmm0 = crate::simd_ops::vpcmpeqb(xmm0, xmm1); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; let bit_in: u32 = (rax as u32); rax = ((bit_in.count_ones() as u32)) as u32 as u64; fl_l = (rax as u32) as u64; rax = ((rax as u32)) as u32 as u64; r9 = ((r9).wrapping_add(rax)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { fl_l = (rcx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; rdx = ((r9.wrapping_add(0x1u64)) as u64) as u64; rdx = (if (((fl_l as u8)) != ((fl_r as u8))) { r9 } else { rdx }) as u64; rcx = ((rcx).wrapping_add(1)) as u64; r9 = (rdx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (r9) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x700068f014;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (rdx) as u64; r9 = ((r9).wrapping_sub(rcx)) as u64; r10 = (r9) as u64; fl_l = (((r10) & (0xffffffffffffffe0u64))) as u64 as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(5u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = ((((r8 as u8)) as i8 as i32) as u32) as u32 as u64; fl_l = ((r10).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; xmm2 = (((rax as u32)) as u32) as u128; ymm2_hi = 0; xmm0 = 0u128; xmm2 = crate::simd_ops::vpshufb(xmm2, xmm0); ymm2_hi = 0; let ins_v: u128 = xmm2; xmm2 = xmm2; ymm2_hi = ins_v; bb = 3; continue; }
            3 => { let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpcmpeqb(xmm2, core::ptr::read_unaligned((rcx) as *const u128)), crate::simd_ops::vpcmpeqb(ymm2_hi, core::ptr::read_unaligned((rcx).wrapping_add(16) as *const u128))); xmm1 = y_lo; ymm1_hi = y_hi; rax = ((crate::simd_ops::vpmovmskb(xmm1) | (crate::simd_ops::vpmovmskb(ymm1_hi) << 16)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = ((((r9 as u32)) & (0x1fu32))) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = (((r9) & (0xfffffffffffffff0u64))) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = ((((r8 as u8)) as i8 as i32) as u32) as u32 as u64; r10 = ((r9.wrapping_add(rcx)) as u64) as u64; xmm1 = (((rax as u32)) as u32) as u128; ymm1_hi = 0; xmm1 = crate::simd_ops::vpunpcklbw(xmm1, xmm1); ymm1_hi = 0; xmm1 = crate::simd_ops::vpunpcklwd(xmm1, xmm1); ymm1_hi = 0; xmm1 = crate::simd_ops::vpshufd(xmm1, 0x0u8); ymm1_hi = 0; bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; xmm0 = crate::simd_ops::vpcmpeqb(xmm0, xmm1); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 10; continue; }
            10 => { fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { rax = (rcx) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { let bit_in: u32 = (rax as u32); rax = ((bit_in.trailing_zeros() as u32)) as u32 as u64; fl_l = (rax as u32) as u64; rax = ((rax as u32)) as u32 as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { rax = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x700068f014;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (rdx) as u64; r9 = ((r9).wrapping_sub(rcx)) as u64; r10 = (r9) as u64; fl_l = (((r10) & (0xffffffffffffffe0u64))) as u64 as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(5u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { xmm2 = (((r8 as u32)) as u32) as u128; ymm2_hi = 0; xmm2 = crate::simd_ops::vpshufd(xmm2, 0x0u8); ymm2_hi = 0; let ins_v: u128 = xmm2; xmm2 = xmm2; ymm2_hi = ins_v; fl_l = ((r10).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpcmpeqd(xmm2, core::ptr::read_unaligned((rcx) as *const u128)), crate::simd_ops::vpcmpeqd(ymm2_hi, core::ptr::read_unaligned((rcx).wrapping_add(16) as *const u128))); xmm1 = y_lo; ymm1_hi = y_hi; rax = ((crate::simd_ops::vpmovmskb(xmm1) | (crate::simd_ops::vpmovmskb(ymm1_hi) << 16)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = ((((r9 as u32)) & (0x1fu32))) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = (((r9) & (0xfffffffffffffff0u64))) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { xmm1 = (((r8 as u32)) as u32) as u128; ymm1_hi = 0; r10 = ((r9.wrapping_add(rcx)) as u64) as u64; xmm1 = crate::simd_ops::vpshufd(xmm1, 0x0u8); ymm1_hi = 0; bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; xmm0 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 10; continue; }
            10 => { fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { rax = (rcx) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { let bit_in: u32 = (rax as u32); rax = ((bit_in.trailing_zeros() as u32)) as u32 as u64; fl_l = (rax as u32) as u64; rax = ((rax as u32)) as u32 as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { rax = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x700068f014; const P1: u64 = 0x70005a3b86; const P2: u64 = 0x70005a3b87; const P3: u64 = 0x70005a3b88; const P4: u64 = 0x70005a3b89; const P5: u64 = 0x70005a3b8a; const P6: u64 = 0x70005a3b8b; const P7: u64 = 0x70005a3b8c; const P8: u64 = 0x70005a3b8d; const P9: u64 = 0x70005a3b8e; const P10: u64 = 0x70005a3b8f; const P11: u64 = 0x70005a3670;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (crate::simd_ops::ru32(P0)) as u32 as u64; r9 = (rdx) as u64; r9 = ((r9).wrapping_sub(rcx)) as u64; r10 = (rdx) as u64; rdx = (r9) as u64; fl_l = (((rdx) & (0xffffffffffffffe0u64))) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = ((r11 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(5u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { xmm2 = ((r8) as u64) as u128; ymm2_hi = 0; xmm2 = crate::simd_ops::vunpcklpd(xmm2, xmm2); ymm2_hi = 0; let ins_v: u128 = xmm2; xmm2 = xmm2; ymm2_hi = ins_v; fl_l = ((rdx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpcmpeqq(xmm2, core::ptr::read_unaligned((rcx) as *const u128)), crate::simd_ops::vpcmpeqq(ymm2_hi, core::ptr::read_unaligned((rcx).wrapping_add(16) as *const u128))); xmm1 = y_lo; ymm1_hi = y_hi; rax = ((crate::simd_ops::vpmovmskb(xmm1) | (crate::simd_ops::vpmovmskb(ymm1_hi) << 16)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = ((((r9 as u32)) & (0x1fu32))) as u32 as u64; fl_r = fl_l; r9 = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = (((r9) & (0xfffffffffffffff0u64))) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { r11 = (((r11 as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = ((r11 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { xmm1 = ((r8) as u64) as u128; ymm1_hi = 0; rdx = ((r9.wrapping_add(rcx)) as u64) as u64; xmm1 = crate::simd_ops::vunpcklpd(xmm1, xmm1); ymm1_hi = 0; bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; xmm0 = crate::simd_ops::vpcmpeqq(xmm0, xmm1); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 17; continue; } bb = 10; continue; }
            10 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 15; continue; }
            15 => { rax = (rcx) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { let bit_in: u32 = (rax as u32); rax = ((bit_in.trailing_zeros() as u32)) as u32 as u64; fl_l = (rax as u32) as u64; rax = ((rax as u32)) as u32 as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { rax = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { crate::lift_ops::trap(0x80000003, P1, rcx, 0); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { crate::lift_ops::trap(0x80000003, P2, rcx, 0); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { crate::lift_ops::trap(0x80000003, P3, rcx, 0); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            21 => { crate::lift_ops::trap(0x80000003, P4, rcx, 0); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { crate::lift_ops::trap(0x80000003, P5, rcx, 0); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { crate::lift_ops::trap(0x80000003, P6, rcx, 0); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { crate::lift_ops::trap(0x80000003, P7, rcx, 0); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { crate::lift_ops::trap(0x80000003, P8, rcx, 0); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { crate::lift_ops::trap(0x80000003, P9, rcx, 0); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { crate::lift_ops::trap(0x80000003, P10, rcx, 0); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            28 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x700068f014; const P1: u64 = 0x70005a9720; const P2: u64 = 0xffffffffffffffe0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0; let mut ymm3_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (rdx) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x40u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(5u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { let y_addr: u64 = P1; xmm3 = core::ptr::read_unaligned(y_addr as *const u128); ymm3_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = (0x7fffffffffffffe0u64) as u64; rdx = (((rdx) & (rax))) as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { let (p_lo, p_hi) = crate::simd_ops::vpermd(xmm3, ymm3_hi, core::ptr::read_unaligned((rcx) as *const u128), core::ptr::read_unaligned((rcx).wrapping_add(16) as *const u128)); xmm2 = p_lo; ymm2_hi = p_hi; let (p_lo, p_hi) = crate::simd_ops::vpermd(xmm3, ymm3_hi, core::ptr::read_unaligned((r8.wrapping_add(P2)) as *const u128), core::ptr::read_unaligned((r8.wrapping_add(P2)).wrapping_add(16) as *const u128)); xmm1 = p_lo; ymm1_hi = p_hi; r8 = ((r8).wrapping_sub(0x20u64)) as u64; let y_addr: u64 = rcx; core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); rcx = ((rcx).wrapping_add(0x20u64)) as u64; let y_addr: u64 = r8; core::ptr::write_unaligned(y_addr as *mut u128, xmm2); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm2_hi); fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = (r8) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x20u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_shr(1u32)) as u64; rdx = (0x7ffffffffffffff0u64) as u64; rax = (((rax) & (rdx))) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; r8 = ((r8).wrapping_sub(0x10u64)) as u64; xmm2 = crate::simd_ops::vpshufd(xmm0, 0x1bu8); ymm2_hi = 0; xmm1 = (crate::simd_ops::ru128(r8)) as u128; ymm1_hi = 0; xmm0 = crate::simd_ops::vpshufd(xmm1, 0x1bu8); crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu128(r8, (xmm2) as u128); fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { r8 = ((r8).wrapping_sub(0x4u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(r8, ((rdx as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x700068f014; const P1: u64 = 0xffffffffffffffe0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (rdx) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x40u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(5u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = (0x7fffffffffffffe0u64) as u64; rdx = (((rdx) & (rax))) as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { let (p_lo, p_hi) = crate::simd_ops::vpermpd(core::ptr::read_unaligned((rcx) as *const u128), core::ptr::read_unaligned((rcx).wrapping_add(16) as *const u128), 0x1bu8); xmm2 = p_lo; ymm2_hi = p_hi; let (p_lo, p_hi) = crate::simd_ops::vpermpd(core::ptr::read_unaligned((r8.wrapping_add(P1)) as *const u128), core::ptr::read_unaligned((r8.wrapping_add(P1)).wrapping_add(16) as *const u128), 0x1bu8); xmm1 = p_lo; ymm1_hi = p_hi; r8 = ((r8).wrapping_sub(0x20u64)) as u64; let y_addr: u64 = rcx; core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); rcx = ((rcx).wrapping_add(0x20u64)) as u64; let y_addr: u64 = r8; core::ptr::write_unaligned(y_addr as *mut u128, xmm2); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm2_hi); fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = (r8) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x20u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_shr(1u32)) as u64; rdx = (0x7ffffffffffffff0u64) as u64; rax = (((rax) & (rdx))) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; r8 = ((r8).wrapping_sub(0x10u64)) as u64; xmm2 = crate::simd_ops::vpshufd(xmm0, 0x4eu8); ymm2_hi = 0; xmm1 = (crate::simd_ops::ru128(r8)) as u128; ymm1_hi = 0; xmm0 = crate::simd_ops::vpshufd(xmm1, 0x4eu8); crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu128(r8, (xmm2) as u128); fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { r8 = ((r8).wrapping_sub(0x8u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(r8, (rdx) as u64); fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a47e4; const P1: u64 = 0x8; const P2: u64 = 0x7000694918;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((crate::exe::teb_self() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; bb = 3; continue; }
            2 => { fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rax) as u64 as u64; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(P2, fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3dcc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a47e4; const P1: u64 = 0x70005a4630; const P2: u64 = 0x70005a47dc; const P3: u64 = 0x70005a4c7a; const P4: u64 = 0x70005a4c80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a3f38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3e18(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70001a4fe0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 1; continue; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70001a4fe0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3e58(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a47e4; const P1: u64 = 0x70005a8f10; const P2: u64 = 0x70005a4c74;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (r9) as u64;
    rsi = (r8) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rbp = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { r8 = (rsi) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbp) as u64; rax = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3eb8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a47e4; const P1: u64 = 0x7000694928; const P2: u64 = 0x70005a4c92; const P3: u64 = 0x70001f4310; const P4: u64 = 0x70005a4c9e;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rcx = ((P1) as u64) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3ee8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70001a4fe0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3efc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000694920; const P1: u64 = 0x70005a4630; const P2: u64 = 0x70001a4fe0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(P0, (0x1u8) as u8); bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 7; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3f38(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000694921; const P1: u64 = 0x70005a47e4; const P2: u64 = 0x7000694928; const P3: u64 = 0x70005a4c86; const P4: u64 = 0x7000694940; const P5: u64 = 0x700062e9f0; const P6: u64 = 0x7000694938; const P7: u64 = 0x7000694950; const P8: u64 = 0x70005a47f8; const P9: u64 = 0x70005a3fc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 11; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = ((P2) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 10; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(P5)) as u128; fl_l = (((rax) | (0xffffffffffffffffu64))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu128(P2, (xmm0) as u128); crate::simd_ops::wu64(P6, (rax) as u64); crate::simd_ops::wu128(P4, (xmm0) as u128); crate::simd_ops::wu64(P7, (rax) as u64); bb = 8; continue; }
            8 => { crate::simd_ops::wu8(P0, (0x1u8) as u8); bb = 9; continue; }
            9 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rcx = (0x5u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a3fc4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000000000; const P1: u64 = 0x700000003c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[0], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x18u64)) as u64; r8 = (rcx) as u64; rax = (0x5a4du32) as u32 as u64; fl_l = (crate::simd_ops::ru16(P0)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 14; continue; } bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru32(P1)) as i32 as i64) as u64) as u64; rdx = ((P0) as u64) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = ((match eh.ru32(0x70005a3fea, rcx) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a4055 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x70005a3fc4) } })) as u32 as u64; fl_r = (0x4550u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rax = (0x20bu32) as u32 as u64; fl_l = ((match eh.ru16(0x70005a3ff7, rcx.wrapping_add(0x18u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a4055 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x70005a3fc4) } })) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 14; continue; } bb = 3; continue; }
            3 => { r8 = ((r8).wrapping_sub(rdx)) as u64; rdx = (((match eh.ru16(0x70005a4000, rcx.wrapping_add(0x14u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a4055 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x70005a3fc4) } })) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rax = (((match eh.ru16(0x70005a400b, rcx.wrapping_add(0x6u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a4055 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x70005a3fc4) } })) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r9 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp, (rdx) as u64); fl_l = (rdx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = ((match eh.ru32(0x70005a4020, rdx.wrapping_add(0xcu64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a4055 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x70005a3fc4) } })) as u32 as u64; fl_l = (r8) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((match eh.ru32(0x70005a4028, rdx.wrapping_add(0x8u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a4055 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x70005a3fc4) } })) as u32 as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx).wrapping_add(0x28u64)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 4; continue; }
            8 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 16; continue; }
            11 => { fl_l = ((match eh.ru32(0x70005a4043, rdx.wrapping_add(0x24u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a4055 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x70005a3fc4) } })) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 16; continue; }
            13 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 16; continue; }
            14 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 16; continue; }
            15 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { rsp = ((rsp).wrapping_add(0x18u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a405c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a47e4; const P1: u64 = 0x7000694918;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rbx & !0xffu64) | ((((rcx as u8)) as u8 as u64));
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(P1, (rdx) as u64); rdx = (tmp_xchg) as u64; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000694920; const P1: u64 = 0x70001a4fe0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (rbx & !0xffu64) | ((((rcx as u8)) as u8 as u64)); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rcx & !0xffu64) | ((((rbx as u8)) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a40ac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000694928; const P1: u64 = 0x70005a4c98; const P2: u64 = 0x70005a4c8c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru64(P0)) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64;
    rbx = (rcx) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); } else { rdx = (rbx) as u64; rcx = ((P0) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rbx } else { rdx }) as u64;
    rax = (rdx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a40e8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a40ac;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rax = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4108(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000694960; const P1: u64 = 0x70005a8000; const P2: u64 = 0x700068f004; const P3: u64 = 0x7000694974; const P4: u64 = 0x70005a8098; const P5: u64 = 0x7000694958; const P6: u64 = 0x70005a8088;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    rcx = ((P0) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (crate::simd_ops::ru32(P2)) as u32 as u64;
    rcx = ((P0) as u64) as u64;
    rdx = (crate::simd_ops::ru32(P3)) as u32 as u64;
    rax = (((rax as u32)).wrapping_add(1)) as u32 as u64;
    crate::simd_ops::wu32(P2, ((rax as u32)) as u32);
    crate::simd_ops::wu32(rbx, ((rax as u32)) as u32);
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    r9 = (0x8u32) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64;
    rax = (crate::simd_ops::ru32(P2)) as u32 as u64;
    crate::simd_ops::wu32(r9.wrapping_add(r8), ((rax as u32)) as u32);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((P5) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4174(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000694960; const P1: u64 = 0x70005a8000; const P2: u64 = 0x7000694958; const P3: u64 = 0x70005a8080; const P4: u64 = 0x7000694974; const P5: u64 = 0x700068f004; const P6: u64 = 0x70005a8098;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; rcx = ((P0) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_or_u32(rbx, (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; bb = 6; continue; }
            3 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((P0) as u64) as u64; r8 = ((((r8 as u32)) | (0xffffffffu32))) as u32 as u64; rcx = ((P2) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 1; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru32(P4)) as u32 as u64; r8 = (0x8u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(rdx), ((rax as u32)) as u32); bb = 6; continue; }
            6 => { rcx = ((P0) as u64) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a41ec(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000694974; const P1: u64 = 0x70005a9678; const P2: u64 = 0x70005a8f10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[1], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = (0xcu32) as u32 as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rdx.wrapping_add(r8), (0x1u8) as u8); rbx = ((P1) as u64) as u64; rdi = ((P1) as u64) as u64; bb = 6; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4238); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { fl_l = ((rbx).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4254(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x70005a41ec;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    rdx = ((r8.wrapping_add(P0)) as u32) as u32 as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4298(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a3e40; const P1: u64 = 0x70005a3e18; const P2: u64 = 0x70005a42e8; const P3: u64 = 0x70005a4400;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (0x1u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            7 => { rax = (((rax as u8)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            8 => { rdx = (r8) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            9 => { fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a42e8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a3efc; const P1: u64 = 0x70005a3d90; const P2: u64 = 0x7000694910; const P3: u64 = 0x70005a3e00; const P4: u64 = 0x70005a4a58; const P5: u64 = 0x70005a4a10; const P6: u64 = 0x70005a4a34; const P7: u64 = 0x70005a9688; const P8: u64 = 0x70005a9680; const P9: u64 = 0x70005a4caa; const P10: u64 = 0x70005a3dcc; const P11: u64 = 0x70005a9668; const P12: u64 = 0x70005a8f38; const P13: u64 = 0x70005a4ca4; const P14: u64 = 0x70005a405c; const P15: u64 = 0x70005a4a50; const P16: u64 = 0x70005a3fc4; const P17: u64 = 0x70005a8f10; const P18: u64 = 0x7000694968; const P19: u64 = 0x70005a47f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[2], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rdx) as u64; r14 = (rcx) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4305); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4305, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4312); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4312, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; rbx = (rbx & !0xffu64) | ((((rax as u8)) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rax as u8)) as u8); rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(P2, (0x1u32) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4337); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4337, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4340); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4340, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4345); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4345, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a434a); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a434a, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; rdx = ((P7) as u64) as u64; rcx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a435d); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a435d, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4366); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4366, (P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((P11) as u64) as u64; rcx = ((P12) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a437d); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a437d, (P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; crate::simd_ops::wu32(P2, (0x2u32) as u32); rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = (rcx & !0xffu64) | ((((rbx as u8)) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4391); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4391, (P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a439b); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a439b, (P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; rbx = (rax) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a43ac); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a43ac, (P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = (rsi) as u64; rdx = (0x2u32) as u32 as u64; rcx = (r14) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = (crate::simd_ops::ru64(P17)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a43ca); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a43ca, (r9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(P18, (1) as u32)) as u32 as u64; rax = (0x1u32) as u32 as u64; bb = 12; continue; }
            11 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rcx = (0x7u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a43f7); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a43f7, (P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a42e8) }; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000694968; const P1: u64 = 0x70005a3d90; const P2: u64 = 0x7000694910; const P3: u64 = 0x70005a3eb8; const P4: u64 = 0x70005a4a20; const P5: u64 = 0x70005a4a94; const P6: u64 = 0x70005a405c; const P7: u64 = 0x70005a4080; const P8: u64 = 0x70005a3ee8; const P9: u64 = 0x70005a47f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[3], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (rdi & !0xffu64) | ((((rcx as u8)) as u8 as u64)); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(P0, ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a442c); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a442c, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a4400) }; rbx = (rbx & !0xffu64) | ((((rax as u8)) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4440); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4440, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a4400) }; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4445); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4445, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a4400) }; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a444a); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a444a, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a4400) }; crate::simd_ops::lk_and_u32(P2, (0x0u32) as u32); rcx = (rcx & !0xffu64) | ((((rbx as u8)) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4458); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4458, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a4400) }; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rcx & !0xffu64) | ((((rdi as u8)) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4462); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4462, (P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a4400) }; rbx = (((rax as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a446a); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a446a, (P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a4400) }; rax = ((rbx as u32)) as u32 as u64; bb = 2; continue; }
            5 => { rcx = (0x7u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4478); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4478, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x70005a4400) }; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000694968; const P1: u64 = 0x70005a9760; const P2: u64 = 0x70005a8f10; const P3: u64 = 0x70005a4298; const P4: u64 = 0x70005a49ec; const P5: u64 = 0x70005a4400;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[4], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r8) as u64; rdi = ((rdx as u32)) as u32 as u64; r14 = (rcx) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 21; continue; }
            3 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(P1)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); bb = 7; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a44d3); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a44d3, ({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a459b => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x70005a4480) } }; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { r8 = (rsi) as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a44ef); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a44ef, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a459b => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x70005a4480) } }; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 8; continue; }
            8 => { r8 = (rsi) as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a450a); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a450a, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a459b => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x70005a4480) } }; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = (rsi) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4526); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4526, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a459b => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x70005a4480) } }; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4531); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4531, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a459b => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x70005a4480) } }; rax = (crate::simd_ops::ru64(P1)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = (rsi) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a454a); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a454a, ({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a459b => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x70005a4480) } }; bb = 12; continue; }
            12 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 14; continue; }
            14 => { r8 = (rsi) as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a4561); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a4561, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a459b => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x70005a4480) } }; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(P1)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rbx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); bb = 18; continue; }
            17 => { r8 = (rsi) as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x70005a458d); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x70005a458d, ({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x70005a459b => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x70005a4480) } }; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); bb = 18; continue; }
            18 => { bb = 20; continue; }
            19 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); bb = 20; continue; }
            20 => { rax = ((rbx as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a45b4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a4940; const P1: u64 = 0x70005a4480;
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
pub unsafe extern "win64" fn fn_70005a4610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x700068f040; const P1: u64 = 0x70005a4b04;
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
pub unsafe extern "win64" fn fn_70005a4630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x700068f020; const P1: u64 = 0x700068f018; const P2: u64 = 0x700069496c; const P3: u64 = 0x700068f010; const P4: u64 = 0x700068f014;
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
pub unsafe extern "win64" fn fn_70005a47dc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (0x1u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a47e4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x700068f090; const P1: u64 = 0x7000694970;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            1 => { fl_l = (crate::simd_ops::lk_and_u32(P1, (0x0u32) as u32)) as u32 as u64; fl_r = fl_l; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x700068f040; const P1: u64 = 0x70005a8030; const P2: u64 = 0x70005a8038; const P3: u64 = 0x70005a8040; const P4: u64 = 0x70005a80a0; const P5: u64 = 0x700068f080;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rbx = (0x2b992ddfa232u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbp.wrapping_add(0x10u64), (0x0u64) as u64); rcx = ((rbp.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rax as u32)) as u32 as u64; crate::simd_ops::lk_xor_u64(rbp.wrapping_sub(0x10u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rax as u32)) as u32 as u64; rcx = ((rbp.wrapping_add(0x18u64)) as u64) as u64; crate::simd_ops::lk_xor_u64(rbp.wrapping_sub(0x10u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rax = ((rax).wrapping_shl(32u32)) as u64; rax = (((rax) ^ (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))))) as u64; rax = (((rax) ^ (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))))) as u64; rax = (((rax) ^ (rcx))) as u64; rcx = (0xffffffffffffu64) as u64; rax = (((rax) & (rcx))) as u64; rcx = (0x2b992ddfa233u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::wu64(P0, (rax) as u64); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (!(rax)) as u64; crate::simd_ops::wu64(P5, (rax) as u64); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a49ec(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a9760; const P1: u64 = 0x70005a8028;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = (0x1u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    fl_l = (crate::simd_ops::ru64(P0)) as u64 as u64; fl_r = (0x0u64) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (0x1u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4a34(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000010230; const P1: u64 = 0x70005a4a2c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::lk_or_u64(rax, (0x24u64) as u64);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::lk_or_u64(rax, (0x2u64) as u64);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a8050; const P1: u64 = 0x70005a8058; const P2: u64 = 0x70005a8018; const P3: u64 = 0x70005a8010;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (rax) as u64;
    rdx = (0xc0000409u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4b04(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a8048; const P1: u64 = 0x70005a4b21; const P2: u64 = 0x7000694a40; const P3: u64 = 0x70005a4bd8; const P4: u64 = 0x7000694b38; const P5: u64 = 0x7000694ad8; const P6: u64 = 0x70006949b0; const P7: u64 = 0x7000694ac0; const P8: u64 = 0x70006949a0; const P9: u64 = 0x70006949a4; const P10: u64 = 0x70006949b8; const P11: u64 = 0x70006949c0; const P12: u64 = 0x700068f040; const P13: u64 = 0x700068f080; const P14: u64 = 0x70005a9768; const P15: u64 = 0x70005a4ad0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rcx = (0x17u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (0x2u32) as u32 as u64; crate::lift_ops::trap(0xc0000409, P1, rcx, rsp); return rax; bb = 2; continue; }
            2 => { rcx = ((P2) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(P4, (rax) as u64); rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(P5, (rax) as u64); rax = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu64(P6, (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(P7, (rax) as u64); crate::simd_ops::wu32(P8, (0xc0000409u32) as u32); crate::simd_ops::wu32(P9, (0x1u32) as u32); crate::simd_ops::wu32(P10, (0x1u32) as u32); rax = (0x8u32) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x0u64) as i64) as u64) as u64; rcx = ((P11) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rax), (0x2u64) as u64); rax = (0x8u32) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x0u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(P12)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(rax).wrapping_add(0x20u64), (rcx) as u64); rax = (0x8u32) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x1u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(P13)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(rax).wrapping_add(0x20u64), (rcx) as u64); rcx = ((P14) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 2>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x10u64)) as u64;
    crate::simd_ops::wu64(rsp, (r10) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (r11) as u64);
    r11 = (0u64) as u64;
    fl_l = 0; fl_r = 0;
    r10 = ((rsp.wrapping_add(0x18u64)) as u64) as u64;
    fl_l = (r10) as u64 as u64; fl_r = (rax) as u64 as u64;
    r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { r11 } else { r10 }) as u64;
    r11 = (0x1000u64) as u64;
    fl_l = (r10) as u64 as u64; fl_r = (r11) as u64 as u64;
    if (((fl_l as u64)) >= ((fl_r as u64))) { r10 = (crate::simd_ops::ru64(rsp)) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; return rax; }
    fl_l = ((((r10 as u16)) & (0xf000u16))) as u16 as u64; fl_r = fl_l;
    r10 = (r10 & !0xffffu64) | ((((fl_l as u16)) as u16 as u64));
    loop { r11 = ((r11.wrapping_sub(0x1000u64)) as u64) as u64; crate::simd_ops::wu8(r11, (0x0u8) as u8); fl_l = (r10) as u64 as u64; fl_r = (r11) as u64 as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    r10 = (crate::simd_ops::ru64(rsp)) as u64;
    r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    rsp = ((rsp).wrapping_add(0x10u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
pub unsafe extern "win64" fn fn_70005a4db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a8f10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4db6(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0xc0000005u32) as u32 as u64;
    rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    rax = ((rcx as u32)) as u32 as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4dce(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a405c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rbp.wrapping_add(0x40u64))) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4de5(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a405c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rbp.wrapping_add(0x20u64))) as u8 as u64));
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4dfe(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a3ee8;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a4e12(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005a4298; const P1: u64 = 0x70005a3e58;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru32(rax)) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32);
    r9 = ((P0) as u64) as u64;
    r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64;
    rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x68u64))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a6df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x70006041a8; const P1: u64 = 0x7000692a88;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(P1, (rax) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a6e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70005ffba8; const P1: u64 = 0x7000692a90; const P2: u64 = 0x70005a6e0f; const P3: u64 = 0x7000691528; const P4: u64 = 0x7000691538; const P5: u64 = 0x7000691530; const P6: u64 = 0x7000697150; const P7: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((P0) as u64) as u64; crate::simd_ops::wu64(P1, (rax) as u64); return rax; }
            1 => { crate::lift_ops::trap(0x80000003, P2, rcx, 0); return rax; }
            2 => { fl_l = (crate::simd_ops::ru8(P3)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(P4)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(P5)) as u64 as u64; fl_r = (0x17u64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P6)) as u64; r8 = (0x8u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a6e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70006915b0; const P1: u64 = 0x70006915c0; const P2: u64 = 0x70006915b8; const P3: u64 = 0x7000697150; const P4: u64 = 0x10; const P5: u64 = 0x70006915a8; const P6: u64 = 0x70001332f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx = ((P5) as u64) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rdx = (crate::simd_ops::ru64(P1)) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx = ((P5) as u64) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    fl_l = (crate::simd_ops::ru64(P2)) as u64 as u64; fl_r = (0x17u64) as u64 as u64;
    if !((((fl_l as u64)) <= ((fl_r as u64)))) { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = (0x8u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rcx = ((P5) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a7310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000697428; const P1: u64 = 0x7000697150; const P2: u64 = 0x10; const P3: u64 = 0x7000697420;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = ((rdx).wrapping_add(0xfffffffffffffff8u64)) as u64; r8 = (0x8u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rdx = (crate::simd_ops::ru64(P3)) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = ((rdx).wrapping_add(0xfffffffffffffff8u64)) as u64; r8 = (0x8u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a7370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x700069179c; const P1: u64 = 0x7000691780; const P2: u64 = 0x70001a32d0; const P3: u64 = 0x7000697150; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = (0x1u32) as u32 as u64;
    rbx = ((P1) as u64) as u64;
    rcx = ((P1) as u64) as u64;
    rbx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(P1) } else { rbx }) as u64;
    rdx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64;
    if !((((fl_l as u32)) <= ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = (0x8u32) as u32 as u64; rdx = (rbx) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    crate::simd_ops::wu32(P0, (0x0u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a75f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x700069181c; const P1: u64 = 0x7000697150; const P2: u64 = 0x7000691810; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64;
    if !((((fl_l as u32)) <= ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = (0x8u32) as u32 as u64; rdx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(P0, (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::wu32(P0, (0x0u32) as u32); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a7940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000692748; const P1: u64 = 0x7000692758; const P2: u64 = 0x7000692750; const P3: u64 = 0x7000697150; const P4: u64 = 0x10; const P5: u64 = 0x70005a7974; const P6: u64 = 0x70005a7975; const P7: u64 = 0x70005a7976; const P8: u64 = 0x70005a7977; const P9: u64 = 0x70005a7978; const P10: u64 = 0x70005a7979; const P11: u64 = 0x70005a797a; const P12: u64 = 0x70005a797b; const P13: u64 = 0x70005a797c; const P14: u64 = 0x70005a797d; const P15: u64 = 0x70005a797e; const P16: u64 = 0x70005a797f; const P17: u64 = 0x7000692760; const P18: u64 = 0x7000692770; const P19: u64 = 0x7000692768; const P20: u64 = 0x70005a79b4; const P21: u64 = 0x70005a79b5; const P22: u64 = 0x70005a79b6; const P23: u64 = 0x70005a79b7; const P24: u64 = 0x70005a79b8; const P25: u64 = 0x70005a79b9; const P26: u64 = 0x70005a79ba; const P27: u64 = 0x70005a79bb; const P28: u64 = 0x70005a79bc; const P29: u64 = 0x70005a79bd; const P30: u64 = 0x70005a79be; const P31: u64 = 0x70005a79bf; const P32: u64 = 0x700062dda8; const P33: u64 = 0x7000692800; const P34: u64 = 0x70005a79cf; const P35: u64 = 0x7000692778;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(P1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(P2)) as u64 as u64; fl_r = (0x17u64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = (0x13u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { return rax; }
            5 => { crate::lift_ops::trap(0x80000003, P5, rcx, 0); return rax; }
            6 => { crate::lift_ops::trap(0x80000003, P6, rcx, 0); return rax; }
            7 => { crate::lift_ops::trap(0x80000003, P7, rcx, 0); return rax; }
            8 => { crate::lift_ops::trap(0x80000003, P8, rcx, 0); return rax; }
            9 => { crate::lift_ops::trap(0x80000003, P9, rcx, 0); return rax; }
            10 => { crate::lift_ops::trap(0x80000003, P10, rcx, 0); return rax; }
            11 => { crate::lift_ops::trap(0x80000003, P11, rcx, 0); return rax; }
            12 => { crate::lift_ops::trap(0x80000003, P12, rcx, 0); return rax; }
            13 => { crate::lift_ops::trap(0x80000003, P13, rcx, 0); return rax; }
            14 => { crate::lift_ops::trap(0x80000003, P14, rcx, 0); return rax; }
            15 => { crate::lift_ops::trap(0x80000003, P15, rcx, 0); return rax; }
            16 => { crate::lift_ops::trap(0x80000003, P16, rcx, 0); return rax; }
            17 => { fl_l = (crate::simd_ops::ru8(P17)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(P18)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru64(P19)) as u64 as u64; fl_r = (0x17u64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = (0x2u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            21 => { return rax; }
            22 => { crate::lift_ops::trap(0x80000003, P20, rcx, 0); return rax; }
            23 => { crate::lift_ops::trap(0x80000003, P21, rcx, 0); return rax; }
            24 => { crate::lift_ops::trap(0x80000003, P22, rcx, 0); return rax; }
            25 => { crate::lift_ops::trap(0x80000003, P23, rcx, 0); return rax; }
            26 => { crate::lift_ops::trap(0x80000003, P24, rcx, 0); return rax; }
            27 => { crate::lift_ops::trap(0x80000003, P25, rcx, 0); return rax; }
            28 => { crate::lift_ops::trap(0x80000003, P26, rcx, 0); return rax; }
            29 => { crate::lift_ops::trap(0x80000003, P27, rcx, 0); return rax; }
            30 => { crate::lift_ops::trap(0x80000003, P28, rcx, 0); return rax; }
            31 => { crate::lift_ops::trap(0x80000003, P29, rcx, 0); return rax; }
            32 => { crate::lift_ops::trap(0x80000003, P30, rcx, 0); return rax; }
            33 => { crate::lift_ops::trap(0x80000003, P31, rcx, 0); return rax; }
            34 => { rax = ((P32) as u64) as u64; crate::simd_ops::wu64(P33, (rax) as u64); return rax; }
            35 => { crate::lift_ops::trap(0x80000003, P34, rcx, 0); return rax; }
            36 => { rax = ((P32) as u64) as u64; crate::simd_ops::wu64(P35, (rax) as u64); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_70005a7bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7000692930; const P1: u64 = 0x7000692940; const P2: u64 = 0x7000692938; const P3: u64 = 0x7000697150; const P4: u64 = 0x10; const P5: u64 = 0x70005a7c04; const P6: u64 = 0x70005a7c05; const P7: u64 = 0x70005a7c06; const P8: u64 = 0x70005a7c07; const P9: u64 = 0x70005a7c08; const P10: u64 = 0x70005a7c09; const P11: u64 = 0x70005a7c0a; const P12: u64 = 0x70005a7c0b; const P13: u64 = 0x70005a7c0c; const P14: u64 = 0x70005a7c0d; const P15: u64 = 0x70005a7c0e; const P16: u64 = 0x70005a7c0f;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(P1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(P2)) as u64 as u64; fl_r = (0x17u64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = (0x13u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x70005a4d90 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { return rax; }
            5 => { crate::lift_ops::trap(0x80000003, P5, rcx, 0); return rax; }
            6 => { crate::lift_ops::trap(0x80000003, P6, rcx, 0); return rax; }
            7 => { crate::lift_ops::trap(0x80000003, P7, rcx, 0); return rax; }
            8 => { crate::lift_ops::trap(0x80000003, P8, rcx, 0); return rax; }
            9 => { crate::lift_ops::trap(0x80000003, P9, rcx, 0); return rax; }
            10 => { crate::lift_ops::trap(0x80000003, P10, rcx, 0); return rax; }
            11 => { crate::lift_ops::trap(0x80000003, P11, rcx, 0); return rax; }
            12 => { crate::lift_ops::trap(0x80000003, P12, rcx, 0); return rax; }
            13 => { crate::lift_ops::trap(0x80000003, P13, rcx, 0); return rax; }
            14 => { crate::lift_ops::trap(0x80000003, P14, rcx, 0); return rax; }
            15 => { crate::lift_ops::trap(0x80000003, P15, rcx, 0); return rax; }
            16 => { crate::lift_ops::trap(0x80000003, P16, rcx, 0); return rax; }
            17 => { return rax; }
            _ => { return rax; }
        }
    }
}
