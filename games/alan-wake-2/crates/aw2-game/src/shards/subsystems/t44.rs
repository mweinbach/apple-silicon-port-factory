#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_140204220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x14183acc8; const P2: u64 = 0x141834d30; const P3: u64 = 0x1426467f0; const P4: u64 = 0x1402032e0; const P5: u64 = 0x140204392;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140204220, funcinfo_rva: 0x35a396c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = (rdx) as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(r8)) as u64; rdi = ((rdi).wrapping_shl(3u32)) as u64; r12 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); rsi = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r13) as u64); rbx = (rdi) as u64; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; r15 = ((r13 as u32)) as u32 as u64; rbp = ((r13 as u32)) as u32 as u64; rax = (0x1fffffffffffffffu64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx.wrapping_mul(8)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020428f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rsi = ((rbx.wrapping_add(rax)) as u64) as u64; r15 = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); r8 = (rdi) as u64; rdx = (r12) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402042b1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); rax = (0x1fffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r14, (r13) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (r13) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (r13) as u64); r15 = ((r15).wrapping_sub(rbp)) as u64; rdi = (r15) as u64; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdi = ((rdi.wrapping_mul(8)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402042f3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); rcx = ((rdi.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rcx) as u64); r8 = (r15) as u64; rdx = (rbp) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204313); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); bb = 6; continue; }
            6 => { crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0x9u8) as u8); fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rsi = ((rsi).wrapping_sub(rbp)) as u64; rsi = (((rsi) & (0xfffffffffffffff8u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rsi = ((rsi).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).wrapping_sub(rcx)) as u64; rax = ((rbp.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rbp = (rcx) as u64; bb = 10; continue; }
            10 => { rdx = (rsi) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204358); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (r14) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204380); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204387); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020438d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402043a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x14183acc8; const P2: u64 = 0x141834d30; const P3: u64 = 0x1426467f0; const P4: u64 = 0x1402032e0; const P5: u64 = 0x140204512;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1402043a0, funcinfo_rva: 0x35a396c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = (rdx) as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(r8)) as u64; rdi = ((rdi).wrapping_shl(3u32)) as u64; r12 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); rsi = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r13) as u64); rbx = (rdi) as u64; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; r15 = ((r13 as u32)) as u32 as u64; rbp = ((r13 as u32)) as u32 as u64; rax = (0x1fffffffffffffffu64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx.wrapping_mul(8)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020440f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rsi = ((rbx.wrapping_add(rax)) as u64) as u64; r15 = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); r8 = (rdi) as u64; rdx = (r12) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204431); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); rax = (0x1fffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r14, (r13) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (r13) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (r13) as u64); r15 = ((r15).wrapping_sub(rbp)) as u64; rdi = (r15) as u64; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdi = ((rdi.wrapping_mul(8)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204473); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); rcx = ((rdi.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rcx) as u64); r8 = (r15) as u64; rdx = (rbp) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204493); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); bb = 6; continue; }
            6 => { crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0xdu8) as u8); fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rsi = ((rsi).wrapping_sub(rbp)) as u64; rsi = (((rsi) & (0xfffffffffffffff8u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rsi = ((rsi).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).wrapping_sub(rcx)) as u64; rax = ((rbp.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rbp = (rcx) as u64; bb = 10; continue; }
            10 => { rdx = (rsi) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402044d8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (r14) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204500); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204507); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020450d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140204520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x14183acc8; const P2: u64 = 0x141834d30; const P3: u64 = 0x1426467f0; const P4: u64 = 0x1402032e0; const P5: u64 = 0x140204692;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140204520, funcinfo_rva: 0x35a396c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = (rdx) as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(r8)) as u64; rdi = ((rdi).wrapping_shl(3u32)) as u64; r12 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); rsi = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r13) as u64); rbx = (rdi) as u64; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; r15 = ((r13 as u32)) as u32 as u64; rbp = ((r13 as u32)) as u32 as u64; rax = (0x1fffffffffffffffu64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx.wrapping_mul(8)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020458f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rsi = ((rbx.wrapping_add(rax)) as u64) as u64; r15 = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); r8 = (rdi) as u64; rdx = (r12) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402045b1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); rax = (0x1fffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r14, (r13) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (r13) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (r13) as u64); r15 = ((r15).wrapping_sub(rbp)) as u64; rdi = (r15) as u64; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdi = ((rdi.wrapping_mul(8)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402045f3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); rcx = ((rdi.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rcx) as u64); r8 = (r15) as u64; rdx = (rbp) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204613); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); bb = 6; continue; }
            6 => { crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0xau8) as u8); fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rsi = ((rsi).wrapping_sub(rbp)) as u64; rsi = (((rsi) & (0xfffffffffffffff8u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rsi = ((rsi).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).wrapping_sub(rcx)) as u64; rax = ((rbp.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rbp = (rcx) as u64; bb = 10; continue; }
            10 => { rdx = (rsi) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204658); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (r14) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204680); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204687); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020468d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402046a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x1402050d0; const P2: u64 = 0x140203210; const P3: u64 = 0x140203690; const P4: u64 = 0x1426467f0; const P5: u64 = 0x14020486e; const P6: u64 = 0x141834d30; const P7: u64 = 0x1402032e0; const P8: u64 = 0x1402048a3;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1402046a0, funcinfo_rva: 0x35a39a0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r14 = (rdx) as u64; r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(r8)) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rsi = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; r15 = ((rsi.wrapping_add(rdi)) as u64) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (r12) as u64); rdi = ((((rdi) as i64).wrapping_shr(4u32) as u64)) as u64; r13 = (0x7ffffffffffffffu64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (rdi) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 24; continue; } bb = 2; continue; }
            2 => { rdi = ((rdi).wrapping_shl(5u32)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020470d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); rax = ((rax).wrapping_add(rdi)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rax) as u64); rax = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rbx) as u64); rdi = (rbx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rbx) as u64); rax = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rax) as u64); fl_l = (rsi) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (r12) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), (r12) as u64); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020476a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rdi).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rdi) as u64); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rdi) as u64); bb = 8; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r14, (r12) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (r12) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (r12) as u64); r15 = (rdi) as u64; r15 = ((r15).wrapping_sub(rbx)) as u64; r15 = ((((r15) as i64).wrapping_shr(5u32) as u64)) as u64; fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { fl_l = (r15) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 23; continue; } bb = 10; continue; }
            10 => { r15 = ((r15).wrapping_shl(5u32)) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402047bc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); rcx = ((rax.wrapping_add(r15)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (r14) as u64); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (r14) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdx = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402047f6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = ((rsi).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rsi) as u64); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; bb = 13; continue; }
            13 => { crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rsi) as u64); bb = 14; continue; }
            14 => { crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0xbu8) as u8); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020482a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = (((rdx) & (0xffffffffffffffe0u64))) as u64; rax = (rbx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204868); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            21 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204872); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            22 => { rax = (r14) as u64; r11 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204898); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            24 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020489e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402048b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646950;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r9; rsi = r8; rdi = rdx; r15 = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = rsi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = (((crate::simd_ops::ru8(r15)) as i8 as i32) as u32) as u32 as u64; r14 = (((crate::simd_ops::ru8(rsi)) as i8 as i32) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((r14 as u32)) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = (((crate::simd_ops::ru8(rsi)) as i8 as i32) as u32) as u32 as u64; r14 = (((crate::simd_ops::ru8(r15)) as i8 as i32) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((r14 as u32)) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 5; continue; }
            5 => { r15 = ((r15).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(1)) as u64; fl_l = r15; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140204950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f2f00; const P1: u64 = 0x1402066f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx);
    fl_l = rax; fl_r = rax;
    if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (((rax) as i64).wrapping_mul((0xf4240u64) as i64) as u64) as u64;
    r8 = (0x7fffffffffffffffu64) as u64;
    rax = r8;
    rax = ((rax).wrapping_sub(rcx)) as u64;
    fl_l = rdx; fl_r = rax;
    if !(((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64))) { rax = ((rcx.wrapping_add(rdx)) as u64) as u64; rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8);
    rcx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402049c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f2f00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi, r8);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rax; fl_r = rax;
    if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (((rax) as i64).wrapping_mul((0xf4240u64) as i64) as u64) as u64;
    rdx = (0x7fffffffffffffffu64) as u64;
    rax = rdx;
    rax = ((rax).wrapping_sub(rcx)) as u64;
    fl_l = r8; fl_r = rax;
    if !(((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64))) { rax = ((rcx.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rdi, rdx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140204a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x19; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x8; const P6: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P5), (1) as u32)) as u32 as u64; }
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x0u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140204a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x140207ef0; const P2: u64 = 0x10; const P3: u64 = 0x19; const P4: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru64(rcx)) as u64; rdi = r8; rbx = rdx; rsi = rcx; bb = 2; continue; }
            2 => { r8 = ((rbx.wrapping_add(P0)) as u64) as u64; rdx = rbp; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rbx = rax; bb = 10; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rbx = rax; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rcx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 10; continue; }
            10 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 12; continue; }
            12 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140204b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401e9110; const P2: u64 = 0x140206d00; const P3: u64 = 0x140205d40; const P4: u64 = 0x1401e6f50;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rdx; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rax = (0x4924924924924925u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rcx; rcx = (0x492492492492492u64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; r8 = rax; rcx = (crate::simd_ops::ru64(rbx)) as u64; r11 = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rsi)) as u64; r8 = rdi; rdx = r11; rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            2 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140204bcc, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140204bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdi = rcx; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140204c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x8; const P2: u64 = 0xffffffffffffffc8; const P3: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = rax; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rsi); bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(0x50u64)) as u64; rax = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 7; continue; }
            7 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140204cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rax = (0xcbf29ce484222325u64) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r10 = (0x100000001b3u64) as u64;
    loop { r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(r9))) as u32) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rax = (((rax) ^ (r8))) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; fl_l = r9; fl_r = rdx; if !((((fl_l as u64)) < ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140204d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140206e10; const P1: u64 = 0x14307e950; const P2: u64 = 0x142645870; const P3: u64 = 0x140204dba; const P4: u64 = 0x141834d64; const P5: u64 = 0x143426fc8; const P6: u64 = 0x140206010; const P7: u64 = 0x14183acda;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140204d00, funcinfo_rva: 0x35a3a68, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (r8) as u64; r15 = (rdx) as u64; r13 = (rcx) as u64; r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(r8)) as u64; bb = 2; continue; }
            2 => { rdi = (0xcbf29ce484222325u64) as u64; r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((r12 as u32)) as u32 as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (0x100000001b3u64) as u64; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(rcx))) as u32) as u32 as u64; rdi = (((rdi) ^ (rax))) as u64; rdi = (((rdi) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { r9 = (rdi) as u64; r8 = (rbx) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204d82); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(r15, (rax) as u64); crate::simd_ops::wu8(r15.wrapping_add(0x8u64), ((r12 as u8)) as u8); bb = 45; continue; }
            7 => { rax = (0x2e8ba2e8ba2e8bau64) as u64; fl_l = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((P1) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204db4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P3, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rbp = ((r13.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r12) as u64); rcx = (0x58u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204dce); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rax.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (r12) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), (r12) as u64); let y_addr: u64 = rbx; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rax.wrapping_add(0x10u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); crate::simd_ops::wu8(rax.wrapping_add(0x30u64), (0x0u8) as u8); crate::simd_ops::wu8(rax.wrapping_add(0x50u64), (0x0u8) as u8); rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; xmm2 = ((crate::simd_ops::ru32(r13)) as u32) as u128; fl_l = ((rcx).wrapping_add(0x1u64)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm1 = 0u128; if ((((fl_l as u64)) as i64) < 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 12; continue; }
            11 => { rax = (rcx) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x38u64))) as u64; xmm0 = 0u128; ymm0_hi = 0; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); ymm0_hi = 0; bb = 15; continue; }
            14 => { rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); ymm0_hi = 0; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); ymm0_hi = 0; bb = 15; continue; }
            15 => { xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); ymm0_hi = 0; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 16; continue; }
            16 => { xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); ymm0_hi = 0; xmm1 = 0u128; xmm1 = crate::simd_ops::vroundss(xmm1, (xmm0 as u32), 0x2u8); rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = ((crate::simd_ops::ru32(P5)) as u32) as u128; ymm0_hi = 0; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (0x8000000000000000u64) as u64; rcx = (rax) as u64; bb = 19; continue; }
            19 => { rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (0x8u32) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; rcx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 20; continue; }
            20 => { fl_l = (rdx) as u64 as u64; fl_r = (0x200u64) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdx = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = (rcx) as u64; bb = 23; continue; }
            23 => { rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204ee8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; rax = (((rax) & (rdi))) as u64; rax = ((rax).wrapping_add(rax)) as u64; r12 = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(r12.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbp)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); bb = 36; continue; }
            25 => { r12 = (crate::simd_ops::ru64(r12.wrapping_add(rax.wrapping_mul(8)))) as u64; rbp = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; bb = 26; continue; }
            26 => { rdx = ((rbx.wrapping_add(0x10u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 28; continue; }
            28 => { rcx = ((r14.wrapping_add(0x10u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64 as u64; fl_r = (0xfu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; bb = 30; continue; }
            30 => { fl_l = (rbp) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { r8 = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204f49); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 34; continue; } bb = 32; continue; }
            32 => { fl_l = (rbx) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 26; continue; }
            34 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); rbp = ((r13.wrapping_add(0x8u64)) as u64) as u64; bb = 37; continue; }
            35 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rbp = ((r13.wrapping_add(0x8u64)) as u64) as u64; bb = 36; continue; }
            36 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); bb = 37; continue; }
            37 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(r13.wrapping_add(0x10u64), (1) as u64); crate::simd_ops::wu64(r14, (rdx) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu64(r8, (r14) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (r14) as u64); rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; rax = (((rax) & (rdi))) as u64; rax = ((rax).wrapping_add(rax)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (r9) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp)) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 40; continue; } bb = 39; continue; }
            39 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), (r14) as u64); bb = 43; continue; }
            40 => { fl_l = (r9) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 42; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), (r14) as u64); bb = 44; continue; }
            42 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 44; continue; } bb = 43; continue; }
            43 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64), (r14) as u64); bb = 44; continue; }
            44 => { crate::simd_ops::wu64(r15, (r14) as u64); crate::simd_ops::wu8(r15.wrapping_add(0x8u64), (0x1u8) as u8); bb = 45; continue; }
            45 => { rax = (r15) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x14183acc8; const P4: u64 = 0x1402032e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (0x1fffffffffffffffu64) as u64; r14 = r9; r15 = r8; rsi = rcx; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((rdx.wrapping_mul(8)) as u64) as u64; rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rax); rdi = rax; crate::simd_ops::wu64(rsi.wrapping_add(P1), rax); rcx = ((rbx.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P2), rcx); rcx = rax; rdx = (crate::simd_ops::ru64(r15)) as u64; rbx = (crate::simd_ops::ru64(r14)) as u64; rbx = ((rbx).wrapping_sub(rdx)) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = ((rdi.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P1), rax); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 3; continue; }
            3 => { return rax; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1402050ad, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402050b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x1; const P3: u64 = 0x1401e9110; const P4: u64 = 0x14183acbc; const P5: u64 = 0x140200110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0x7fffffffffffffffu64) as u64; rdi = r8; rbp = rdx; rsi = rcx; fl_l = r8; fl_r = rbx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = r8; fl_r = (0xfu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx.wrapping_add(P0), r8); crate::simd_ops::wu64(rcx.wrapping_add(P1), (0xfu64) as u64); xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); bb = 6; continue; }
            3 => { rax = rdi; rax = (((rax) | (0xfu64))) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0x16; rbx = rax; fl_l = rax; fl_r = rcx; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rbx }) as u64; bb = 5; continue; }
            5 => { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rdi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rsi, rax); rdx = rbp; crate::simd_ops::wu64(rsi.wrapping_add(P0), rdi); rcx = rax; crate::simd_ops::wu64(rsi.wrapping_add(P1), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x140205230, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140203690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rcx; fl_r = rdx;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    loop { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x38u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x30; const P2: u64 = 0x140203600; const P3: u64 = 0x10; const P4: u64 = 0x140203690; const P5: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rax, (0x0u64) as u64);
    rbx = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi);
    loop { rsi = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0x58; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = rsi; fl_l = rsi; fl_r = rsi; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402052e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x1; const P2: u64 = 0x1401e9110; const P3: u64 = 0x10; const P4: u64 = 0x14183acbc; const P5: u64 = 0xfffffffffffffff8; const P6: u64 = 0x141834d30; const P7: u64 = 0x1426467f0; const P8: u64 = 0x140200110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (0x7fffffffffffffffu64) as u64; r15 = r9; rbp = rdx; rdi = rcx; fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rdx = (((rdx) | (0xfu64))) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = rsi; rax = rbx; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rcx.wrapping_add(rsi)) as u64) as u64; rbx = rdx; fl_l = rdx; fl_r = rax; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; bb = 4; continue; }
            4 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbp; crate::simd_ops::wu64(rdi.wrapping_add(P3), rbp); rdx = r15; crate::simd_ops::wu64(rdi.wrapping_add(P0), rbx); rcx = rax; r14 = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(r14.wrapping_add(rbp), (0x0u8) as u8); fl_l = rsi; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rsi.wrapping_add(P1)) as u64) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P5)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = r8; bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi, r14); rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1402053bd, rcx, rsp); return rax; }
            11 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1402053c3, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402053d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x30; const P2: u64 = 0x140203600; const P3: u64 = 0x10; const P4: u64 = 0x140203690; const P5: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rcx;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x58; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x28; const P3: u64 = 0x20; const P4: u64 = 0x30; const P5: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rdx)) as u64; rbp = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P4))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = rsp.wrapping_add(0x40); rdx = rsp.wrapping_add(0x30); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbp, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 7; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402054e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rax))) as u64;
    rax = (0x4924924924924925u64) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64;
    rax = rdx;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rdx)) as u64; rbp = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; bb = 2; continue; }
            2 => { rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P3))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = rsp.wrapping_add(0x40); rdx = rsp.wrapping_add(0x30); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbp, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(0x38u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 7; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402055e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1414a5b24; const P2: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = 0x68; rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414a5b24; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = 0x68; rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402056e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = 0xffffffff; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { rdx = 0x18; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xc; const P2: u64 = 0x8; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx = 0x18; rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); } rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); } rdx = 0x18; rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402057a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x140202c60; const P3: u64 = 0x8; const P4: u64 = 0xc; const P5: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = 0xffffffff; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { rdx = 0x20; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080818; const P1: u64 = 0x140207680;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x37); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r12 = rdx; r14 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r13 = (crate::simd_ops::ru64(rax)) as u64; r13 = ((r13).wrapping_shl(6u32)) as u64; r13 = ((r13).wrapping_add(r14)) as u64; fl_l = r14; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 1; continue; }
            1 => { rdi = r14.wrapping_add(0x28); rdx = (0x6666666666666667u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r10 = (crate::simd_ops::ru64(rdi.wrapping_sub(0x18u64))) as u64; r11 = ((crate::simd_ops::ru8(rdi.wrapping_sub(0x10u64))) as u32) as u32 as u64; r15 = ((crate::simd_ops::ru8(rdi.wrapping_sub(0xfu64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(r14)) as u128; xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x8u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rax = rdx; r9 = (crate::simd_ops::ru64(rdi.wrapping_sub(0x8u64))) as u64; r8 = ((xmm1 as u64)) as u64; rcx = (((xmm1 >> 64) as u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), rbx); let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), r8); rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rdx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r10); rax = P0; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); rax = rbp.wrapping_sub(0x9); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); r8 = rbp.wrapping_sub(0x49); crate::simd_ops::wu8(rbp.wrapping_sub(0x21u64), ((r11 as u8)) as u8); rdx = rbp.wrapping_sub(0x39); crate::simd_ops::wu8(rbp.wrapping_sub(0x20u64), ((r15 as u8)) as u8); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x7u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(r12)) as u128; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), r9); r9 = rbp.wrapping_sub(0x59); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rbx); crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (((rax as u8)) as u32) as u32 as u64; r15 = (((rax as u8)) as u32) as u32 as u64; rcx = 0xffffffff; let xa_src: u32 = (rcx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; bb = 12; continue; }
            11 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), r9); r9 = rbp.wrapping_sub(0x59); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rbx); crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (((rax as u8)) as u32) as u32 as u64; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2fu64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rcx = 0xffffffff; let xa_src: u32 = (rcx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2fu64))) as u64; rcx = rbx; rdx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2fu64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { r14 = ((r14).wrapping_add(0x40u64)) as u64; rdi = ((rdi).wrapping_add(0x40u64)) as u64; rdx = (0x6666666666666667u64) as u64; fl_l = r14; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 18; continue; }
            18 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            19 => { r11 = rsp.wrapping_add(0xb0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 19; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x8; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rsi = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rdx)) as u64; r14 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdi = ((rbx.wrapping_add(P1)) as u64) as u64; bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x28u64)) as u64; rdi = ((rdi).wrapping_add(0x28u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x140203020; const P2: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = 0x8; rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140203020; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = 0x8; rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x14183acc8; const P2: u64 = 0x14183acc2; const P3: u64 = 0x141834d30; const P4: u64 = 0x1426467f0; const P5: u64 = 0x1401e6f50; const P6: u64 = 0x140205ff0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rbp = rcx; rbx = 0x1; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdx = (0xfffffffffffffffu64) as u64; fl_l = rcx; fl_r = rcx; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rbx }) as u64; bb = 1; continue; }
            1 => { rax = rbx; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = rdx; rax = ((rax).wrapping_sub(rbx)) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx).wrapping_add(rbx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 1; continue; }
            5 => { rax = (0x1fffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rcx = ((rbx.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; r15 = ((rsi.wrapping_mul(8)) as u64) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; r14 = rax; r12 = ((r15.wrapping_add(rax)) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rbx = ((rbx).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rbx = ((rbx).wrapping_sub(rcx)) as u64; rdi = ((rcx.wrapping_mul(8)) as u64) as u64; rcx = r12; rdx = ((rax.wrapping_add(r15)) as u64) as u64; rdi = ((rdi).wrapping_sub(rdx)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; r8 = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r12 = ((r12).wrapping_add(rdi)) as u64; rcx = r12; fl_l = rsi; fl_r = rbx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = r15; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; rcx = ((r15.wrapping_add(r12)) as u64) as u64; r8 = ((r8).wrapping_sub(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = r15; rcx = r14; bb = 11; continue; }
            10 => { rsi = ((rbx.wrapping_mul(8)) as u64) as u64; r8 = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = r14; rdx = ((rsi.wrapping_add(rdi)) as u64) as u64; rdi = ((rdi).wrapping_sub(rdx)) as u64; rdi = ((rdi).wrapping_add(r15)) as u64; r8 = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(r14)) as u64) as u64; r8 = rsi; bb = 11; continue; }
            11 => { rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rcx = r8; bb = 15; continue; }
            15 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x10u64), (rbx) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140205d31, rcx, rsp); return rax; }
            18 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140205d37, rcx, rsp); return rax; }
            19 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140205d3d, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140203690; const P2: u64 = 0x10; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x141834d30; const P5: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; r14 = r9; r15 = r8; rbp = rdx; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x38u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = (0x4924924924924925u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = (((rdx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; rax = ((r8.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { r8 = rcx; bb = 6; continue; }
            6 => { rcx = r8; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsi, rbp); rax = (((r15) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(rbp)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); rax = (((r14) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(rbp)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P2), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140205e10, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30; const P1: u64 = 0x20; const P2: u64 = 0x140203460; const P3: u64 = 0x10; const P4: u64 = 0x8; const P5: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = 0xffffffff; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P5), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 7; continue; }
            7 => { rdx = 0x38; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140205fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x58; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140206010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307e970; const P1: u64 = 0x142645870; const P2: u64 = 0x140203a40; const P3: u64 = 0x14183acda;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (0xfffffffffffffffu64) as u64; r9 = 0x1; let bit_in: u64 = rax; if bit_in != 0 { r8 = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; r15 = rcx; rax = ((r9).wrapping_shl((((r8) as u32) & 63))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140206049, rcx, rsp); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rax = rdx.wrapping_sub(0x1); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rax = (((rax) | (r9))) as u64; rbp = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); r8 = rbp; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rbx = ((r9).wrapping_shl((((rcx) as u32) & 63))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); r13 = r15.wrapping_add(0x18); rcx = r13; rdx = ((rbx.wrapping_add(rbx)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r15.wrapping_add(0x38u64), rbx); rax = rbx.wrapping_sub(0x1); crate::simd_ops::wu64(r15.wrapping_add(0x30u64), rax); rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rdi)) as u64; rsi = rdi; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); rax = (0xcbf29ce484222325u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rbx = (0x100000001b3u64) as u64; bb = 4; continue; }
            4 => { r12 = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r9 = rdi.wrapping_add(0x10); rsi = (crate::simd_ops::ru64(rsi)) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = r12; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; r10 = r8; r11 = r12; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(rdx))) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rcx = (((rcx) & (crate::simd_ops::ru64(r15.wrapping_add(0x30u64))))) as u64; rax = (crate::simd_ops::ru64(r13)) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); r13 = (crate::simd_ops::ru64(rax)) as u64; fl_l = r13; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rax, rdi); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdi); bb = 30; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_r = (0xfu64) as u64; rdx = rbx.wrapping_add(0x10); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 13; continue; }
            13 => { rcx = rdi.wrapping_add(0x10); r15 = r8; fl_l = r12; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; r15 = r10; r12 = r11; bb = 15; continue; }
            15 => { fl_l = r8; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r8; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rsi); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, r8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rdi); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdi); bb = 29; continue; }
            20 => { fl_l = r13; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_r = (0xfu64) as u64; rdx = rbx.wrapping_add(0x10); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 23; continue; }
            23 => { rcx = rdi.wrapping_add(0x10); fl_l = r12; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; bb = 25; continue; }
            25 => { fl_l = r15; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { r8 = r15; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 33; continue; } bb = 27; continue; }
            27 => { fl_l = r13; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rsi); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, rbx); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rax, rdi); bb = 29; continue; }
            29 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (0x100000001b3u64) as u64; bb = 30; continue; }
            30 => { r13 = r15.wrapping_add(0x18); rdi = rsi; rax = (0xcbf29ce484222325u64) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 31; continue; }
            31 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 32; continue; }
            32 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            33 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rsi); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, r8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rdi); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); bb = 29; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140206290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x143426fc8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; xmm0 = 0u128; fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 3; continue; }
            2 => { rax = rdx; rdx = ((((rdx as u32)) & (0x1u32))) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = (((rax) | (rdx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 3; continue; }
            3 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm1 = 0u128; xmm1 = crate::simd_ops::vroundss(xmm1, (xmm0 as u32), 0x2u8); xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; rax = 0; fl_l = 0; fl_r = 0; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (0x8000000000000000u64) as u64; rax = rcx; bb = 6; continue; }
            6 => { rdx = 0x8; rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rcx; fl_r = rdx; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rcx } else { rdx }) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = r8; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { fl_l = r8; fl_r = (0x200u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((r8.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rdx; rdx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rax } else { rdx }) as u64; bb = 10; continue; }
            10 => { rax = rdx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140206330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x142646578; const P2: u64 = 0x142645838; const P3: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = (0xffffffffffffffffu64) as u64;
    loop { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax).wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = ((rax.wrapping_add(P0)) as u64) as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    rsi = ((rax.wrapping_add(P0)) as u64) as u64;
    rdx = 0x1;
    rcx = rsi;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    rdi = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14020637b, rcx, rsp); return rax; }
    fl_l = rsi; fl_r = rsi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = rsi; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402063b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1402082e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1402063b0, funcinfo_rva: 0x35a3e18, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = (r8) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, (rbp) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rbp) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbp) as u64); r8 = ((r8).wrapping_sub(rdx)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206401); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (rax) as u64; fl_l = 0; fl_r = (rcx) as u64 as u64; rcx = (0u64.wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (rax) as u64; rcx = (!(rcx)) as u64; rcx = ((rcx).wrapping_shr(5u32)) as u64; rcx = ((rcx.wrapping_mul(4).wrapping_add(0x4u64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rcx) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); bb = 4; continue; }
            3 => { rcx = (rax) as u64; fl_l = ((rcx).wrapping_shr(5u32)) as u64 as u64; rcx = ((fl_l as u64)) as u64; rcx = ((rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); bb = 4; continue; }
            4 => { rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { r11 = (0x1u32) as u32 as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (((r11 as u32)).wrapping_shl(((((rax as u32)) as u32) & 31))) as u32 as u64; fl_l = (crate::simd_ops::lk_or_u32(rdx, ((rcx as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 9; continue; }
            8 => { rax = (((r11 as u32)).wrapping_shl(((((rcx as u32)) as u32) & 31))) as u32 as u64; rax = (!((rax as u32))) as u32 as u64; fl_l = (crate::simd_ops::lk_and_u32(rdx, ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l; r8 = (r9) as u64; bb = 9; continue; }
            9 => { fl_l = (r8) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = ((r10.wrapping_add(0x1u64)) as u64) as u64; r10 = (rax) as u64; rcx = (rax) as u64; r8 = (rax) as u64; r9 = (rax) as u64; bb = 12; continue; }
            11 => { r10 = (rbp) as u64; fl_l = ((rdx).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rax = (rbp) as u64; rcx = (rbp) as u64; r8 = (rbp) as u64; r9 = (rbp) as u64; bb = 12; continue; }
            12 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 13; continue; }
            13 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402064e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1402050d0; const P2: u64 = 0x1402063b0; const P3: u64 = 0x140208950; const P4: u64 = 0x140203cf0; const P5: u64 = 0x140203f20; const P6: u64 = 0x140204220; const P7: u64 = 0x1402040a0; const P8: u64 = 0x140204520; const P9: u64 = 0x1402046a0; const P10: u64 = 0x140208800; const P11: u64 = 0x140208830; const P12: u64 = 0x1402087f0; const P13: u64 = 0x1402066a1;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1402064e0, funcinfo_rva: 0x35a3e44, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r9 = (r8) as u64; rbx = (rcx) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (((crate::simd_ops::ru8(r8.wrapping_add(0x10u64))) as i8 as i64) as u64) as u64; rcx = ((P0) as u64) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x2066a8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = (rax) as u64; bb = 0xffff_fffe; continue; }
            1 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; crate::simd_ops::wu8(rbx, ((rax as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), ((rdi as u8)) as u8); bb = 19; continue; }
            2 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0x1u8) as u8); bb = 19; continue; }
            3 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0x3u8) as u8); bb = 19; continue; }
            4 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0x2u8) as u8); bb = 19; continue; }
            5 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0x4u8) as u8); bb = 19; continue; }
            6 => { rdx = (crate::simd_ops::ru64(r8)) as u64; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rdi) as u64); r8 = (0xffffffffffffffffu64) as u64; bb = 7; continue; }
            7 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206583); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; let y_addr: u64 = rbp.wrapping_sub(0x40u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbx; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0x5u8) as u8); bb = 19; continue; }
            9 => { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rdi) as u64); r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(r9)) as u64; bb = 8; continue; }
            10 => { r8 = (crate::simd_ops::ru64(r8)) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402065c2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402065cf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0x6u8) as u8); rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402065e4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            11 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); r8 = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402065ff); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); r8 = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206617); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            13 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); r8 = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020662f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            14 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); r8 = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206647); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            15 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); r8 = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020665f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            16 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0xcu8) as u8); bb = 19; continue; }
            17 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206675); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            18 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020667f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            19 => { rax = (rbx) as u64; r11 = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = (r11) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020669c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P13, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1402064e0u64) { 0x32 => { bb = 1; continue; } 0x41 => { bb = 2; continue; } 0x4f => { bb = 3; continue; } 0x5e => { bb = 4; continue; } 0x6c => { bb = 5; continue; } 0x7b => { bb = 6; continue; } 0x96 => { bb = 7; continue; } 0x9f => { bb = 8; continue; } 0xba => { bb = 9; continue; } 0xd4 => { bb = 10; continue; } 0x10e => { bb = 11; continue; } 0x126 => { bb = 12; continue; } 0x13e => { bb = 13; continue; } 0x156 => { bb = 14; continue; } 0x16e => { bb = 15; continue; } 0x186 => { bb = 16; continue; } 0x192 => { bb = 17; continue; } 0x19c => { bb = 18; continue; } 0x1a4 => { bb = 19; continue; } 0x1bc => { bb = 20; continue; } _ => { drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402066f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645888; const P1: u64 = 0x142645880; const P2: u64 = 0x141833c3c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rsi = (0x4e94914f0000u64) as u64; rbp = (0x431bde82d7b634dbu64) as u64; r14 = (0xb2f4fc0794908cf3u64) as u64; bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rbx; fl_r = (0x989680u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (((rcx) as i64).wrapping_mul((0x64u64) as i64) as u64) as u64; bb = 7; continue; }
            3 => { fl_l = rbx; fl_r = (0x16e3600u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = r14; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r8 = ((rcx.wrapping_add(rdx)) as u64) as u64; r8 = ((((r8) as i64).wrapping_shr(24u32) as u64)) as u64; rax = r8; rax = ((rax).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rax = (((r8) as i64).wrapping_mul((0x16e3600u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = r14; rcx = (((rcx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rax = ((rcx.wrapping_add(rdx)) as u64) as u64; rax = ((((rax) as i64).wrapping_shr(24u32) as u64)) as u64; rcx = rax; rcx = ((rcx).wrapping_shr(63u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (((r8) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; bb = 6; continue; }
            5 => { rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rbx) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rcx = rax; rax = (((rdx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rbx) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rcx = (((rcx) as i64).wrapping_mul((0x3b9aca00u64) as i64) as u64) as u64; bb = 6; continue; }
            6 => { fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rcx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 13; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = rcx; fl_r = rsi; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = 0x5265c00; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            10 => { rax = rbp; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(18u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (((rdx) as i64).wrapping_mul((0xf4240u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((rdx).wrapping_add(1)) as u64; rdx = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rcx = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140206830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x140203210; const P2: u64 = 0x140206d00; const P3: u64 = 0x140205d40; const P4: u64 = 0x1401e6f50; const P5: u64 = 0x1402069aa; const P6: u64 = 0x1402032e0; const P7: u64 = 0x1402069b0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140206830, funcinfo_rva: 0x35a3e94, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r12 = (rdx) as u64; r14 = (rcx) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; r10 = (rdx) as u64; r10 = ((r10).wrapping_sub(r9)) as u64; r11 = (0x4924924924924925u64) as u64; rax = (r11) as u64; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = (rdx) as u64; rsi = ((((rsi) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (rsi) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(r9)) as u64; rax = (r11) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = (0x492492492492492u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { r13 = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(r9)) as u64; rax = (r11) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = (r8) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rbx = (r8) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); bb = 3; continue; }
            3 => { rcx = (((rbx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402068e7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rsi = (((rsi) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; r15 = ((rsi.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (r15) as u64); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206914); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x140206914, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x140206830) }; rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsi.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rsi) as u64); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; r8 = (rdi) as u64; fl_l = (r12) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020694b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14020694b, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x140206830) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rdi) as u64); r8 = (r15) as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (r12) as u64; bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020695f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14020695f, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x140206830) }; rax = c_rax; xmm0 = c_x0; r9 = (rbx) as u64; r8 = (r13) as u64; rdx = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206971); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x140206971, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x140206830) }; rax = c_rax; xmm0 = c_x0; rax = (rsi) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rax = ((rcx.wrapping_add(rdx)) as u64) as u64; rbx = (r13) as u64; fl_l = (rax) as u64 as u64; fl_r = (r13) as u64 as u64; rbx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rax } else { rbx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402069a5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402069ab); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402069c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x1402050d0; const P2: u64 = 0x140206d00; const P3: u64 = 0x140205d40; const P4: u64 = 0x1401e6f50; const P5: u64 = 0x140206b4d; const P6: u64 = 0x1402032e0; const P7: u64 = 0x140206b53;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1402069c0, funcinfo_rva: 0x35a3ed8, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r12 = (rdx) as u64; r14 = (rcx) as u64; r11 = (crate::simd_ops::ru64(rcx)) as u64; r10 = (rdx) as u64; r10 = ((r10).wrapping_sub(r11)) as u64; r9 = (0x4924924924924925u64) as u64; rax = (r9) as u64; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = (rdx) as u64; rdi = ((((rdi) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (rdi) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(r11)) as u64; rax = (r9) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = (0x492492492492492u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { r13 = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(r11)) as u64; rax = (r9) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = (r8) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rbx = (r8) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); bb = 3; continue; }
            3 => { rcx = (((rbx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206a77); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rdi = (((rdi) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; r15 = ((rdi.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (r15) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206abd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x140206abd, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1402069c0) }; rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu8(rdi.wrapping_add(0x30u64), (0x5u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rdi) as u64); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; r8 = (rsi) as u64; fl_l = (r12) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206aee); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x140206aee, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1402069c0) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rsi) as u64); r8 = (r15) as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (r12) as u64; bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206b02); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x140206b02, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1402069c0) }; rax = c_rax; xmm0 = c_x0; r9 = (rbx) as u64; r8 = (r13) as u64; rdx = (rsi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206b14); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x140206b14, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1402069c0) }; rax = c_rax; xmm0 = c_x0; rax = (rdi) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rax = ((rcx.wrapping_add(rdx)) as u64) as u64; rbx = (r13) as u64; fl_l = (rax) as u64 as u64; fl_r = (r13) as u64 as u64; rbx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rax } else { rbx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206b48); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206b4e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140206b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x1402050d0; const P2: u64 = 0x140206d00; const P3: u64 = 0x140205d40; const P4: u64 = 0x1401e6f50; const P5: u64 = 0x140206cf3; const P6: u64 = 0x1402032e0; const P7: u64 = 0x140206cf9;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140206b60, funcinfo_rva: 0x35a3f0c, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r12 = (rdx) as u64; r14 = (rcx) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; r10 = (rdx) as u64; r10 = ((r10).wrapping_sub(r9)) as u64; r11 = (0x4924924924924925u64) as u64; rax = (r11) as u64; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = (rdx) as u64; rdi = ((((rdi) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (rdi) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(r9)) as u64; rax = (r11) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = (0x492492492492492u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { r13 = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(r9)) as u64; rax = (r11) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = (r8) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rbx = (r8) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); bb = 3; continue; }
            3 => { rcx = (((rbx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206c17); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rdi = (((rdi) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; r15 = ((rdi.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (r15) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206c5d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x140206c5d, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x140206b60) }; rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rdi) as u64); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; r8 = (rsi) as u64; fl_l = (r12) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206c94); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x140206c94, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x140206b60) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rsi) as u64); r8 = (r15) as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (r12) as u64; bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206ca8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x140206ca8, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x140206b60) }; rax = c_rax; xmm0 = c_x0; r9 = (rbx) as u64; r8 = (r13) as u64; rdx = (rsi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206cba); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x140206cba, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x140206b60) }; rax = c_rax; xmm0 = c_x0; rax = (rdi) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rax = ((rcx.wrapping_add(rdx)) as u64) as u64; rbx = (r13) as u64; fl_l = (rax) as u64 as u64; fl_r = (r13) as u64 as u64; rbx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rax } else { rbx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206cee); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140206cf4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140206d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0x38; const P4: u64 = 0xffffffffffffffc0; const P5: u64 = 0xffffffffffffffc8; const P6: u64 = 0xffffffffffffffb0; const P7: u64 = 0xffffffffffffffd0; const P8: u64 = 0xffffffffffffffe0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = r8;
    fl_l = rcx; fl_r = rdx;
    if (((fl_l as u64)) == ((fl_r as u64))) { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    r9 = rax;
    r8 = ((rcx.wrapping_add(P0)) as u64) as u64;
    r9 = ((r9).wrapping_sub(rcx)) as u64;
    r10 = 0;
    fl_l = 0; fl_r = 0;
    loop { xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu64(r8.wrapping_add(r9).wrapping_add(P1), r10); crate::simd_ops::wu64(r8.wrapping_add(r9), r10); let y_addr: u64 = r8.wrapping_add(P2); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); r8 = ((r8.wrapping_add(P3)) as u64) as u64; let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(r8.wrapping_add(P4), r10); rcx = ((r8.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(P5), (0xfu64) as u64); rax = ((rax).wrapping_add(0x38u64)) as u64; crate::simd_ops::wu8(r8.wrapping_add(P6), ((r10 as u8)) as u8); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(P7))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(r9.wrapping_add(r8).wrapping_add(P7), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(P8))) as u64) as u128; crate::simd_ops::wu64(r9.wrapping_add(r8).wrapping_add(P8), ((xmm1 as u64)) as u64); fl_l = rcx; fl_r = rdx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140206d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x19; const P1: u64 = 0x10; const P2: u64 = 0x140206d80; const P3: u64 = 0x40; const P4: u64 = 0x140203690; const P5: u64 = 0x20; const P6: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = r8; rbp = rdx; r14 = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = rbp; rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0x60; rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140206e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x10; const P4: u64 = 0x28; const P5: u64 = 0x14183acda;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = r8; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r14 = (((r14) & (r9))) as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; rdi = rdx; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P2)))) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdx, rax); rax = rdx; crate::simd_ops::wu64(rdx.wrapping_add(P1), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { r14 = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rbp = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); r15 = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_r = (0xfu64) as u64; rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 5; continue; }
            5 => { rcx = rsi; fl_l = r15; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; bb = 7; continue; }
            7 => { fl_l = rbp; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; bb = 3; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P1), rbx); bb = 13; continue; }
            12 => { crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu64(rdi.wrapping_add(P1), (0x0u64) as u64); bb = 13; continue; }
            13 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rdi; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140206ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x140203600; const P2: u64 = 0x140203140; const P3: u64 = 0x141834d30; const P4: u64 = 0x1426467f0; const P5: u64 = 0x140203690; const P6: u64 = 0x140203d50; const P7: u64 = 0x140203b50; const P8: u64 = 0x140207be0;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbx = r8; rdx = P0; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x207574u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = rax; bb = 0xffff_fffe; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0xffu8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rdi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; crate::simd_ops::wu8(rdi, ((rax as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rdi.wrapping_add(0x20u64), (0xffu8) as u8); rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rdi.wrapping_add(0x20u64), (0xffu8) as u8); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rdi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rdi.wrapping_add(0x20u64), (0xffu8) as u8); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x2u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rdi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rdi, rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rdi.wrapping_add(0x20u64), (0xffu8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x3u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rdi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 14; continue; }
            14 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rdi.wrapping_add(0x20u64), (0xffu8) as u8); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0x20u64), (0x4u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rdi = (crate::simd_ops::ru64(rsi)) as u64; rcx = rdi; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rdi.wrapping_add(0x20u64), (0xffu8) as u8); rdi = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rsi)) as u64; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); let y_addr: u64 = rbx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, ((rdi as u8)) as u8); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x5u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { r14 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 19; continue; }
            19 => { fl_l = r14; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 24; continue; } bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 48; continue; } bb = 23; continue; }
            23 => { rcx = r8; bb = 24; continue; }
            24 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rdi); bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r14, rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0xffu8) as u8); rdi = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rdx); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(r8.wrapping_add(0x18u64), rax); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x6u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { r14 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 38; continue; } bb = 28; continue; }
            28 => { fl_l = r14; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 30; continue; }
            30 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; fl_l = (((rdx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 31; continue; }
            31 => { fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 34; continue; } bb = 32; continue; }
            32 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 48; continue; } bb = 33; continue; }
            33 => { rcx = r8; bb = 34; continue; }
            34 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rdi); bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r14, rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); bb = 37; continue; }
            37 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            38 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0xffu8) as u8); rdi = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rdx); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x7u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            39 => { r14 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 40; continue; }
            40 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0xffu8) as u8); rdi = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rdx); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x8u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            41 => { r14 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 45; continue; } bb = 42; continue; }
            42 => { fl_l = r14; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 43; continue; }
            43 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 44; continue; }
            44 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; fl_l = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 31; continue; }
            45 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0xffu8) as u8); rdi = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rdx); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x9u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            46 => { r14 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 42; continue; } bb = 47; continue; }
            47 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0xffu8) as u8); rdi = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rdx); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0xau8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            48 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1402073e2, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            49 => { r14 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xbu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 55; continue; } bb = 50; continue; }
            50 => { fl_l = r14; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 51; continue; }
            51 => { rsi = (crate::simd_ops::ru64(r14)) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 52; continue; }
            52 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rbp = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 54; continue; } bb = 53; continue; }
            53 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(0x20u64)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 53; continue; } bb = 54; continue; }
            54 => { rdx = (crate::simd_ops::ru64(r14)) as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; r8 = ((((r8) as i64).wrapping_shr(5u32) as u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rdi); bb = 36; continue; }
            55 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0xffu8) as u8); rdi = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rdx); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0xbu8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            56 => { rdi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 57; continue; }
            57 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rdi.wrapping_add(0x20u64), (0xffu8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0xcu8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            58 => { r14 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 62; continue; } bb = 59; continue; }
            59 => { fl_l = r14; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 60; continue; }
            60 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 61; continue; }
            61 => { r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; r8 = ((((r8) as i64).wrapping_shr(3u32) as u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            62 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0xffu8) as u8); rdi = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rdx); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0xdu8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            63 => { rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x140206ee0u64) { 0x26 => { bb = 1; continue; } 0x46 => { bb = 2; continue; } 0x4f => { bb = 3; continue; } 0x66 => { bb = 4; continue; } 0x92 => { bb = 5; continue; } 0x9b => { bb = 6; continue; } 0xb1 => { bb = 7; continue; } 0xdc => { bb = 8; continue; } 0xe5 => { bb = 9; continue; } 0x110 => { bb = 10; continue; } 0x119 => { bb = 11; continue; } 0x130 => { bb = 12; continue; } 0x15d => { bb = 13; continue; } 0x166 => { bb = 14; continue; } 0x190 => { bb = 15; continue; } 0x19c => { bb = 16; continue; } 0x1b5 => { bb = 17; continue; } 0x202 => { bb = 18; continue; } 0x210 => { bb = 19; continue; } 0x219 => { bb = 20; continue; } 0x223 => { bb = 21; continue; } 0x237 => { bb = 22; continue; } 0x250 => { bb = 23; continue; } 0x253 => { bb = 24; continue; } 0x263 => { bb = 25; continue; } 0x2a1 => { bb = 26; continue; } 0x2f8 => { bb = 27; continue; } 0x306 => { bb = 28; continue; } 0x30b => { bb = 29; continue; } 0x315 => { bb = 30; continue; } 0x320 => { bb = 31; continue; } 0x329 => { bb = 32; continue; } 0x342 => { bb = 33; continue; } 0x345 => { bb = 34; continue; } 0x34a => { bb = 35; continue; } 0x355 => { bb = 36; continue; } 0x376 => { bb = 37; continue; } 0x387 => { bb = 38; continue; } 0x3d2 => { bb = 39; continue; } 0x3e0 => { bb = 40; continue; } 0x42b => { bb = 41; continue; } 0x435 => { bb = 42; continue; } 0x43e => { bb = 43; continue; } 0x44c => { bb = 44; continue; } 0x45c => { bb = 45; continue; } 0x4a7 => { bb = 46; continue; } 0x4b1 => { bb = 47; continue; } 0x4fc => { bb = 48; continue; } 0x503 => { bb = 49; continue; } 0x50d => { bb = 50; continue; } 0x516 => { bb = 51; continue; } 0x524 => { bb = 52; continue; } 0x532 => { bb = 53; continue; } 0x543 => { bb = 54; continue; } 0x56b => { bb = 55; continue; } 0x5b6 => { bb = 56; continue; } 0x5c3 => { bb = 57; continue; } 0x5f0 => { bb = 58; continue; } 0x5fa => { bb = 59; continue; } 0x603 => { bb = 60; continue; } 0x611 => { bb = 61; continue; } 0x626 => { bb = 62; continue; } 0x671 => { bb = 63; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402075c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x140203690; const P2: u64 = 0x140203cf0; const P3: u64 = 0x140202a90; const P4: u64 = 0x140203c60; const P5: u64 = 0x140203c00;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = P0; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x207614u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = rax; bb = 0xffff_fffe; continue; }
            1 => { rcx = r8; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { return rax; }
            3 => { rcx = r8; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { rcx = r8; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { rcx = r8; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rcx = r8; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { rcx = r8; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { rcx = r8; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { rcx = r8; return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1402075c0u64) { 0x13 => { bb = 1; continue; } 0x1b => { bb = 2; continue; } 0x1c => { bb = 3; continue; } 0x24 => { bb = 4; continue; } 0x2c => { bb = 5; continue; } 0x34 => { bb = 6; continue; } 0x3c => { bb = 7; continue; } 0x44 => { bb = 8; continue; } 0x4c => { bb = 9; continue; } _ => { return crate::simd_ops::call_fwd(j_tgt, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140207660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140203600; const P1: u64 = 0x140203690;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x20u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140207680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rsi = rdx; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rdx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x18u64))) as u32) as u32 as u64; r11 = ((crate::simd_ops::ru8(rsi.wrapping_add(0x19u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; r10 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rdx as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x39u64), ((r11 as u8)) as u8); rdi = 0xffffffff; rdx = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(r9)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(r10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (((rax as u8)) as u32) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (((rbp as u8)) as u32) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (((rbp as u8)) as u32) as u32 as u64; bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(r10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 18; continue; }
            13 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 18; continue; }
            18 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402077f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (crate::simd_ops::ru64(rcx)) as u64; rbx = (rdx) as u64; r15 = (rcx) as u64; rax = (((crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rcx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { rdi = (0xffffffffu32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 15; continue; }
            7 => { rcx = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x207918u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = (rax) as u64; bb = 0xffff_fffe; continue; }
            8 => { rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdi = (0xffffffffu32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { crate::simd_ops::wu8(r14.wrapping_add(0x10u64), (0xffu8) as u8); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x10u64), (0x5u8) as u8); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1402077f0u64) { 0x27 => { bb = 1; continue; } 0x30 => { bb = 2; continue; } 0x34 => { bb = 3; continue; } 0x4f => { bb = 4; continue; } 0x64 => { bb = 5; continue; } 0x7a => { bb = 6; continue; } 0x88 => { bb = 7; continue; } 0xa0 => { bb = 8; continue; } 0xa9 => { bb = 9; continue; } 0xba => { bb = 10; continue; } 0xcc => { bb = 11; continue; } 0xd5 => { bb = 12; continue; } 0xef => { bb = 13; continue; } 0xf3 => { bb = 14; continue; } 0x10d => { bb = 15; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140207940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (crate::simd_ops::ru64(rcx)) as u64; rbx = (rdx) as u64; r15 = (rcx) as u64; rax = (((crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rcx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { rdi = (0xffffffffu32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 15; continue; }
            7 => { rcx = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x207a68u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = (rax) as u64; bb = 0xffff_fffe; continue; }
            8 => { rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdi = (0xffffffffu32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { crate::simd_ops::wu8(r14.wrapping_add(0x10u64), (0xffu8) as u8); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x10u64), (0x6u8) as u8); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x140207940u64) { 0x27 => { bb = 1; continue; } 0x30 => { bb = 2; continue; } 0x34 => { bb = 3; continue; } 0x4f => { bb = 4; continue; } 0x64 => { bb = 5; continue; } 0x7a => { bb = 6; continue; } 0x88 => { bb = 7; continue; } 0xa0 => { bb = 8; continue; } 0xa9 => { bb = 9; continue; } 0xba => { bb = 10; continue; } 0xcc => { bb = 11; continue; } 0xd5 => { bb = 12; continue; } 0xef => { bb = 13; continue; } 0xf3 => { bb = 14; continue; } 0x10d => { bb = 15; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140207a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (crate::simd_ops::ru64(rcx)) as u64; rbx = (rdx) as u64; r15 = (rcx) as u64; rax = (((crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rcx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { rdi = (0xffffffffu32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 15; continue; }
            7 => { rcx = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x207bb8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = (rax) as u64; bb = 0xffff_fffe; continue; }
            8 => { rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdi = (0xffffffffu32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { crate::simd_ops::wu8(r14.wrapping_add(0x10u64), (0xffu8) as u8); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x10u64), (0x7u8) as u8); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x140207a90u64) { 0x27 => { bb = 1; continue; } 0x30 => { bb = 2; continue; } 0x34 => { bb = 3; continue; } 0x4f => { bb = 4; continue; } 0x64 => { bb = 5; continue; } 0x7a => { bb = 6; continue; } 0x88 => { bb = 7; continue; } 0xa0 => { bb = 8; continue; } 0xa9 => { bb = 9; continue; } 0xba => { bb = 10; continue; } 0xcc => { bb = 11; continue; } 0xd5 => { bb = 12; continue; } 0xef => { bb = 13; continue; } 0xf3 => { bb = 14; continue; } 0x10d => { bb = 15; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140207be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30; const P1: u64 = 0x1426467f0; const P2: u64 = 0x140203600;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; rdi = rcx; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0xeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = r8; bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rsi); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rsi); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rsi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140207c87, rcx, rsp); return rax; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0xffu8) as u8); rdx = (crate::simd_ops::ru64(rsi)) as u64; rsi = 0; fl_l = 0; fl_r = 0; r9 = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rsi); crate::simd_ops::wu64(rdx, rsi); crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(r9.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r9.wrapping_add(0x10u64), r8); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0xeu8) as u8); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140207ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x14183acc8; const P4: u64 = 0x1402032e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (0x7fffffffffffffffu64) as u64; r14 = r9; r15 = r8; rbx = rdx; rsi = rcx; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rax); rdi = rax; crate::simd_ops::wu64(rsi.wrapping_add(P1), rax); rcx = ((rax.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P2), rcx); rcx = rax; rdx = (crate::simd_ops::ru64(r15)) as u64; rbx = (crate::simd_ops::ru64(r14)) as u64; rbx = ((rbx).wrapping_sub(rdx)) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rbx.wrapping_add(rdi)) as u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P1), rax); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 3; continue; }
            3 => { return rax; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140207d74, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140207d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x14183acc8; const P4: u64 = 0x1402032e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (0x3fffffffffffffffu64) as u64; r14 = r9; r15 = r8; rsi = rcx; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((rdx.wrapping_mul(4)) as u64) as u64; rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rax); rdi = rax; crate::simd_ops::wu64(rsi.wrapping_add(P1), rax); rcx = ((rbx.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P2), rcx); rcx = rax; rdx = (crate::simd_ops::ru64(r15)) as u64; rbx = (crate::simd_ops::ru64(r14)) as u64; rbx = ((rbx).wrapping_sub(rdx)) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = ((rdi.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P1), rax); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 3; continue; }
            3 => { return rax; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140207e1d, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140207e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1402050d0; const P5: u64 = 0x1402032e0; const P6: u64 = 0x140207eed;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140207e20, funcinfo_rva: 0x35a40cc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = (r9) as u64; r14 = (r8) as u64; rbx = (rdx) as u64; rsi = (rcx) as u64; rax = (0x7ffffffffffffffu64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_shl(5u32)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140207e61); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); crate::simd_ops::wu64(rsi.wrapping_add(P1), (rax) as u64); rcx = ((rbx.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P2), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rsi) as u64); rbp = (crate::simd_ops::ru64(rbp)) as u64; rbx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(P2), (r14) as u64); crate::simd_ops::wu64(rdi.wrapping_add(P3), (r14) as u64); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140207eba); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rdi).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rdi) as u64); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsi.wrapping_add(P1), (rdi) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140207ee8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140207ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x140203210; const P2: u64 = 0x140200480; const P3: u64 = 0x140208a10; const P4: u64 = 0x1402088c0; const P5: u64 = 0x140208b50; const P6: u64 = 0x140208500; const P7: u64 = 0x140200130;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140207ef0, funcinfo_rva: 0x35a4110, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rdi = (r8) as u64; rbp = (rdx) as u64; r14 = (rcx) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x0u64) as u64); rcx = (0x60u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140207f2a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rbx = ((rax.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), (rbx) as u64); rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140207f49); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbx.wrapping_add(0x20u64)) as u64) as u64; rdx = ((rdi.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140207f57); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r15, (rsi) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (rsi) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu16(r15.wrapping_add(0x18u64), (0x0u16) as u16); rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdi = ((rax.wrapping_add(0x10u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; r8 = ((r8).wrapping_add(0x20u64)) as u64; rdx = ((r15.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140207f91); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = ((r15.wrapping_add(0x20u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140207fa6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 17; continue; }
            5 => { rdx = ((rbp.wrapping_add(0x20u64)) as u64) as u64; r8 = ((r15.wrapping_add(0x20u64)) as u64) as u64; fl_l = (rbp) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax)) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140207fd8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; bb = 15; continue; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140207feb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), (rbp) as u64); rcx = ((rsp.wrapping_add(0xc0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140208004); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rax)) as u64; r8 = ((rdi.wrapping_add(0x20u64)) as u64) as u64; rdx = ((r15.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140208014); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 17; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 17; continue; }
            13 => { r8 = ((r15.wrapping_add(0x20u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020805d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            14 => { rdx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; bb = 15; continue; }
            15 => { r8 = ((r15.wrapping_add(0x20u64)) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140208073); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; bb = 17; continue; }
            17 => { rax = (0x2aaaaaaaaaaaaaau64) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r8 = (r15) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020809d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r11 = ((rsp.wrapping_add(0xa0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402080bf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402080d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = r8;
    rbx = ((rbx).wrapping_sub(rcx)) as u64;
    rdx = rcx;
    r8 = rbx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rbx.wrapping_add(rdi)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140208140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x40; const P2: u64 = 0x140203690; const P3: u64 = 0x20; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rcx;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x60; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402081a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x141834d30; const P3: u64 = 0x8; const P4: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = r8; bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rsi); crate::simd_ops::wu64(rbx.wrapping_add(P3), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rsi); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu64(rdi.wrapping_add(P3), rsi); crate::simd_ops::wu64(rdi.wrapping_add(P0), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 7; continue; }
            7 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140208236, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140208240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x141834d30; const P3: u64 = 0x8; const P4: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = r8; bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rsi); crate::simd_ops::wu64(rbx.wrapping_add(P3), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rsi); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu64(rdi.wrapping_add(P3), rsi); crate::simd_ops::wu64(rdi.wrapping_add(P0), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 7; continue; }
            7 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1402082da, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402082e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140208c90; const P1: u64 = 0x14183acc2; const P2: u64 = 0x1402077d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; rbp = r8; r14 = (crate::simd_ops::ru64(rdx)) as u64; rdi = rcx; r14 = ((r14).wrapping_sub(r10)) as u64; r14 = ((((r14) as i64).wrapping_shr(2u32) as u64)) as u64; r14 = ((r14).wrapping_shl(5u32)) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(0x8u64)))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rax = (0x7fffffffffffffffu64) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 36; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rbx = r8.wrapping_add(0x1f); rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; rcx = rsi; rbx = ((rbx).wrapping_shr(5u32)) as u64; rcx = ((rcx).wrapping_sub(r10)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rdx as u32)) as u32); rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((r10.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); bb = 8; continue; }
            4 => { if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = rsp.wrapping_add(0x50); rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            7 => { rbx = ((rbx).wrapping_sub(rcx)) as u64; rcx = rsi; rbx = ((rbx.wrapping_mul(4)) as u64) as u64; r8 = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rsi).wrapping_add(rbx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rsi); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = ((rcx.wrapping_add(rbp)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 11; continue; }
            11 => { rax = r14; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { r10 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = rcx; fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = rcx; r9 = r10; rax = (!(rax)) as u64; rax = ((rax).wrapping_shr(5u32)) as u64; rax = ((rax.wrapping_mul(4).wrapping_add(0x4u64)) as u64) as u64; fl_l = r9; fl_r = rax; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 16; continue; }
            15 => { rax = rcx; fl_l = ((rax).wrapping_shr(5u32)) as u64; rax = ((fl_l as u64)) as u64; r9 = ((r10.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; bb = 16; continue; }
            16 => { r8 = ((rcx.wrapping_add(rbp)) as u64) as u64; rdx = rcx; rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), r8); fl_l = r8; fl_r = r8; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = r8; fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = r8; rcx = r10; rax = (!(rax)) as u64; rax = ((rax).wrapping_shr(5u32)) as u64; rax = ((rax.wrapping_mul(4).wrapping_add(0x4u64)) as u64) as u64; fl_l = rcx; fl_r = rax; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); bb = 20; continue; }
            19 => { rax = r8; fl_l = ((rax).wrapping_shr(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = ((r10.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); bb = 20; continue; }
            20 => { r8 = ((((r8 as u32)) & (0x1fu32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8); fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = r14; fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = r14; rax = (!(rax)) as u64; rax = ((rax).wrapping_shr(5u32)) as u64; rax = ((rax.wrapping_mul(4).wrapping_add(0x4u64)) as u64) as u64; fl_l = r10; fl_r = rax; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 24; continue; }
            23 => { rax = r14; fl_l = ((rax).wrapping_shr(5u32)) as u64; rax = ((fl_l as u64)) as u64; r10 = ((r10.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; bb = 24; continue; }
            24 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = r14; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = ((((rbx as u32)) & (0x1fu32))) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; bb = 25; continue; }
            25 => { r11 = rdx; fl_l = r10; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 27; continue; }
            27 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { fl_l = ((rdx).wrapping_sub(1)) as u64; rdx = ((fl_l as u64)) as u64; bb = 30; continue; }
            29 => { rdx = 0x1f; bb = 30; continue; }
            30 => { fl_l = r11; fl_r = r11; rax = r9.wrapping_sub(0x4); r9 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { r9 }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { fl_l = ((rcx).wrapping_sub(1)) as u64; rcx = ((fl_l as u64)) as u64; bb = 33; continue; }
            32 => { rcx = 0x1f; fl_l = r8; fl_r = (0x4u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_l = ((((rax as u32)) >> ((((rdx as u32)) as u32 & 31))) & 1) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; if ((fl_l as u32) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { fl_l = ((((rax as u32)) >> ((((rcx as u32)) as u32 & 31))) & 1) as u64; rax = (((rax as u32)) | ((1u32) << ((((rcx as u32)) as u32 & 31)))) as u32 as u64; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); bb = 25; continue; }
            35 => { fl_l = ((((rax as u32)) >> ((((rcx as u32)) as u32 & 31))) & 1) as u64; rax = (((rax as u32)) & !((1u32) << ((((rcx as u32)) as u32 & 31)))) as u32 as u64; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); bb = 25; continue; }
            36 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1402084fe, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140208760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x60; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140208780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = rcx;
    fl_l = rdx; fl_r = rdx;
    if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r9.wrapping_add(P0)))) as u64; rax = rdx; crate::simd_ops::wu64(r9.wrapping_add(P0), rdx); rax = ((rax).wrapping_shr(5u32)) as u64; rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; rax = ((rax).wrapping_shl(2u32)) as u64; fl_l = (crate::simd_ops::lk_add_u64(r9, (rax) as u64)) as u64; fl_r = fl_l; rax = r9; crate::simd_ops::wu64(r9.wrapping_add(P0), rdx); return rax; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = rdx;
    rax = (0u64.wrapping_sub(rax)) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(rcx)) as u64; rax = (0xfffffffffffffffcu64) as u64; rcx = rdx; crate::simd_ops::wu64(r9.wrapping_add(P0), rdx); rcx = (!(rcx)) as u64; rcx = ((rcx).wrapping_shr(5u32)) as u64; rcx = ((rcx).wrapping_shl(2u32)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; crate::simd_ops::lk_add_u64(r9, (rax) as u64); rax = r9; fl_l = ((((rdx as u32)) & (0x1fu32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(P0), rdx); return rax; }
    rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r9.wrapping_add(P0)))) as u64;
    rax = rdx;
    crate::simd_ops::wu64(r9.wrapping_add(P0), rdx);
    rax = ((rax).wrapping_shr(5u32)) as u64;
    rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64;
    rax = ((rax).wrapping_shl(2u32)) as u64;
    fl_l = (crate::simd_ops::lk_add_u64(r9, (rax) as u64)) as u64; fl_r = fl_l;
    rax = r9;
    crate::simd_ops::wu64(r9.wrapping_add(P0), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140208860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140203690;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x20u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402088c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x19; const P1: u64 = 0x10; const P2: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; r8 = rcx; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rcx, rax); rax = rcx; return rax; }
            2 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = rdx; r9 = rdx; bb = 5; continue; }
            5 => { r10 = rax; r11 = rax; rdx = r9; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rax; crate::simd_ops::wu64(r8, rax); rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; r9 = rcx; rdx = rcx; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r8, rax); rax = r8; return rax; }
            9 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rcx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(r8, rcx); bb = 13; continue; }
            13 => { rax = r8; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140208950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1401e9110; const P3: u64 = 0x14183acc8; const P4: u64 = 0x1402032e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; r14 = rdx; crate::simd_ops::wu64(rcx, rax); rsi = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rbx = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (0x3fffffffffffffffu64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx.wrapping_mul(4)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rax); rdi = rax; crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); rcx = ((rbx.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P1), rcx); rcx = rax; rdx = (crate::simd_ops::ru64(r14)) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(P0))) as u64; rbx = ((rbx).wrapping_sub(rdx)) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; rbx = ((fl_l as u64)) as u64; rax = ((rdi.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1402089f8, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140208a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646950;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r13 = r8; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, r15); fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r14); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x38u64))) as u64; fl_r = (0xfu64) as u64; crate::simd_ops::wu64(rdx, r15); rdx = r15.wrapping_add(0x20); if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r12 = (crate::simd_ops::ru64(rdx)) as u64; rbx = r12; fl_l = ((r12).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 5; continue; }
            4 => { r12 = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rbx = rdx; fl_l = ((r12).wrapping_add(rdx)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(r13)) as u64; r14 = rbp; fl_l = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 8; continue; }
            7 => { rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; r14 = r13; fl_l = ((rbp).wrapping_add(r13)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { fl_l = r14; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 10; continue; }
            10 => { rcx = (((crate::simd_ops::ru8(r14)) as i8 as i32) as u32) as u32 as u64; rsi = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi as u32)) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rsi = (((crate::simd_ops::ru8(r14)) as i8 as i32) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi as u32)) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 12; continue; }
            12 => { r14 = ((r14).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = r14; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0x1; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), r15); r15 = (crate::simd_ops::ru64(r15)) as u64; bb = 16; continue; }
            15 => { r15 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 17; continue; }
            17 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 18; continue; }
            18 => { rax = rdx; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140208b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646950;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r13 = r8; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, r15); fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r14); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; fl_r = (0xfu64) as u64; crate::simd_ops::wu64(rdx, r15); rdx = r15.wrapping_add(0x20); if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbp = (crate::simd_ops::ru64(r13)) as u64; rbx = rbp; fl_l = ((rbp).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 5; continue; }
            4 => { rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; rbx = r13; fl_l = ((rbp).wrapping_add(r13)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r12 = (crate::simd_ops::ru64(rdx)) as u64; r14 = r12; fl_l = ((r12).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 8; continue; }
            7 => { r12 = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; r14 = rdx; fl_l = ((r12).wrapping_add(rdx)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { fl_l = r14; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 10; continue; }
            10 => { rcx = (((crate::simd_ops::ru8(r14)) as i8 as i32) as u32) as u32 as u64; rsi = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi as u32)) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rsi = (((crate::simd_ops::ru8(r14)) as i8 as i32) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi as u32)) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 12; continue; }
            12 => { r14 = ((r14).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = r14; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r15 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 16; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0x1; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), r15); r15 = (crate::simd_ops::ru64(r15)) as u64; bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 17; continue; }
            17 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 18; continue; }
            18 => { rax = rdx; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140208c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x14183acc2; const P2: u64 = 0x14183acc8; const P3: u64 = 0x140208de0; const P4: u64 = 0x1402032e0; const P5: u64 = 0x1401e6f50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rsi = rdx; r8 = (0x3fffffffffffffffu64) as u64; r14 = rcx; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = ((rbp).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r14))) as u64; rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; rdx = rcx; rbp = ((((rbp) as i64).wrapping_shr(2u32) as u64)) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbx = r8; bb = 3; continue; }
            3 => { rcx = ((rbx.wrapping_mul(4)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; r15 = rax; rdx = ((rdx).wrapping_sub(rbp)) as u64; fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = 0x0; rcx = ((rax.wrapping_add(rbp.wrapping_mul(4))) as u64) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { r8 = ((rdx.wrapping_mul(4)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            5 => { rbx = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = rsi; bb = 3; continue; }
            7 => { fl_l = rbx; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 8; continue; }
            8 => { bb = 3; continue; }
            9 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = rcx.wrapping_add(0x4); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = r15; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = rbx; r8 = rsi; rdx = r15; rcx = r14; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            13 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140208d8d, rcx, rsp); return rax; }
            14 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140208d93, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140208da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140203210; const P1: u64 = 0x20;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140208da0, funcinfo_rva: 0x35a4360, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rdx) as u64;
    rdi = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x140208db5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x140208dc3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rdi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140208de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x141834d30; const P3: u64 = 0x8; const P4: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rsi = r9; rcx = (crate::simd_ops::ru64(rcx)) as u64; rbp = r8; rdi = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = ((rdi.wrapping_add(rbp.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rbx, rdi); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); rax = ((rdi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140208e67, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140208e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x140203210; const P4: u64 = 0x1402032e0; const P5: u64 = 0x140208f1e;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140208e70, funcinfo_rva: 0x35a4394, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r9) as u64; r15 = (r8) as u64; rbx = (rdx) as u64; r14 = (rcx) as u64; rax = (0x7ffffffffffffffu64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_shl(5u32)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140208eae); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(P1), (rax) as u64); rcx = ((rbx.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(P2), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (r14) as u64); rsi = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140208eea); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = ((rdi).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rdi) as u64); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r14.wrapping_add(P1), (rdi) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 5; continue; }
            5 => { return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140208f19); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140208f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1401e9110; const P3: u64 = 0x14183acc8; const P4: u64 = 0x1402032e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; r14 = rdx; crate::simd_ops::wu64(rcx, rax); rsi = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (0x1fffffffffffffffu64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rax); rdi = rax; crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); rcx = ((rbx.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P1), rcx); rcx = rax; rdx = (crate::simd_ops::ru64(r14)) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(P0))) as u64; rbx = ((rbx).wrapping_sub(rdx)) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rbx = ((fl_l as u64)) as u64; rax = ((rdi.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140208fc8, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140209070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141864e10; const P1: u64 = 0x141834d30;
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x60; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402090f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x143080d40; const P4: u64 = 0x14397d820; const P5: u64 = 0x14020b580; const P6: u64 = 0x14186a310; const P7: u64 = 0x14020b7e0; const P8: u64 = 0x1425371a0; const P9: u64 = 0x142533200; const P10: u64 = 0x14020b1f0; const P11: u64 = 0x142442de0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = r9; rsi = r8; r14 = rcx; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r15 as u32)) as u32); rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0x28; r8 = ((rdx.wrapping_sub(0x18u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = r15; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rbx); rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = P3; crate::simd_ops::wu64(rbx, rdx); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), (0x0u8) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rdi = (crate::simd_ops::ru64(P4)) as u64; r8 = rsp.wrapping_add(0x48); rcx = rsp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r15 as u32)) as u32); crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x4u64), (1) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x4u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); rdx = rsp.wrapping_add(0xa0); rcx = rdi.wrapping_add(0x1f0); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r10 = rdi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rdi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rdi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            10 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            11 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rdx = rcx; r9 = rdi; rcx = r10; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), ((rax as u8)) as u8); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x4u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (((rsi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { crate::simd_ops::wu64(r14, rbx); rax = r14; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140209310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426460e8; const P1: u64 = 0x14186aad0; const P2: u64 = 0x14186a450; const P3: u64 = 0x142646230; const P4: u64 = 0x140208a00;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (((crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as i8 as i64) as u64) as u64; rbx = rdx; rdi = rcx; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { r8 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = rsp.wrapping_add(0x20); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); r8 = P2; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(rdx))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64)))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x34u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r9 = rsp.wrapping_add(0x20); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); r8 = P2; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; r11 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r11 as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; r9 = ((rcx.wrapping_add(rax)) as u32) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r9 = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rcx as u32) } else { (r9 as u32) }) as u32 as u64; rcx = ((rcx).wrapping_shr(32u32)) as u64; r8 = ((r8).wrapping_shr(32u32)) as u64; r8 = (((r8 as u32)).wrapping_sub(crate::simd_ops::ru32(rbx.wrapping_add(0xcu64)))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(crate::simd_ops::ru32(rbx.wrapping_add(0x4u64)))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rdx = ((rcx.wrapping_add(rax)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; r11 = (((r11 as u32)).wrapping_sub(crate::simd_ops::ru32(rbx))) as u32 as u64; r11 = (((r11 as u32)).wrapping_add((r9 as u32))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r11 as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rcx, ((r9 as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))) as u32 as u64; crate::simd_ops::lk_add_u32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((rdx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::lk_add_u32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::lk_add_u32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); }
            10 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1402094b9, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402094c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x143080b18; const P4: u64 = 0x8; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x140209650; const P8: u64 = 0x38; const P9: u64 = 0x40; const P10: u64 = 0xffffffffffffffc8; const P11: u64 = 0x142535070; const P12: u64 = 0x1426456c8; const P13: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = r8; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rax = P3; crate::simd_ops::wu64(rbx, rax); let y_addr: u64 = rsi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(P4); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P6), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(P8), rax); crate::simd_ops::wu16(rbx.wrapping_add(P9), (0x0u16) as u16); crate::simd_ops::wu64(rdi, rbx); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = 0x48; r8 = ((rdx.wrapping_add(P10)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402096e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080b48; const P1: u64 = 0x14020af00; const P2: u64 = 0x14186a310; const P3: u64 = 0x142442de0; const P4: u64 = 0x142645748; const P5: u64 = 0x142646178; const P6: u64 = 0x142646168; const P7: u64 = 0x1401e5070; const P8: u64 = 0x14186a0c0; const P9: u64 = 0x141834d30; const P10: u64 = 0x143406c90; const P11: u64 = 0x1423f0780; const P12: u64 = 0x14136eec0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = ((rdx as u32)) as u32 as u64; rdi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::lk_add_u32(rsi, (1) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rdx = rsp.wrapping_add(0x60); rcx = rdi.wrapping_add(0x1f0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x2f8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = 0xffffffff; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(0x330u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x388u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x338); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = 0x398; rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rdx = P10; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1402097f5, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140209810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x140209bb0; const P3: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rbx as u32)) as u32); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402098b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x160; const P1: u64 = 0x1426455a8; const P2: u64 = 0x170; const P3: u64 = 0x18; const P4: u64 = 0x168; const P5: u64 = 0x140209af0; const P6: u64 = 0x10; const P7: u64 = 0x8; const P8: u64 = 0xc; const P9: u64 = 0x1; const P10: u64 = 0x1401deb40; const P11: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; r14 = rcx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru32(r14.wrapping_add(P2))) as u32 as u64; r15 = (((r15 as u32)).wrapping_add(0x7u32)) as u32 as u64; r15 = ((((r15 as u32)) & (0xfffffff8u32))) as u32 as u64; rdx = ((r15.wrapping_add(P3)) as u64) as u64; rcx = ((r14.wrapping_add(P4)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14.wrapping_add(P4))) as u64; xmm0 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(r15.wrapping_add(rax), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(r15.wrapping_add(rax).wrapping_add(P6), ((xmm1 as u64)) as u64); r14 = ((r14).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(r14.wrapping_add(P7))) as u32 as u64; r8 = (crate::simd_ops::ru32(r14.wrapping_add(P8))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(P9)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(P7))) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r15 as u32)) as u32); crate::simd_ops::lk_add_u32(r14.wrapping_add(P7), (1) as u32); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140209990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 4>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rsi = rcx; rcx = (0x100000001b3u64) as u64; rax = (0xcbf29ce484222325u64) as u64; r9 = (((r9) ^ (rax))) as u64; r15 = r8; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; r14 = rdx; rbp = (crate::simd_ops::ru64(rsi)) as u64; rdi = ((r12 as u32)) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x2u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x3u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x4u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x5u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x6u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x7u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = (0xde5fb9d2630458e9u64) as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((r9) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rcx = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rcx; r8 = ((r8).wrapping_shr(7u32)) as u64; r8 = (((r8) & (rbx))) as u64; fl_l = ((((rcx as u8)) & (0x7fu8))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rax = ((((rcx as u8)) as i8 as i32) as u32) as u32 as u64; xmm2 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpbroadcastb((xmm2 as u8)); bb = 1; continue; }
            1 => { xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rbp))) as u128; xmm0 = crate::simd_ops::vpcmpeqb(xmm1, xmm2); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r11 = (crate::simd_ops::ru64(r15)) as u64; bb = 3; continue; }
            3 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rcx = (((rcx) & (rbx))) as u64; r9 = rcx; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(r10)) as u64; fl_l = (crate::simd_ops::ru64(r9)) as u64; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (((rax as u32)) & (((rax as u32)).wrapping_sub(1))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { xmm0 = crate::simd_ops::vpsignb(xmm1, xmm1); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; r8 = ((r8).wrapping_add(rdi)) as u64; fl_l = (((r8) & (rbx))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 1; continue; }
            7 => { rdx = ((rcx.wrapping_add(rbp)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(r14, rdx); bb = 9; continue; }
            8 => { rax = ((rbx.wrapping_add(rbp)) as u64) as u64; crate::simd_ops::wu64(r14, rax); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = r14; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140209af0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x140209f30; const P3: u64 = 0x14183acc8; const P4: u64 = 0x1401de830; const P5: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = rcx; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = rdx; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rbx }) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = rax; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rbp); crate::simd_ops::wu32(rsi.wrapping_add(P1), ((rbx as u32)) as u32); r8 = rdi; fl_l = r8; fl_r = r14; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = ((r14.wrapping_add(rbp)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rdi as u32)) as u32); bb = 9; continue; }
            3 => { fl_l = rdi; fl_r = r14; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { r8 = rdi; fl_l = r8; fl_r = r14; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rdi as u32)) as u32); bb = 9; continue; }
            7 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdi as u32)) as u32); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140209ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x142537280; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x18; const P7: u64 = 0x143080bc0; const P8: u64 = 0x8; const P9: u64 = 0x1401f3d90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = r8; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { rbx = rdx; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rax = P7; crate::simd_ops::wu64(rbx, rax); rax = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P8), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); crate::simd_ops::wu16(rbx.wrapping_add(P6), (0x0u16) as u16); crate::simd_ops::wu64(rdi, rbx); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020a180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434286b0; const P1: u64 = 0x143428770;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; r8 = rcx; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rcx = rdx; rax = rdx.wrapping_add(0x1); rax = ((rax).wrapping_add(r9)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm2 = (crate::simd_ops::ru128(P0)) as u128; xmm3 = (crate::simd_ops::ru128(P1)) as u128; bb = 2; continue; }
            2 => { xmm0 = crate::simd_ops::vpshufb(xmm2, crate::simd_ops::ru128(rcx)); xmm1 = (xmm0) | (xmm3); crate::simd_ops::wu128(rcx, (xmm1) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(r9.wrapping_add(rdx), (0xffu8) as u8); r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; r11 = ((rbp as u32)) as u32 as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); r10 = (0xcbf29ce484222325u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rsi = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r12); r12 = (0x100000001b3u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), r13); r13 = (0xde5fb9d2630458e9u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), r14); r14 = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsp, r15); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(r11))) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; r15 = ((rdi.wrapping_add(r11)) as u64) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rdx = (((rdx) ^ (r10))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x5u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x6u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x7u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = r13; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rbx = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = rbp; rcx = rbx; rcx = ((rcx).wrapping_shr(7u32)) as u64; rcx = (((rcx) & (r9))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rdi.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r9))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rdi.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { r10 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rdx = rbx; rax = r11; r10 = ((r10).wrapping_add(rcx)) as u64; rdx = ((rdx).wrapping_shr(7u32)) as u64; rdx = (((rdx) & (r9))) as u64; r10 = (((r10) & (r9))) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; rcx = r10; rcx = ((rcx).wrapping_sub(rdx)) as u64; rbx = (rbx & !0xffu64) | ((((((rbx as u8)) & (0x7fu8))) as u8 as u64)); rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) & (r9))) as u64; fl_l = rcx; fl_r = (0xfffffffffffffff0u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu8(r15, ((rbx as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rcx = rax; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (rsi))) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((rbx as u8)) as u8); bb = 14; continue; }
            11 => { rax = ((rdi.wrapping_add(r10)) as u64) as u64; rdx = r10.wrapping_sub(0x10); r10 = ((r10).wrapping_add(r10)) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rcx = rax; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (rdx))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((rbx as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r14))) as u128; crate::simd_ops::wu128(rax.wrapping_add(r10.wrapping_mul(8)), (xmm0) as u128); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu8(r11.wrapping_add(rax), (0x80u8) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rcx = rax; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (rsi))) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), (0x80u8) as u8); bb = 14; continue; }
            13 => { rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (rdx))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r11 = ((r11).wrapping_sub(1)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rsi = ((rsi).wrapping_sub(1)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((rbx as u8)) as u8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(r14))) as u128; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r10.wrapping_mul(8)))) as u128; crate::simd_ops::wu128(rax.wrapping_add(r14), (xmm0) as u128); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = r14; fl_r = (0x10u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu128(rax.wrapping_add(r10.wrapping_mul(8)), (xmm1) as u128); bb = 14; continue; }
            14 => { r10 = (0xcbf29ce484222325u64) as u64; bb = 15; continue; }
            15 => { r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; r11 = ((r11).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(1)) as u64; r14 = ((r14).wrapping_add(0x10u64)) as u64; fl_l = r11; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 16; continue; }
            16 => { r15 = (crate::simd_ops::ru64(rsp)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 17; continue; }
            17 => { rax = r9; rax = ((rax).wrapping_shr(3u32)) as u64; r9 = ((r9).wrapping_sub(rax)) as u64; r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8.wrapping_add(0x10u64)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x28u64), r9); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020a430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14183acc2; const P5: u64 = 0x1425371a0; const P6: u64 = 0x142533200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; r14 = rdx.wrapping_add(0x18); rsi = (crate::simd_ops::ru64(rcx)) as u64; rdi = rdx; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r14 = (((r14) & (0xfffffffffffffff8u64))) as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rax)) as u64; r12 = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(4u32)) as u64; r10 = 0x6348; rdx = ((rdx).wrapping_add(r14)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            4 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = r12; bb = 12; continue; }
            6 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 12; continue; }
            10 => { r8 = 0x10; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = rax; bb = 12; continue; }
            12 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r12; bb = 14; continue; }
            14 => { rax = ((r14.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(rbx, rcx); r8 = rdi.wrapping_add(0x10); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rdx = 0xffffff80; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdi; r11 = r12; crate::simd_ops::wu8(rdi.wrapping_add(rax), (0xffu8) as u8); rax = rdi; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rcx); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r13); rdi = rbp.wrapping_add(0x2); r13 = (0xde5fb9d2630458e9u64) as u64; rax = (0xcbf29ce484222325u64) as u64; r14 = (0x100000001b3u64) as u64; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(r11))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 21; continue; } bb = 17; continue; }
            17 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_sub(0x2u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_sub(0x1u64))) as u32) as u32 as u64; r10 = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x3u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x5u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = r13; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rbp = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = r12; rcx = rbp; rcx = ((rcx).wrapping_shr(7u32)) as u64; rcx = (((rcx) & (r9))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r10.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r9))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r10.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { r8 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rbp = (rbp & !0xffu64) | ((((((rbp as u8)) & (0x7fu8))) as u8 as u64)); r8 = ((r8).wrapping_add(rcx)) as u64; r8 = (((r8) & (r9))) as u64; crate::simd_ops::wu8(r10.wrapping_add(r8), ((rbp as u8)) as u8); rcx = r8.wrapping_sub(0x10); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = (((rcx) & (rax))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((r8).wrapping_add(r8)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((rbp as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rdi.wrapping_sub(0x2u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(r8.wrapping_mul(8)), (xmm0) as u128); rax = (0xcbf29ce484222325u64) as u64; bb = 21; continue; }
            21 => { r11 = ((r11).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = r11; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 22; continue; }
            22 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 23; continue; }
            23 => { rcx = rsi.wrapping_sub(0x1); rax = rsi; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rsi); if (((fl_l as u32)) != 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            27 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 29; continue; }
            28 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            29 => { r9 = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020a740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14186a0c0; const P1: u64 = 0x141834d30;
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x330; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020a780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080c78; const P1: u64 = 0x14020a7d0; const P2: u64 = 0x1423f0c20; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rdi = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x128u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x140; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020a7d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbp = 0; fl_l = 0; fl_r = 0; rbx = ((rbp as u32)) as u32 as u64; rsi = rcx.wrapping_add(0x10); r14 = rcx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rax = ((rax).wrapping_add(rbx)) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r15 = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r15)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r15; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = r15; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rsi = rdi.wrapping_add(0x10); bb = 6; continue; }
            6 => { crate::simd_ops::wu32(r14, ((rbp as u32)) as u32); rbx = rdi.wrapping_add(0xc); crate::simd_ops::wu32(rbx, ((rbp as u32)) as u32); crate::simd_ops::wu32(rsi, ((rbp as u32)) as u32); r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rcx = r9.wrapping_sub(0x1); rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rcx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = rdx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = rdi.wrapping_add(0x8); rbx = rdi.wrapping_add(0xc); rsi = rdi.wrapping_add(0x10); bb = 14; continue; }
            11 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu32(r14, ((rbp as u32)) as u32); crate::simd_ops::wu32(rbx, ((rbp as u32)) as u32); crate::simd_ops::wu32(rsi, ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020aaa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x160; const P1: u64 = 0x1426455a8; const P2: u64 = 0x170; const P3: u64 = 0x20; const P4: u64 = 0x168; const P5: u64 = 0x140209af0; const P6: u64 = 0x8; const P7: u64 = 0xc; const P8: u64 = 0x1; const P9: u64 = 0x1401deb40; const P10: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; r14 = rcx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru32(r14.wrapping_add(P2))) as u32 as u64; r15 = (((r15 as u32)).wrapping_add(0x7u32)) as u32 as u64; r15 = ((((r15 as u32)) & (0xfffffff8u32))) as u32 as u64; rdx = ((r15.wrapping_add(P3)) as u64) as u64; rcx = ((r14.wrapping_add(P4)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14.wrapping_add(P4))) as u64; let y_addr: u64 = rbp; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r15.wrapping_add(rax); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); r14 = ((r14).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(r14.wrapping_add(P6))) as u32 as u64; r8 = (crate::simd_ops::ru32(r14.wrapping_add(P7))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(P8)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(P6))) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r15 as u32)) as u32); crate::simd_ops::lk_add_u32(r14.wrapping_add(P6), (1) as u32); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020ab80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x160; const P1: u64 = 0x1426455a8; const P2: u64 = 0x170; const P3: u64 = 0x38; const P4: u64 = 0x168; const P5: u64 = 0x140209af0; const P6: u64 = 0x20; const P7: u64 = 0x30; const P8: u64 = 0x8; const P9: u64 = 0xc; const P10: u64 = 0x1; const P11: u64 = 0x1401deb40; const P12: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; r14 = rcx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru32(r14.wrapping_add(P2))) as u32 as u64; r15 = (((r15 as u32)).wrapping_add(0x7u32)) as u32 as u64; r15 = ((((r15 as u32)) & (0xfffffff8u32))) as u32 as u64; rdx = ((r15.wrapping_add(P3)) as u64) as u64; rcx = ((r14.wrapping_add(P4)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14.wrapping_add(P4))) as u64; let y_addr: u64 = rbp; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r15.wrapping_add(rax); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r15.wrapping_add(rax).wrapping_add(P6), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(r15.wrapping_add(rax).wrapping_add(P7), ((xmm0 as u64)) as u64); r14 = ((r14).wrapping_add(0x178u64)) as u64; rdx = (crate::simd_ops::ru32(r14.wrapping_add(P8))) as u32 as u64; r8 = (crate::simd_ops::ru32(r14.wrapping_add(P9))) as u32 as u64; rcx = ((r8.wrapping_mul(4)) as u64) as u64; rax = ((rdx.wrapping_mul(4)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(P10)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), ((r15 as u32)) as u32); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x8u64), (1) as u32); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020acc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x142442de0; const P5: u64 = 0x1425371a0; const P6: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 29; continue; } bb = 1; continue; }
            1 => { r12 = ((rax as u32)) as u32 as u64; r12 = (((r12 as u32)).wrapping_shr(1u32)) as u32 as u64; r12 = (((r12 as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = 0x8; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r12 = (if (((fl_l as u32)) < ((fl_r as u32))) { (rax as u32) } else { (r12 as u32) }) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r12 = (if (((fl_l as u32)) < ((fl_r as u32))) { (rdx as u32) } else { (r12 as u32) }) as u32 as u64; r13 = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r12 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            5 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 12; continue; }
            6 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = rsi; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 12; continue; }
            10 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rsi = rax; bb = 12; continue; }
            12 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rdi = 0; fl_l = 0; fl_r = 0; r14 = rbx.wrapping_add(0x10); rbp = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 20; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; r9 = ((rdi as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; rax = ((rax).wrapping_add(r9)) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rdx.wrapping_mul(8)), (0x0u64) as u64); crate::simd_ops::wu64(rsi.wrapping_add(rdi.wrapping_mul(8)), rax); r8 = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; rax = ((rax).wrapping_add(r9)) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r15 = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r15)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r15; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = r15; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 19; continue; }
            19 => { r14 = rbx.wrapping_add(0x10); bb = 20; continue; }
            20 => { fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 21; continue; }
            21 => { rcx = r13.wrapping_sub(0x1); rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r13; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rcx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r13, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r13); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { rcx = rdx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbp = rbx.wrapping_add(0x8); r14 = rbx.wrapping_add(0x10); bb = 28; continue; }
            25 => { r9 = r13; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rbp = rbx.wrapping_add(0x8); r14 = rbx.wrapping_add(0x10); bb = 28; continue; }
            27 => { r9 = r13; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rbx, rsi); crate::simd_ops::wu32(rbp, (0x0u32) as u32); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((r12 as u32)) as u32); bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020af00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x142537280; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x18; const P7: u64 = 0x143080c90; const P8: u64 = 0x1401f3d90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { rbx = rdx; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rax = P7; crate::simd_ops::wu64(rbx, rax); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); crate::simd_ops::wu16(rbx.wrapping_add(P6), (0x0u16) as u16); crate::simd_ops::wu64(rdi, rbx); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020b0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080d40; const P1: u64 = 0x14397d820; const P2: u64 = 0x14020b640; const P3: u64 = 0x14186a310; const P4: u64 = 0x1401e5660; const P5: u64 = 0x1401e5470; const P6: u64 = 0x142442de0; const P7: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = ((rdx as u32)) as u32 as u64; rsi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); rbx = (crate::simd_ops::ru64(P1)) as u64; r8 = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); crate::simd_ops::lk_add_u32(rdx, (1) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rdx = rsp.wrapping_add(0x70); rcx = rbx.wrapping_add(0x1f0); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = 0x28; rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020b1f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x20; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x1426455a8; const P6: u64 = 0x1426455b8; const P7: u64 = 0x142645598; const P8: u64 = 0x14020b2d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; r14 = rcx; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(P2), rcx); crate::simd_ops::wu64(rdi.wrapping_add(P3), rcx); crate::simd_ops::wu64(rdi.wrapping_add(P4), rcx); crate::simd_ops::wu8(rdi.wrapping_add(P1), (0x1u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rcx as u32)) as u32); bb = 3; continue; }
            3 => { rcx = rax; rcx = (((rcx) | (0x2u64))) as u64; fl_l = rax; let cx_src: u64 = rcx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rdi.wrapping_add(P0), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 10; continue; }
            6 => { rcx = ((rdi.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rdi.wrapping_add(P4)) as u64) as u64; r8 = 0xffffffff; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 10; continue; }
            10 => { rbx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rcx = r14; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (((rbx as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020b340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240ab10; const P2: u64 = 0x14241c190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020b700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x142645598; const P2: u64 = 0x1426455c0; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x9u64), (0x1u8) as u8); r14 = (crate::simd_ops::ru64(rcx)) as u64; rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x41u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64; crate::simd_ops::wu16(r14, ((rax as u16)) as u16); rdi = 0x1; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(r14.wrapping_add(0x8u64), rdi); rdi = (tmp_xchg) as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(1u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = r14.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x20u64), (1) as u32); rcx = r14.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = r14.wrapping_add(0x10); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(r14.wrapping_add(0x4u64), (1) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = r14; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(r14.wrapping_add(0x4u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020b850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffe0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x1; const P8: u64 = 0x4; const P9: u64 = 0x8; const P10: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x30; rcx = rax; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rcx.wrapping_add(P7), (0x0u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P8), (0x0u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P9), (0x0u64) as u64); crate::simd_ops::wu8(rcx.wrapping_add(P10), (0x0u8) as u8); crate::simd_ops::lk_add_u32(rcx.wrapping_add(P8), (1) as u32); rax = rcx; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020b8e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x14240a730;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = r8; rsi = rdx; rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8); fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::lk_and_u32(rdi, (0xff000002u32) as u32); crate::simd_ops::lk_or_u32(rdi, (0x2u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P0), (0x2u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rbp as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P2), (0x0u8) as u8); rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020b9a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1418651c0; const P4: u64 = 0xffffffffffffffb0; const P5: u64 = 0x142535070; const P6: u64 = 0x1426456c8; const P7: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14020b9a0, funcinfo_rva: 0x35a48e0, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (r9) as u64; r14 = (r8) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); r9 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ba53); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x14020ba53, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x14020b9a0) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = (0x60u32) as u32 as u64; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ba7e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ba8f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ba9d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020bab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14186b980; const P1: u64 = 0x141834d30;
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x58; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020baf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080b48; const P1: u64 = 0x14020af00; const P2: u64 = 0x14186a310; const P3: u64 = 0x142442de0; const P4: u64 = 0x142645748; const P5: u64 = 0x142646178; const P6: u64 = 0x142646168; const P7: u64 = 0x1401e5070; const P8: u64 = 0x14186a0c0; const P9: u64 = 0x143406c90; const P10: u64 = 0x1423f0780; const P11: u64 = 0x14136eec0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = P0; crate::simd_ops::wu64(rdi, rax); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::lk_add_u32(rsi, (1) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rdx = rsp.wrapping_add(0x48); rcx = rdi.wrapping_add(0x1f0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x2f8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = 0xffffffff; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(0x330u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x388u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x338); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rdx = P9; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14020bc05, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020bc10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3a0; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x141867bb0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14020bc10, funcinfo_rva: 0x359b5c0, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rbx) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020bc6a); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x14020bc6a, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x14020bc10) }; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = (0x398u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020bc8b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020bc98); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020bca6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020be00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rdx, (0xffffffffu32) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020be40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rdx, (0x3e800000u32) as u32);
    crate::simd_ops::wu32(r8, (0x3f000000u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020be50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143427050; const P1: u64 = 0x14342668c; const P2: u64 = 0xc; const P3: u64 = 0x14; const P4: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm3 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm3);
    crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64);
    xmm0 = crate::simd_ops::vunpcklps(xmm4, xmm4);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((xmm4 as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((xmm3 as u32)) as u32);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020bf20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x4;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020bfd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x8; const P4: u64 = 0x14020c080; const P5: u64 = 0xc; const P6: u64 = 0x140211010;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; crate::simd_ops::wu8(rax, (0xdu8) as u8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rcx); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { r9 = 0x10000; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x38); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            4 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx, r8); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rcx); rcx = 0xffff; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = ((rcx).wrapping_sub(rdi)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rcx); rcx = ((rax.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rcx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as i64).wrapping_mul((0x160u64) as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c1f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1420a6b70;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    r8 = (((rdx as u16)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    r9 = 0x1;
    rdx = ((crate::simd_ops::ru16(rbx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14020c350; const P2: u64 = 0x8; const P3: u64 = 0x1420a6b70;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    r8 = rdx;
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8);
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    r9 = rsp.wrapping_add(0x30);
    rbx = rcx;
    rcx = rsp.wrapping_add(0x50);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    r9 = 0x2;
    r8 = ((crate::simd_ops::ru16(rsp.wrapping_add(0x50u64))) as u32) as u32 as u64;
    rdx = ((crate::simd_ops::ru16(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14020c350; const P2: u64 = 0x8; const P3: u64 = 0x1420a6b70;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    r8 = rdx;
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8);
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    r9 = rsp.wrapping_add(0x30);
    rbx = rcx;
    rcx = rsp.wrapping_add(0x50);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    r9 = 0x1;
    r8 = ((crate::simd_ops::ru16(rbx.wrapping_add(P2))) as u32) as u32 as u64;
    rdx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c2e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x2; const P2: u64 = 0x3;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64;
    r9 = (0x100000001b3u64) as u64;
    rax = (0xcbf29ce484222325u64) as u64;
    rdx = rcx;
    r8 = (((r8) ^ (rax))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rax = (((rax) ^ (r8))) as u64;
    rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rax = (((rax) ^ (rcx))) as u64;
    rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rax = (((rax) ^ (rcx))) as u64;
    rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1434268d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm1 = (xmm0) as u128; xmm0 = 0u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 2; continue; } bb = 1; continue; }
            1 => { if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 3; continue; }
            3 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020e980; const P1: u64 = 0x14020afa0; const P2: u64 = 0x1420a6b70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r12 = r9; r14 = r8; rsi = rdx.wrapping_add(0x8); r15 = rcx; rcx = rdx.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(r8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rbp = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; r13 = 0xffff; fl_l = ((rbp as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(r12)) as u64; xmm0 = (crate::simd_ops::ru128(r14)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r14 = rcx.wrapping_add(0x10); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbp = rdx; rbp = ((((rbp) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbp; rax = ((rax).wrapping_shr(63u32)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rbp.wrapping_add(0x1); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r14))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0xeu64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x20u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x18u64), rbp); crate::simd_ops::wu16(rdi, ((rbp as u16)) as u16); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x90u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x2u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r9 = 0x2; r8 = ((crate::simd_ops::ru16(r12.wrapping_add(0x8u64))) as u32) as u32 as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rcx); crate::simd_ops::wu8(rdi.wrapping_add(0x2u64), (0x1u8) as u8); bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; crate::simd_ops::wu16(r15, ((rax as u16)) as u16); rax = r15; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c4d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu64(rcx, rdx);
    r9 = 0x811c9dc5;
    r8 = (0xffffffffffffffffu64) as u64;
    loop { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    r8 = ((r8).wrapping_add(rdx)) as u64;
    fl_l = rdx; fl_r = r8;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r9 as u32)) as u32); return rax; }
    loop { rax = (((crate::simd_ops::ru8(rdx)) as i8 as i32) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rax = ((((rax as u32)) ^ ((r9 as u32)))) as u32 as u64; r9 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rdx; fl_r = r8; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r9 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14020e9f0; const P3: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rbx = rcx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64;
    rcx = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64;
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); } else { rax = ((r8.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); }
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rax = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdx = 0xc;
    r9 = 0x6348;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    r8 = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; }
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rax, rcx);
    crate::simd_ops::wu32(rax.wrapping_add(P4), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438798e0; const P1: u64 = 0x14243b0b0; const P2: u64 = 0x1426d4580;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rbx = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rdi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x4u64))) as u32) as u32 as u64; rsi = P2; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x338u64))) as u32 as u64; r8 = (!((r8 as u32))) as u32 as u64; rcx = (((r8 as u8)) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; r8 = (((r8 as u32)).wrapping_shr(8u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((((r8 as u32)) ^ (crate::simd_ops::ru32(rsi.wrapping_add(rcx.wrapping_mul(4)))))) as u32 as u64; r8 = (!((r8 as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x338u64), ((r8 as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x320u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x338u64))) as u32 as u64; r8 = 0x4; rdx = (!((rdx as u32))) as u32 as u64; r9 = ((r8 as u32)) as u32 as u64; r10 = rdi; bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; r10 = r10.wrapping_add(0x1); rcx = (((rdx as u8)) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rdx = (((rdx as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((((rdx as u32)) ^ (crate::simd_ops::ru32(rsi.wrapping_add(rcx.wrapping_mul(4)))))) as u32 as u64; fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = ((rdx as u32)) as u32 as u64; r9 = rdi.wrapping_add(0x8); rax = (!((rax as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x338u64), ((rax as u32)) as u32); bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r9 = r9.wrapping_add(0x1); rcx = (((rdx as u8)) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rdx = (((rdx as u32)).wrapping_shr(8u32)) as u32 as u64; rdx = ((((rdx as u32)) ^ (crate::simd_ops::ru32(rsi.wrapping_add(rcx.wrapping_mul(4)))))) as u32 as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdx = (!((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x338u64), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x328u64), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c8c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438799b8; const P1: u64 = 0x1438798e0; const P2: u64 = 0x1401e9fa0; const P3: u64 = 0x1401e9e70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rsi = rcx; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; rdx = rsi.wrapping_add(0x4); if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rbx.wrapping_add(0x369); bb = 4; continue; }
            2 => { rdi = rbx.wrapping_add(0x369); fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rdx, (0x0u8) as u8); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x320u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; bb = 9; continue; }
            9 => { if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsi, (0x0u32) as u32); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x2; rdx = rsi.wrapping_add(0x8); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rdx, (0x0u32) as u32); bb = 17; continue; }
            17 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020c9d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438798e0; const P1: u64 = 0x14243b0b0; const P2: u64 = 0x338; const P3: u64 = 0x4; const P4: u64 = 0x80; const P5: u64 = 0x318; const P6: u64 = 0x328; const P7: u64 = 0x320; const P8: u64 = 0x8; const P9: u64 = 0x1; const P10: u64 = 0x38; const P11: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rbx = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rdi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r8.wrapping_add(P9)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = rdi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x8); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x328u64), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020caa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438799b8; const P1: u64 = 0x1438798e0; const P2: u64 = 0x1401e9b60; const P3: u64 = 0x1401e9a20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; rsi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; rdi = rbx.wrapping_add(0x369); if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x120u64))) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x4u64), ((rcx as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rdx.wrapping_add(0x120u64), (1) as u64)) as u64; bb = 4; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rsi.wrapping_add(0x4u64), (0x0u8) as u8); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x320u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { r8 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x120u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsi, ((rcx as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rdx.wrapping_add(0x120u64), (0x4u64) as u64)) as u64; fl_r = fl_l; bb = 12; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rsi, ((r8 as u32)) as u32); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x120u64))) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = rcx.wrapping_add(0x4); crate::simd_ops::wu64(rdx.wrapping_add(0x120u64), rax); bb = 16; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 16; continue; }
            16 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020cf60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438798e0; const P1: u64 = 0x14243b0b0; const P2: u64 = 0x338; const P3: u64 = 0x4; const P4: u64 = 0x318; const P5: u64 = 0x328; const P6: u64 = 0x320; const P7: u64 = 0x8; const P8: u64 = 0x1; const P9: u64 = 0x38; const P10: u64 = 0xfffffffffffffff8; const P11: u64 = 0xffffffffffffffff;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rbx = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rdi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rdi.wrapping_add(P3), (0x0u8) as u8); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r8.wrapping_add(P8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi, ((rcx as u32)) as u32); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rcx as u32)) as u32); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P10))) as u32 as u64; rax = ((rdx.wrapping_add(P11)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x328u64), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020d020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438799b8; const P1: u64 = 0x1438798e0; const P2: u64 = 0x1401e9970; const P3: u64 = 0x338; const P4: u64 = 0x4; const P5: u64 = 0x120; const P6: u64 = 0x318; const P7: u64 = 0x328; const P8: u64 = 0x320; const P9: u64 = 0x8; const P10: u64 = 0x1; const P11: u64 = 0x38; const P12: u64 = 0x1401e9860;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; rdi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P9))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r8.wrapping_add(P10)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xe0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x8); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020d0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438798e0; const P1: u64 = 0x14243b0b0; const P2: u64 = 0x338; const P3: u64 = 0x4; const P4: u64 = 0x120; const P5: u64 = 0x318; const P6: u64 = 0x328; const P7: u64 = 0x320; const P8: u64 = 0x8; const P9: u64 = 0x1; const P10: u64 = 0x38; const P11: u64 = 0xe0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rbx = rdx; rdx = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rdi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r8.wrapping_add(P9)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = rdi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x8); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x328u64), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020d1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438799b8; const P1: u64 = 0x1438798e0; const P2: u64 = 0x1401e9770; const P3: u64 = 0x338; const P4: u64 = 0x4; const P5: u64 = 0x120; const P6: u64 = 0x318; const P7: u64 = 0x328; const P8: u64 = 0x320; const P9: u64 = 0x8; const P10: u64 = 0x1; const P11: u64 = 0x38; const P12: u64 = 0x1401e9590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = rcx; rdi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P5))) as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); crate::simd_ops::lk_add_u64(r8.wrapping_add(P5), (1) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P9))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r8.wrapping_add(P10)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x120u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::lk_add_u64(rdx.wrapping_add(0x120u64), (0x4u64) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x120u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = rcx.wrapping_add(0x4); rcx = rbx; crate::simd_ops::wu64(rdx.wrapping_add(0x120u64), rax); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020d290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438798e0; const P1: u64 = 0x14307aa58; const P2: u64 = 0x1413c3940; const P3: u64 = 0x14307a9d0; const P4: u64 = 0x14307a9e0; const P5: u64 = 0x1423fe030; const P6: u64 = 0x142481290; const P7: u64 = 0x4; const P8: u64 = 0x143080f30; const P9: u64 = 0x1; const P10: u64 = 0x14020f0f0; const P11: u64 = 0x143080f20; const P12: u64 = 0x14020f120; const P13: u64 = 0x143080f10; const P14: u64 = 0x14020f240;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (crate::simd_ops::ru64(rdx)) as u64; rbx = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x2u32) as u32); rsi = rcx; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(P0)) as u64; r8 = rsp.wrapping_add(0x50); rdx = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbx)) as u64; rax = P3; bb = 3; continue; }
            2 => { rdx = (crate::simd_ops::ru64(P0)) as u64; rcx = P4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { r8 = rsp.wrapping_add(0x60); rdx = rax; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), (0x0u8) as u8); rax = rsp.wrapping_add(0x60); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = ((rsi.wrapping_add(P7)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); r9 = P8; r8 = ((rdi.wrapping_add(P9)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsp.wrapping_add(0x60); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r9 = P11; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); r8 = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsp.wrapping_add(0x60); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rsi.wrapping_add(0x8); xmm0 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); r9 = P13; r8 = ((rdi.wrapping_add(0x2u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020d3b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020f310; const P1: u64 = 0x88; const P2: u64 = 0x8; const P3: u64 = 0x14020bfd0; const P4: u64 = 0xc;
    let (mut rax, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, r14);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rsi = ((r8 as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x68u64)) as u64;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rdx = 0x1;
    r14 = rax;
    crate::simd_ops::wu64(rax, rbx);
    crate::simd_ops::wu32(rax.wrapping_add(P2), ((rsi as u32)) as u32);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu8(rax, (0xcu8) as u8);
    crate::simd_ops::wu64(r14.wrapping_add(P4), (0xffffffffffffffffu64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    r14 = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020d420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; r9 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0xffffffffffffffffu64) as u64); fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rcx = r8; rcx = ((rcx).wrapping_shr(32u32)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(r9)) as u64; rcx = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0xffffffffffffffffu64) as u64); rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            7 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            8 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { r8 = ((r8).wrapping_shr(32u32)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            13 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020d730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14020c520; const P1: u64 = 0xb0; const P2: u64 = 0x14020bfd0; const P3: u64 = 0x4;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x78u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rdx = 0x1;
    rsi = rax;
    crate::simd_ops::wu32(rax, ((rbx as u32)) as u32);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rax, (0xcu8) as u8);
    crate::simd_ops::wu64(rsi.wrapping_add(P3), (0xffffffffffffffffu64) as u64);
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020e980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140213f20; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x12;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rdx;
    r8 = rdx;
    rdx = rsp.wrapping_add(0x20);
    rbx = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rdi)) as u128; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2), ((rdx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2), (0xffffu16) as u16); crate::simd_ops::wu8(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3), ((rdx as u8)) as u8); }
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    rax = ((rax).wrapping_add(0x10u64)) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020e9f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1402111b0; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbp = 0x3fffffff; rdi = (((rdi) & (rbp))) as u64; r14 = rdx; rax = rdi; rsi = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = (0x8u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = 0x8; bb = 3; continue; }
            2 => { fl_l = rdi; fl_r = rbp; rdi = (if (((fl_l as u64)) > ((fl_r as u64))) { rbp } else { rdi }) as u64; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rsi))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rdx = ((rdi as u32)) as u32 as u64; rbx = ((((rbx) as i64).wrapping_shr(1u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; r15 = rax; rbx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((r8.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = r15; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_add(0xcu64)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); rdx = rdx.wrapping_add(0xc); fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { bb = 14; continue; }
            8 => { rcx = r15; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_add(0xcu64)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = rcx.wrapping_add(0xc); fl_l = r8; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = rbx.wrapping_add(0xc); rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; crate::simd_ops::wu64(r8, ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx).wrapping_add(0xcu64)) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rcx as u32)) as u32); r8 = r8.wrapping_add(0xc); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x40000000; r9 = (crate::simd_ops::ru64(rsi)) as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 15; continue; }
            15 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 16; continue; }
            16 => { rcx = r9.wrapping_sub(0x1); rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); if (((fl_l as u32)) != 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            20 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 22; continue; }
            21 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 22; continue; }
            22 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rdi = ((((rdi as u32)) & ((rbp as u32)))) as u32 as u64; crate::simd_ops::wu64(rsi, r15); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rdi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020ebe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142481040; const P1: u64 = 0x14307cb10; const P2: u64 = 0x1426cffff; const P3: u64 = 0x1401f2430; const P4: u64 = 0x1401eb150; const P5: u64 = 0x14240ab10; const P6: u64 = 0x142409b50; const P7: u64 = 0x1401f22d0; const P8: u64 = 0x14240a730;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 45>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14020ebe0, funcinfo_rva: 0x35a50f8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x168, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x148u64)) as u64; rbx = (r9) as u64; r14 = (rdx) as u64; rsi = (rcx) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a0u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x148u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rdi = ((rcx.wrapping_add(0x30u64)) as u64) as u64; r8 = (r14) as u64; rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ec23); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { r9 = (rbx) as u64; r8 = ((P1) as u64) as u64; rdx = ((crate::simd_ops::ru8(P2)) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ec46); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsi.wrapping_add(0x20u64)) as u64) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ec52); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ec5c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a0u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(r14, ((rcx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x10000010u64) as u64); rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rsi as u8)) as u8); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rbx as u8)) as u8); bb = 8; continue; }
            5 => { r9 = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ecb5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ecec); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ed17); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ed24); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x148u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020ed40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x54; const P1: u64 = 0x20; const P2: u64 = 0x40; const P3: u64 = 0x140212ae0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    r11 = r9;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64;
    r9 = rdx;
    r10 = rcx;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
    rcx = ((rcx).wrapping_add(0x30u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    rdx = ((r10.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64;
    r8 = r11;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020ed90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x54; const P1: u64 = 0x40; const P2: u64 = 0x140212c20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    r11 = r9;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64;
    r10 = rcx;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx, ((rcx as u8)) as u8); rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
    rcx = ((rcx).wrapping_add(0x30u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    r9 = rdx;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64;
    r8 = r11;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020ede0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142481040; const P1: u64 = 0x142409b50; const P2: u64 = 0x1401f22d0; const P3: u64 = 0x14240a730; const P4: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14020ede0, funcinfo_rva: 0x35a5124, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xd8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbx = (r9) as u64; rsi = (rdx) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rdi = ((rcx.wrapping_add(0x30u64)) as u64) as u64; r8 = (rsi) as u64; rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ee1c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsi, ((rcx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x10000010u64) as u64); rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rsi as u8)) as u8); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rbx as u8)) as u8); bb = 8; continue; }
            5 => { r9 = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020ee81); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020eeb8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020eee3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020eef0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020ef00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x54; const P1: u64 = 0x40; const P2: u64 = 0x140212d30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    r11 = r9;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64;
    r10 = rcx;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
    rcx = ((rcx).wrapping_add(0x30u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    r9 = rdx;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64;
    r8 = r11;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020ef50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x54; const P1: u64 = 0x30; const P2: u64 = 0x40; const P3: u64 = 0x140212e40;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    r10 = r9;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx, ((rcx as u8)) as u8); rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
    rax = ((rcx.wrapping_add(P1)) as u64) as u64;
    r9 = rdx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    r8 = r10;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020ef90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142481040; const P1: u64 = 0x142409b50; const P2: u64 = 0x1401f22d0; const P3: u64 = 0x14240a730; const P4: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14020ef90, funcinfo_rva: 0x35a514c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xd8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xc8u64)) as u64; rbx = (r9) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0xc8u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rdi = ((rcx.wrapping_add(0x30u64)) as u64) as u64; r8 = (rdx) as u64; rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020efc7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rbx as u8)) as u8); bb = 8; continue; }
            5 => { r9 = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020f021); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020f058); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020f087); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020f094); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0xc8u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020f0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x54; const P1: u64 = 0x30; const P2: u64 = 0x40; const P3: u64 = 0x140212f30;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    r10 = r9;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
    rax = ((rcx.wrapping_add(P1)) as u64) as u64;
    r9 = rdx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    r8 = r10;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020f0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x142480f30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = rdx; rdx = r9; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            3 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020f120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1413c3800; const P1: u64 = 0x14183acbc; const P2: u64 = 0x1413c3460;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14020f120, funcinfo_rva: 0x35a5178, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r9) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 2; continue; }
            2 => { r14 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rbp = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); rcx = (((rcx as u32)).wrapping_shr(26u32)) as u32 as u64; rcx = (rcx & !0xffffu64) | (((!((rcx as u16))) as u16 as u64)); rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) & (0x20u16))) as u16 as u64)); rax = (0x1d6u32) as u32 as u64; rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) | ((rax as u16)))) as u16 as u64)); crate::simd_ops::wu16(rsp.wrapping_add(0x3eu64), ((rcx as u16)) as u16); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rax = (rsi) as u64; rbx = ((rsi as u32)) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(r9)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(1)) as u64; rbx = (rax) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = ((rsi as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rbx = (((rbx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0xdu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (0x1c05u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); rax = (0xdu32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rax = (rax & !0xffu64) | ((((fl_l as u8).wrapping_sub(fl_r as u8)) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x2du64), ((rax as u8)) as u8); rdi = ((rsp.wrapping_add(0x20u64)) as u64) as u64; bb = 9; continue; }
            8 => { rax = (0xc05u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x2eu64), ((rax as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); rdx = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020f1e4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (0xffff000000000000u64) as u64; rdx = (((rdx) & (rax))) as u64; fl_l = (((rdx) | (rdi))) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rdx) as u64); bb = 9; continue; }
            9 => { r8 = (rbx) as u64; rdx = (rsi) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020f20f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0x0u8) as u8); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020f229); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020f240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x142480e20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 3; continue; } bb = 2; continue; }
            2 => { if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = rdx; rdx = r9; return crate::simd_ops::tail_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            4 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020f270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x3; const P2: u64 = 0x1426d01e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; rbx = rdx; r11 = rcx; r8 = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r10 = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r11)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r10 = (crate::simd_ops::ru64(r10)) as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(r11.wrapping_add(0x4u64))) as u32 as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rdi); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = r10; fl_r = r9; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rdi = P2; bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(r9))) as u32) as u32 as u64; rcx = ((rcx).wrapping_sub(1)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 14; continue; } bb = 11; continue; }
            11 => { if ((((fl_l as u8)) as i8) > (((fl_r as u8)) as i8)) { bb = 14; continue; } bb = 12; continue; }
            12 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(r11.wrapping_add(0x48u64), rbx); bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; bb = 15; continue; }
            15 => { rsp = ((rsp).wrapping_add(0x10u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14020f310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1402112c0; const P3: u64 = 0x1; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rbx = rcx;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64;
    rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64;
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); } else { rax = ((r8.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); crate::simd_ops::wu64(rcx, rax); }
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402112a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = rdx;
    rbx = rdx;
    r8 = ((r8).wrapping_sub(rcx)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402112c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e2180; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbp = 0x3fffffff; rdi = (((rdi) & (rbp))) as u64; r14 = rdx; rax = rdi; rsi = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = (0x8u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = 0x8; bb = 3; continue; }
            2 => { fl_l = rdi; fl_r = rbp; rdi = (if (((fl_l as u64)) > ((fl_r as u64))) { rbp } else { rdi }) as u64; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rsi))) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rdx = ((rdi as u32)) as u32 as u64; rbx = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rbx, rax); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; rax = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = r15; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64) as u128; rdx = ((rdx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rcx = rcx.wrapping_add(0x18); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { bb = 14; continue; }
            8 => { rax = r15; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64) as u128; rdx = ((rdx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = rax.wrapping_add(0x18); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = rbx.wrapping_add(0x18); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64) as u128; rcx = ((rcx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = rdx.wrapping_add(0x18); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x40000000; r9 = (crate::simd_ops::ru64(rsi)) as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 15; continue; }
            15 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 16; continue; }
            16 => { rcx = r9.wrapping_sub(0x1); rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); if (((fl_l as u32)) != 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            20 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 22; continue; }
            21 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 22; continue; }
            22 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rdi = ((((rdi as u32)) & ((rbp as u32)))) as u32 as u64; crate::simd_ops::wu64(rsi, r15); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rdi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402114b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e6d50; const P1: u64 = 0x140213020; const P2: u64 = 0x142535070; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x140213060; const P6: u64 = 0x143084fd0; const P7: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = rdx; rbx = rcx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x3cu64))) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = P1; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdx); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x38u64), (1) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = rdx; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; rsi = 0x8; rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rcx }) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rbp = 0x10000; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = (0x80u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = ((rdx.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rax); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r8 = 0x10; rdx = rbp; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rdx = rsp.wrapping_add(0x30); rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rbp); rax = rcx; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; rsi = ((rsi).wrapping_sub(rax)) as u64; fl_l = rax; fl_r = rax; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rsi }) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = rbp; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = rax; fl_r = (0x80u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdi = ((rcx.wrapping_add(rsi)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rax); bb = 11; continue; }
            10 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); rax = P6; crate::simd_ops::wu64(rdi, rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0xcu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xcu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0xdu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xdu64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x21u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x21u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x22u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x22u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x23u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x23u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x24u64), ((rax as u8)) as u8); rcx = rdi.wrapping_add(0x28); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = r14.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x50u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x50u64), ((rax as u8)) as u8); rcx = rdi.wrapping_add(0x54); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = r14.wrapping_add(0x54); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u64(rbx.wrapping_add(0x40u64), (0xffffffffffffff80u64) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x48u64), (0xffffffffffffff80u64) as u64); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x20u64)))) as u64; rbp = ((rbp).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x48u64)))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x8u64), rbp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402116e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307e950; const P1: u64 = 0x142645870; const P2: u64 = 0x1401e5930; const P3: u64 = 0x143426fc8; const P4: u64 = 0x1402128d0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = r8; r14 = rdx; rbx = rcx; rax = (0x7ffffffffffffffu64) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x140211720, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { r15 = rcx.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rdx as u32)) as u32); r9 = ((rdx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(24u32)) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64; r8 = (((rcx as u8)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(8u32)) as u32 as u64; rcx = (((rdx as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); rsi = ((crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; rax = (0xcbf29ce484222325u64) as u64; rsi = (((rsi) ^ (rax))) as u64; rax = (0x100000001b3u64) as u64; rsi = (((rsi) as i64).wrapping_mul((rax) as i64) as u64) as u64; rsi = (((rsi) ^ (rcx))) as u64; rsi = (((rsi) as i64).wrapping_mul((rax) as i64) as u64) as u64; rsi = (((rsi) ^ (r8))) as u64; rsi = (((rsi) as i64).wrapping_mul((rax) as i64) as u64) as u64; rsi = (((rsi) ^ (r9))) as u64; rsi = (((rsi) as i64).wrapping_mul((rax) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; xmm2 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; fl_l = ((rcx).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm1 = 0u128; if ((((fl_l as u64)) as i64) < 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 5; continue; }
            4 => { rax = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rax) | (rcx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; xmm0 = 0u128; fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 8; continue; }
            7 => { rcx = rdx; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = rdx; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 8; continue; }
            8 => { xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = 0u128; xmm1 = crate::simd_ops::vroundss(xmm1, (xmm0 as u32), 0x2u8); rcx = 0; fl_l = 0; fl_r = 0; xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (0x8000000000000000u64) as u64; rcx = rax; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = 0x8; fl_l = rax; fl_r = rcx; rcx = (if (((fl_l as u64)) > ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = rdx; fl_r = (0x200u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = rcx; bb = 16; continue; }
            16 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r8 = rbx.wrapping_add(0x30); r9 = (crate::simd_ops::ru64(r8)) as u64; r9 = (((r9) & (rsi))) as u64; rcx = rbx.wrapping_add(0x18); r9 = ((r9).wrapping_add(r9)) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 18; continue; }
            18 => { r10 = (crate::simd_ops::ru64(r10.wrapping_add(r9.wrapping_mul(8)))) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 22; continue; }
            22 => { rcx = rbx.wrapping_add(0x18); r8 = rbx.wrapping_add(0x30); bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(rax)) as u64; bb = 24; continue; }
            24 => { r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64); crate::simd_ops::wu64(rdi, rdx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(r9, rdi); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rdi); rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rax = (((rax) & (rsi))) as u64; rax = ((rax).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), rdi); bb = 30; continue; }
            26 => { rdx = rax; rcx = rbx.wrapping_add(0x18); r8 = rbx.wrapping_add(0x30); bb = 24; continue; }
            27 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), rdi); bb = 31; continue; }
            29 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64), rdi); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(r14, rdi); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140211b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140211c80; const P1: u64 = 0x140212740; const P2: u64 = 0x140211d60;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0x100u64)) as u64; r15 = (crate::simd_ops::ru64(rcx)) as u64; rbx = (((rdx as u16)) as u32) as u32 as u64; r14 = r15.wrapping_add(0x40); fl_l = (crate::simd_ops::ru64(r14.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; rdi = ((r14.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 3; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { rdx = rcx; rcx = rbp.wrapping_add(0x7); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; rcx = rbp.wrapping_sub(0x49); r9 = (((rbx as u16)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); r8 = r14; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rbx as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((r9).wrapping_add(rdi)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax).wrapping_add(rdi)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx).wrapping_add(rdi)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((rdx).wrapping_add(rdi)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rdx); rdx = rbp.wrapping_sub(0x49); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), r8); r8 = rbp.wrapping_add(0x7); let y_addr: u64 = rbp.wrapping_sub(0x49u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x59u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), r9); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), rcx); let y_addr: u64 = rbp.wrapping_sub(0x79u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x29u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { r11 = rsp.wrapping_add(0x100); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140211d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x40; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x48; const P6: u64 = 0x30; const P7: u64 = 0x14186d750;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; rbp = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbx = r8; rdi = rdx; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { let y_addr: u64 = rdi.wrapping_add(P1); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm2 = (crate::simd_ops::ru128(rdi.wrapping_add(P2))) as u128; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = rsp.wrapping_add(0x70); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; xmm0 = (ymm1_hi) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (((xmm1 >> 64) as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; xmm1 = (ymm1_hi) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (((xmm1 >> 64) as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P1))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm2 as u64)) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = rsp.wrapping_add(0xa0); let y_addr: u64 = rsp.wrapping_add(0xa0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0xc0u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140211e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d68c; const P1: u64 = 0x180ac; const P2: u64 = 0x18098; const P3: u64 = 0x142489ee0; const P4: u64 = 0x40; const P5: u64 = 0x14020c1d0; const P6: u64 = 0x25;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = 0; fl_l = 0; fl_r = 0; r11 = (((rdx as u16)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rdi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rbx as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi).wrapping_add(P1))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdi).wrapping_add(P2))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdi).wrapping_add(P4))) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 7; continue; }
            6 => { rdi = rbx; bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rdi; rax = ((r14.wrapping_add(rdi)) as u64) as u64; rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rdi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(1)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P6), (0x0u8) as u8); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402120d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d468; const P1: u64 = 0x143c8d474; const P2: u64 = 0x142489ee0; const P3: u64 = 0x14020c1e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rbx = 0; fl_l = 0; fl_r = 0; r11 = (((rdx as u16)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rdi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rbx as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x580e0u64))) as u64; r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r15); r15 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi).wrapping_add(0x180acu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdi).wrapping_add(0x18098u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdi).wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 7; continue; }
            6 => { rdi = rbx; bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rdi; rax = ((r14.wrapping_add(rdi)) as u64) as u64; rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rdi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x154u64))) as u32 as u64; fl_r = 0x2; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = ((rbx).wrapping_add(1)) as u64; crate::simd_ops::wu8(r15, ((rax as u8)) as u8); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 11; continue; }
            11 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140212420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x120;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu8(r8, ((rax as u8)) as u8);
    rax = rcx;
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (1) as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140212440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x120;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64;
    rax = rcx;
    crate::simd_ops::wu8(rdx, ((r8 as u8)) as u8);
    fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P0), (1) as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140212460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x28; const P2: u64 = 0x143084e28; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x54u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P2;
    crate::simd_ops::wu64(rdi, rax);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x80; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140212740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d478; const P1: u64 = 0x142489ee0; const P2: u64 = 0x143c8d5f4; const P3: u64 = 0x143c8d4b4; const P4: u64 = 0x143c8d528; const P5: u64 = 0x143c8d508; const P6: u64 = 0x143c8d46c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (((r9 as u16)) as u32) as u32 as u64; r14 = r8; rax = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); rax = ((rax).wrapping_shl(5u32)) as u64; r15 = rcx; rax = ((rax).wrapping_add(r8)) as u64; r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); r13 = (crate::simd_ops::ru32(rax.wrapping_add(0x1806cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18058u64))) as u64; rdx = ((r13 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; r11 = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = rbx.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r14))) as u64; rbp = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; bb = 3; continue; }
            2 => { rbp = r11; bb = 3; continue; }
            3 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = 0xc03; rdx = r13; rbx = ((rbx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(r14))) as u64; rsi = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; bb = 6; continue; }
            5 => { rsi = r11; bb = 6; continue; }
            6 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rdx = r13; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r12 = ((rbx.wrapping_add(r14)) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r12)) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 9; continue; }
            8 => { rdi = r11; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r8 = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18068u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18050u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18058u64))) as u64; r14 = (0xf1a9f1a9f1a9f1a9u64) as u64; rdx = r13; r14 = (if (((fl_l as u32)) == ((fl_r as u32))) { r11 } else { r14 }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r12)) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; bb = 12; continue; }
            11 => { rbx = r11; bb = 12; continue; }
            12 => { r8 = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rdx = r13; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(r12)) as u64; r11 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 14; continue; }
            14 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = r15; crate::simd_ops::wu64(r15.wrapping_add(0x20u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(r15, rbp); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rsi); crate::simd_ops::wu64(r15.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(r15.wrapping_add(0x18u64), r14); crate::simd_ops::wu64(r15.wrapping_add(0x28u64), r11); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402128d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307e970; const P1: u64 = 0x142645870; const P2: u64 = 0x1401e5730;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (0xfffffffffffffffu64) as u64; r9 = 0x1; let bit_in: u64 = rax; if bit_in != 0 { r8 = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rsi = rcx; rax = ((r9).wrapping_shl((((r8) as u32) & 63))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140212904, rcx, rsp); return rax; }
            2 => { rax = rdx.wrapping_sub(0x1); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (((rax) | (r9))) as u64; let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); r8 = rbx; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rdi = ((r9).wrapping_shl((((rcx) as u32) & 63))) as u64; rcx = rsi.wrapping_add(0x18); rdx = ((rdi.wrapping_add(rdi)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = rdi.wrapping_sub(0x1); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rdi = (0x100000001b3u64) as u64; rbp = (0xcbf29ce484222325u64) as u64; bb = 4; continue; }
            4 => { rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x11u64))) as u32) as u32 as u64; r11 = ((crate::simd_ops::ru8(rax.wrapping_add(0x10u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; r11 = (((r11) ^ (rbp))) as u64; r11 = (((r11) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x12u64))) as u32) as u32 as u64; r11 = (((r11) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) ^ (rdx))) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x13u64))) as u32) as u32 as u64; r11 = (((r11) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) ^ (rdx))) as u64; r11 = (((r11) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r11 = (((r11) & (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))))) as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x18u64)))) as u64; r9 = (crate::simd_ops::ru64(r11)) as u64; fl_l = r9; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r11, rax); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax); bb = 15; continue; }
            6 => { rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { r10 = (crate::simd_ops::ru64(rdx)) as u64; fl_l = r10; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r9, rcx); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, r10); rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(r10.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdx); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax); bb = 15; continue; }
            10 => { fl_l = r9; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { fl_l = r9; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r10, rcx); r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r9, rdx); r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r10); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(r11, rax); bb = 15; continue; }
            15 => { rax = rcx; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 16; continue; }
            16 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { r10 = (crate::simd_ops::ru64(rdx)) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r9, rcx); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, r10); rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(r10.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdx); bb = 15; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140212a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x140215610;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r8 = ((r8).wrapping_add(r8)) as u64;
    rax = ((rdx.wrapping_add(rdx)) as u64) as u64;
    fl_l = rax; fl_r = r8;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64;
    rax = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64;
    crate::simd_ops::wu16(rdx, ((rax as u16)) as u16);
    crate::simd_ops::lk_add_u32(rbx.wrapping_add(P0), (1) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140212ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424810b0; const P1: u64 = 0x14307cb10; const P2: u64 = 0x1426cffff; const P3: u64 = 0x1401f2430; const P4: u64 = 0x1401eb150; const P5: u64 = 0x14240ab10; const P6: u64 = 0x142409b50; const P7: u64 = 0x1401f22d0; const P8: u64 = 0x14240a730;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140212ae0, funcinfo_rva: 0x35a5524, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x148, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rdi = (r9) as u64; rbx = (r8) as u64; r8 = (r9) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212af9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { r9 = (rbx) as u64; r8 = ((P1) as u64) as u64; rdx = ((crate::simd_ops::ru8(P2)) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212b1c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x178u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212b2c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212b36); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x190u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rcx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x10000010u64) as u64); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rdi as u8)) as u8); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rbx as u8)) as u8); bb = 6; continue; }
            3 => { r9 = (0xffffffffffffffffu64) as u64; bb = 4; continue; }
            4 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212b91); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x180u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212bd0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212bfb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212c08); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; rsp = ((rsp).wrapping_add(0x140u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140212c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142481220; const P1: u64 = 0x142409b50; const P2: u64 = 0x1401f22d0; const P3: u64 = 0x14240a730; const P4: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140212c20, funcinfo_rva: 0x35a5550, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rdi = (r9) as u64; rbx = (r8) as u64; r8 = (r9) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212c39); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rdi, ((rcx as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x10000010u64) as u64); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rdi as u8)) as u8); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rbx as u8)) as u8); bb = 6; continue; }
            3 => { r9 = (0xffffffffffffffffu64) as u64; bb = 4; continue; }
            4 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212ca1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212ce0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212d0b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212d18); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140212d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424810b0; const P1: u64 = 0x142409b50; const P2: u64 = 0x1401f22d0; const P3: u64 = 0x14240a730; const P4: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140212d30, funcinfo_rva: 0x35a5550, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rdi = (r9) as u64; rbx = (r8) as u64; r8 = (r9) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212d49); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rcx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x10000010u64) as u64); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rdi as u8)) as u8); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rbx as u8)) as u8); bb = 6; continue; }
            3 => { r9 = (0xffffffffffffffffu64) as u64; bb = 4; continue; }
            4 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212db1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212df0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212e1b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140212e28); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140213060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x1; const P3: u64 = 0x140215e80; const P4: u64 = 0x14183acc8; const P5: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rdi = rcx; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rbp = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsi = ((((rsi) as i64).wrapping_shr(4u32) as u64)) as u64; rax = ((rsi.wrapping_add(P2)) as u64) as u64; rcx = ((((rcx) as i64).wrapping_shr(4u32) as u64)) as u64; rbx = rcx; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rax; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rsi as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rax = ((rsi as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(rax)) as u64; xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(P1), (1) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff0u64)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402134d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140211c80; const P1: u64 = 0x140212740; const P2: u64 = 0x14186d750;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 51>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x78); rsp = ((rsp).wrapping_sub(0x178u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = rcx; r15 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r15 = ((r15).wrapping_add(r15)) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(r15.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(r15.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x168u64), r13); rcx = rbp.wrapping_sub(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx)) as u64; r13 = ((r15.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); fl_l = r15; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x1a0u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x170u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), r14); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r15)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xa8u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xb8u64), rax); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 7; continue; }
            7 => { r8 = rdx; rcx = rbp.wrapping_sub(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r12 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { r12 = ((rbx.wrapping_add(rax)) as u64) as u64; bb = 10; continue; }
            10 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r14).wrapping_add(rbx)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rsi).wrapping_add(rbx)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdi).wrapping_add(rbx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 22; continue; } bb = 19; continue; }
            19 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 20; continue; }
            20 => { xmm0 = (crate::simd_ops::ru128(rbp)) as u128; ymm0_hi = 0; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; rdx = rbp.wrapping_sub(0x40); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; rcx = rbp.wrapping_add(0x20); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), r13); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), r15); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); let y_addr: u64 = rbp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((r15).wrapping_add(1)) as u64; fl_l = r15; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 23; continue; }
            23 => { r15 = ((r15).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); fl_l = r15; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 24; continue; }
            24 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a0u64))) as u64; bb = 25; continue; }
            25 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0x178u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}
