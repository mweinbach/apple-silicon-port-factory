#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f6490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14242e660; const P1: u64 = 0x143a082d0; const P2: u64 = 0x1413a50e0; const P3: u64 = 0x14240ab10; const P4: u64 = 0x1401de830; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0xf0u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x30); rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x60); rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xf0u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(P1, (0x0u64) as u64); rcx = rsi.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rbx.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 11; continue; }
            11 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f65a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f6bc0; const P1: u64 = 0x14307db90; const P2: u64 = 0x1426680a0; const P3: u64 = 0x14341f118; const P4: u64 = 0x1401f7120; const P5: u64 = 0x1401f6b30; const P6: u64 = 0x1401f6a60; const P7: u64 = 0x1401f6cb0; const P8: u64 = 0x1401f6eb0; const P9: u64 = 0x14307df20; const P10: u64 = 0x142646338; const P11: u64 = 0x141462ac0; const P12: u64 = 0x1437feb90; const P13: u64 = 0x14183acce; const P14: u64 = 0x14307e078; const P15: u64 = 0x1401f6903;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f65a0, funcinfo_rva: 0x35a13f4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x37u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rsi = (r8) as u64; r14 = (rdx) as u64; rbx = (rcx) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdi = ((rcx.wrapping_add(0x1u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x7du8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rax = ((rcx.wrapping_sub(0x61u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rcx.wrapping_sub(0x41u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4cu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f65f8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 51; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((rax as u8)) as u8); rax = (rdi) as u64; bb = 49; continue; }
            7 => { fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 8; continue; }
            8 => { r8 = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = ((P1) as u64) as u64; rcx = (((crate::simd_ops::ru8(rax.wrapping_add(rcx))) as i8 as i32) as u32) as u32 as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (r14) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; rdi = (rbx) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; rdi = (if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { rdx } else { rdi }) as u64; bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; r15 = ((P2) as u64) as u64; r13 = ((P3) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3cu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3eu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5eu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (rdi) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 13; continue; }
            13 => { rdi = (rbx) as u64; bb = 9; continue; }
            14 => { r12 = (r12 & !0xffu64) | (((0x3u8) as u8 as u64)); bb = 17; continue; }
            15 => { r12 = (r12 & !0xffu64) | (((0x2u8) as u8 as u64)); bb = 17; continue; }
            16 => { r12 = (r12 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 17; continue; }
            17 => { fl_l = (rdi) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 18; continue; }
            18 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x7bu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 52; continue; } bb = 19; continue; }
            19 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x7du8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rbx) as u64); rax = (rdi) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (rax) as u64); rdx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f66af); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            21 => { rdi = (rbx) as u64; bb = 22; continue; }
            22 => { rbx = ((rdi.wrapping_add(0x1u64)) as u64) as u64; fl_l = ((r12 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f66c6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x9u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ ((r12 as u8)))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfu8))) as u8 as u64)); fl_l = (crate::simd_ops::lk_xor_u8(rdx.wrapping_add(0x9u64), ((rcx as u8)) as u8)) as u8 as u64; fl_r = fl_l; bb = 25; continue; }
            25 => { fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 26; continue; }
            26 => { rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 27; continue; }
            27 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2bu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2du8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 29; continue; }
            29 => { rdi = (0x1u32) as u32 as u64; rdx = (((rdi as u8)) as u32) as u32 as u64; bb = 33; continue; }
            30 => { rdx = (rdx & !0xffu64) | (((0x2u8) as u8 as u64)); bb = 32; continue; }
            31 => { rdx = (rdx & !0xffu64) | (((0x3u8) as u8 as u64)); bb = 32; continue; }
            32 => { rdi = (0x1u32) as u32 as u64; bb = 33; continue; }
            33 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f670c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(rdi)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x23u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6725); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::lk_or_u8(rax.wrapping_add(0x9u64), (0x80u8) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 37; continue; }
            37 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6745); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x9u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf4u8))) as u8 as u64)); fl_l = ((((rax as u8)) | (0x4u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x9u64), ((rax as u8)) as u8); bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0xbu64), (0x30u8) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 40; continue; }
            40 => { r8 = (rsi) as u64; rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6774); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 49; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x2eu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 43; continue; } bb = 42; continue; }
            42 => { r8 = (rsi) as u64; rdx = (r14) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f678f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 49; continue; } bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x4cu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f67a4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::lk_or_u8(rax.wrapping_add(0xau64), (0x1u8) as u8); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 45; continue; }
            45 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x7du8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 48; continue; } bb = 46; continue; }
            46 => { rbx = ((rbx).wrapping_add(1)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f67c3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 50; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x8u64), ((rdi as u8)) as u8); bb = 48; continue; }
            48 => { rax = (rbx) as u64; bb = 49; continue; }
            49 => { r11 = ((rsp.wrapping_add(0x90u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            50 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (r15) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x31u64), (0x1u8) as u8); rdx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f681e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (r13) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x67u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x67u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x1u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6838); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P12) as u64) as u64; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6848); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 51; continue; }
            51 => { r15 = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (r15) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x31u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x11u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6879); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (r13) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x67u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x67u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f689a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P12) as u64) as u64; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f68aa); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 52; continue; }
            52 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (r15) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); rax = ((P14) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x21u64), (0x1u8) as u8); rdx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f68d4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (r13) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x67u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x67u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x1u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f68ee); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P12) as u64) as u64; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f68fe); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P15, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f6910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426680a0; const P1: u64 = 0x14307df20; const P2: u64 = 0x142646338; const P3: u64 = 0x14341f118; const P4: u64 = 0x141462ac0; const P5: u64 = 0x1437feb90; const P6: u64 = 0x14183acce; const P7: u64 = 0x1401f6992;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f6910, funcinfo_rva: 0x35a1428, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 3; continue; }
            3 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rbx = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6956); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f697c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P5) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f698d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f6a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143823cae;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    if !((((fl_l as u8)) == ((fl_r as u8)))) { crate::simd_ops::wu8(P0, (0x0u8) as u8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f6a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f6b30; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x9; const P4: u64 = 0x1426680a0; const P5: u64 = 0x14307df38; const P6: u64 = 0x142646338; const P7: u64 = 0x14341f118; const P8: u64 = 0x141462ac0; const P9: u64 = 0x1437feb90; const P10: u64 = 0x14183acce; const P11: u64 = 0x1401f6b23;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f6a60, funcinfo_rva: 0x35a1454, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = (((rdx as u8)) as u32) as u32 as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6a75); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rax = ((rdx.wrapping_add(P2)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x8u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (0x12au32) as u32 as u64; fl_l = ((((rax as u32)) >> ((((rdx as u32)) as u32 & 31))) & 1) as u64; if ((fl_l as u32) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rbx = (rbx & !0xffu64) | (((((rbx as u8)).wrapping_shl(4u32)) as u8 as u64)); rbx = (rbx & !0xffu64) | ((((((rbx as u8)) ^ (crate::simd_ops::ru8(rcx.wrapping_add(P3))))) as u8 as u64)); rbx = (rbx & !0xffu64) | ((((((rbx as u8)) & (0x70u8))) as u8 as u64)); crate::simd_ops::lk_xor_u8(rcx.wrapping_add(P3), ((rbx as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rsi = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6ae7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6b0d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P9) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6b1e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f6b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x1426680a0; const P2: u64 = 0x14307e030; const P3: u64 = 0x142646338; const P4: u64 = 0x14341f118; const P5: u64 = 0x141462ac0; const P6: u64 = 0x1437feb90; const P7: u64 = 0x14183acce; const P8: u64 = 0x1401f6bb3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f6b30, funcinfo_rva: 0x35a1474, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rbx = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6b77); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6b9d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P6) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6bae); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f6bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((((rcx as u8)) as i8 as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xffffffc1u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x39; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 20; continue; } bb = 1; continue; }
            1 => { rdx = P0; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(0x1f6c70u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1f6c24u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            2 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            3 => { rax = (rax & !0xffu64) | (((0x2u8) as u8 as u64)); return rax; }
            4 => { rax = (rax & !0xffu64) | (((0x3u8) as u8 as u64)); return rax; }
            5 => { rax = (rax & !0xffu64) | (((0x4u8) as u8 as u64)); return rax; }
            6 => { rax = (rax & !0xffu64) | (((0x5u8) as u8 as u64)); return rax; }
            7 => { rax = (rax & !0xffu64) | (((0x6u8) as u8 as u64)); return rax; }
            8 => { rax = (rax & !0xffu64) | (((0x7u8) as u8 as u64)); return rax; }
            9 => { rax = (rax & !0xffu64) | (((0x8u8) as u8 as u64)); return rax; }
            10 => { rax = (rax & !0xffu64) | (((0x9u8) as u8 as u64)); return rax; }
            11 => { rax = (rax & !0xffu64) | (((0xau8) as u8 as u64)); return rax; }
            12 => { rax = (rax & !0xffu64) | (((0xbu8) as u8 as u64)); return rax; }
            13 => { rax = (rax & !0xffu64) | (((0xcu8) as u8 as u64)); return rax; }
            14 => { rax = (rax & !0xffu64) | (((0xdu8) as u8 as u64)); return rax; }
            15 => { rax = (rax & !0xffu64) | (((0xeu8) as u8 as u64)); return rax; }
            16 => { rax = (rax & !0xffu64) | (((0xfu8) as u8 as u64)); return rax; }
            17 => { rax = (rax & !0xffu64) | (((0x10u8) as u8 as u64)); return rax; }
            18 => { rax = (rax & !0xffu64) | (((0x11u8) as u8 as u64)); return rax; }
            19 => { rax = (rax & !0xffu64) | (((0x12u8) as u8 as u64)); return rax; }
            20 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1401f6bc0u64) { 0xb => { bb = 1; continue; } 0x28 => { bb = 2; continue; } 0x2b => { bb = 3; continue; } 0x2e => { bb = 4; continue; } 0x31 => { bb = 5; continue; } 0x34 => { bb = 6; continue; } 0x37 => { bb = 7; continue; } 0x3a => { bb = 8; continue; } 0x3d => { bb = 9; continue; } 0x40 => { bb = 10; continue; } 0x43 => { bb = 11; continue; } 0x46 => { bb = 12; continue; } 0x49 => { bb = 13; continue; } 0x4c => { bb = 14; continue; } 0x4f => { bb = 15; continue; } 0x52 => { bb = 16; continue; } 0x55 => { bb = 17; continue; } 0x58 => { bb = 18; continue; } 0x5b => { bb = 19; continue; } 0x5e => { bb = 20; continue; } _ => { return crate::simd_ops::call_fwd(j_tgt, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f6cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f7770; const P1: u64 = 0x1401f76e0; const P2: u64 = 0x1426680a0; const P3: u64 = 0x14307dbb0; const P4: u64 = 0x142646338; const P5: u64 = 0x14341f118; const P6: u64 = 0x141462ac0; const P7: u64 = 0x1437feb90; const P8: u64 = 0x14183acce; const P9: u64 = 0x14307e060; const P10: u64 = 0x1401f6ea7;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f6cb0, funcinfo_rva: 0x35a149c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (r8) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; rcx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rax = ((rcx.wrapping_sub(0x30u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 11; continue; } bb = 1; continue; }
            1 => { r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r9 = (rbx) as u64; bb = 2; continue; }
            2 => { r8 = ((r10 as u32)) as u32 as u64; rdx = ((((rcx as u8)) as i8 as i32) as u32) as u32 as u64; r10 = ((r10.wrapping_add(r10.wrapping_mul(4))) as u32) as u32 as u64; r10 = ((r10.wrapping_sub(0x18u64)) as u32) as u32 as u64; r10 = ((rdx.wrapping_add(r10.wrapping_mul(2))) as u32) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; fl_l = (r9) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x39u8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rbx = (r9) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x9u64) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (rax) as u64 as u64; fl_r = (0xau64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 7; continue; }
            7 => { rax = ((rdx.wrapping_sub(0x30u64)) as u32) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x7fffffffu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 20; continue; } bb = 8; continue; }
            8 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu32(rax, ((r10 as u32)) as u32); bb = 10; continue; }
            10 => { rax = (rbx) as u64; r11 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x7bu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 12; continue; }
            12 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), (rsi) as u64); rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7du8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { r8 = ((rbp.wrapping_add(0x10u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6d98); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 16; continue; }
            16 => { bb = 18; continue; }
            17 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6dac); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), (0x1u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x10u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), ((xmm1 as u64)) as u64); rax = (rbx) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x7du8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rbx = ((rax.wrapping_add(0x1u64)) as u64) as u64; bb = 10; continue; }
            20 => { rdi = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rdi) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6e0f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6e30); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6e40); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            21 => { rdi = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rdi) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6e71); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6e92); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6ea2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f6eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426680a0; const P1: u64 = 0x14341f118; const P2: u64 = 0x1401f71e0; const P3: u64 = 0x1401f7910; const P4: u64 = 0x1401f76e0; const P5: u64 = 0x14307e060; const P6: u64 = 0x142646338; const P7: u64 = 0x141462ac0; const P8: u64 = 0x1437feb90; const P9: u64 = 0x14183acce; const P10: u64 = 0x14307dfc0; const P11: u64 = 0x14307dbb0; const P12: u64 = 0x14307df90; const P13: u64 = 0x1401f7115;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f6eb0, funcinfo_rva: 0x35a14d0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x37u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x90u64)) as u64; r15 = (r8) as u64; r14 = (rdx) as u64; rdi = ((rcx.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rdi) as u64); rbx = ((P0) as u64) as u64; rsi = ((P1) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rax = ((rcx.wrapping_sub(0x30u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (0xffffffffu32) as u32 as u64; rcx = ((rbp.wrapping_add(0x67u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6f01); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); bb = 13; continue; }
            4 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x7bu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 5; continue; }
            5 => { rdi = ((rdi).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rdi) as u64); rax = (rdi) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), (r15) as u64); rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7du8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6f45); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; bb = 10; continue; }
            9 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6f53); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), (0x1u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x1u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x28u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x38u64), ((xmm1 as u64)) as u64); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rdi) as u64); rax = (rdi) as u64; bb = 11; continue; }
            11 => { fl_l = (rax) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rax) as u64); fl_l = (crate::simd_ops::ru8(rdi)) as u8 as u64; fl_r = (0x7du8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x18u64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 19; continue; } bb = 14; continue; }
            14 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xeu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x21u64), (0x1u8) as u8); rdx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f6fea); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (rsi) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x6fu64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x6fu64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x1u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7004); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P8) as u64) as u64; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7014); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); rax = ((P10) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x21u64), (0x1u8) as u8); rdx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f703e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), (rsi) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x6fu64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x6fu64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x1u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7058); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P8) as u64) as u64; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7068); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); rax = ((P11) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x31u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x11u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7092); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rsi) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x6fu64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x6fu64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x1u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f70ac); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P8) as u64) as u64; rcx = ((rbp.wrapping_sub(0x1u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f70bc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); rax = ((P12) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x21u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x11u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f70e6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rsi) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x6fu64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x6fu64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x1u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7100); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P8) as u64) as u64; rcx = ((rbp.wrapping_sub(0x1u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7110); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P13, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f7120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1; const P2: u64 = 0x4; const P3: u64 = 0x1426680a0; const P4: u64 = 0x14307e020; const P5: u64 = 0x142646338; const P6: u64 = 0x14341f118; const P7: u64 = 0x141462ac0; const P8: u64 = 0x1437feb90; const P9: u64 = 0x14183acce; const P10: u64 = 0x1401f71d8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f7120, funcinfo_rva: 0x35a14f8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; r10 = (crate::simd_ops::ru64(rcx)) as u64; r10 = ((r10).wrapping_add(0xbu64)) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = (r9) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (r10) as u64; r8 = ((xmm0 as u64)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r10) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rdx = (r9) as u64; bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(r8))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rcx = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(r10.wrapping_add(P2), ((r9 as u8)) as u8); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rbx = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f719c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P6) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f71c2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P8) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f71d3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f71e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffe8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); rdi = (crate::simd_ops::ru64(rcx)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r10 = rdi; rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; bb = 1; continue; }
            1 => { rbx = ((r9 as u32)) as u32 as u64; r11 = r10; r9 = ((r9.wrapping_add(r9.wrapping_mul(4))) as u32) as u32 as u64; rax = ((((rax as u8)) as i8 as i32) as u32) as u32 as u64; r10 = ((r10).wrapping_add(1)) as u64; r9 = ((r9.wrapping_add(P0)) as u32) as u32 as u64; r9 = ((rax.wrapping_add(r9.wrapping_mul(2))) as u32) as u32 as u64; fl_l = r10; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x39u8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 1; continue; } bb = 4; continue; }
            4 => { rax = r10; crate::simd_ops::wu64(rcx, r10); rax = ((rax).wrapping_sub(rdi)) as u64; fl_l = rax; fl_r = (0x9u64) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = (0xau64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (((crate::simd_ops::ru8(r11)) as i8 as i32) as u32) as u32 as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rax = (((rax as u32)).wrapping_sub(0x30u32)) as u32 as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = rax; fl_r = (0x7fffffffu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r9 = ((r8 as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = ((r9 as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f7270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f7ab0; const P1: u64 = 0x1401f8cc0; const P2: u64 = 0x1401f7bf0; const P3: u64 = 0x1401f7c90; const P4: u64 = 0x1401f7d30; const P5: u64 = 0x1426680a0; const P6: u64 = 0x14307df20; const P7: u64 = 0x142646338; const P8: u64 = 0x14341f118; const P9: u64 = 0x141462ac0; const P10: u64 = 0x1437feb90; const P11: u64 = 0x14183acce; const P12: u64 = 0x1401f764e;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm2_hi: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f7270, funcinfo_rva: 0x35a152c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x148, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0x48u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x120u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); rbx = (r9) as u64; r14 = (r8) as u64; rsi = (rdx) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); r9 = (rcx) as u64; r8 = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f72c1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 48; continue; }
            3 => { let y_addr: u64 = rcx; xmm2 = core::ptr::read_unaligned(y_addr as *const u128); ymm2_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbp.wrapping_sub(0x30u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm2); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm2_hi); let y_addr: u64 = rcx.wrapping_add(0x20u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbp.wrapping_sub(0x10u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); xmm1 = (ymm2_hi) as u128; rcx = ((xmm1 as u32)) as u32 as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r12 = (0x7fffffffffffffffu64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 5; continue; }
            5 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rax as u8)) as u8); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x18u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = ((r9.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7323); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r15 as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (((rcx) & (r12))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 8; continue; }
            8 => { rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(8)))) as u128; ymm0_hi = 0; xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm1 as u64)) as u64); bb = 12; continue; }
            9 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0xfu32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = ((rdx.wrapping_mul(4)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr((((rax) as u32) & 63))) as u64; fl_l = ((((rcx as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rcx as u32)) as u32); if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (rdx) as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(8)))) as u128; ymm0_hi = 0; bb = 12; continue; }
            12 => { crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); bb = 13; continue; }
            13 => { rax = ((rsp.wrapping_add(0x50u64)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), ((xmm1 as u64)) as u64); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; bb = 23; continue; }
            14 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r15 as u32)) as u32); rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; xmm0 = (ymm2_hi) as u128; ymm0_hi = 0; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = (((rcx) & (r12))) as u64; rax = (((xmm0 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 20; continue; } bb = 16; continue; }
            16 => { rax = (rax as u32 as i32 as i64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(8)))) as u128; ymm0_hi = 0; xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64) as u128; bb = 22; continue; }
            17 => { rdx = (((xmm0 >> 64) as u32) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0xfu32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 20; continue; } bb = 18; continue; }
            18 => { rax = ((rdx.wrapping_mul(4)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr((((rax) as u32) & 63))) as u64; fl_l = ((((rcx as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rcx as u32)) as u32); if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = ((((rdx as u32)) as i32 as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(8)))) as u128; ymm0_hi = 0; bb = 21; continue; }
            20 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x38u64))) as u128; ymm0_hi = 0; bb = 21; continue; }
            21 => { xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64) as u128; bb = 22; continue; }
            22 => { crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), ((xmm1 as u64)) as u64); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; bb = 23; continue; }
            23 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f743e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((rax as u32)) as u32); bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 25; continue; }
            25 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 45; continue; } bb = 26; continue; }
            26 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rax as u8)) as u8); xmm0 = (crate::simd_ops::ru128(rbp)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); rdx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; rcx = ((rbx.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7476); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r15 as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 34; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 30; continue; } bb = 28; continue; }
            28 => { rcx = (((rcx) & (r12))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 34; continue; } bb = 29; continue; }
            29 => { rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(8)))) as u128; ymm0_hi = 0; xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), ((xmm1 as u64)) as u64); bb = 33; continue; }
            30 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0xfu32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 34; continue; } bb = 31; continue; }
            31 => { rax = ((rdx.wrapping_mul(4)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr((((rax) as u32) & 63))) as u64; fl_l = ((((rcx as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rcx as u32)) as u32); if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 34; continue; } bb = 32; continue; }
            32 => { rcx = (rdx) as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(8)))) as u128; ymm0_hi = 0; bb = 33; continue; }
            33 => { crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); bb = 34; continue; }
            34 => { rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), ((xmm1 as u64)) as u64); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; bb = 44; continue; }
            35 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r15 as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 38; continue; } bb = 36; continue; }
            36 => { rcx = (((rcx) & (r12))) as u64; rax = (((crate::simd_ops::ru32(rbp)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 41; continue; } bb = 37; continue; }
            37 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(8)))) as u128; ymm0_hi = 0; xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64) as u128; bb = 43; continue; }
            38 => { rdx = (((crate::simd_ops::ru32(rbp)) as i32 as i64) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0xfu32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 41; continue; } bb = 39; continue; }
            39 => { rax = ((rdx.wrapping_mul(4)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shr((((rax) as u32) & 63))) as u64; fl_l = ((((rcx as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rcx as u32)) as u32); if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rcx = (rdx) as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(8)))) as u128; ymm0_hi = 0; bb = 42; continue; }
            41 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x38u64))) as u128; ymm0_hi = 0; bb = 42; continue; }
            42 => { xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64) as u128; bb = 43; continue; }
            43 => { crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), ((xmm1 as u64)) as u64); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; bb = 44; continue; }
            44 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7586); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_sub(0x2cu64), ((rax as u32)) as u32); bb = 45; continue; }
            45 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; xmm6 = (crate::simd_ops::ru128(r14)) as u128; rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x28u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xefu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 49; continue; } bb = 47; continue; }
            47 => { crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm6) as u128); r9 = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; r8 = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; rdx = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f75b8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 48; continue; }
            48 => { rax = (rsi) as u64; r11 = ((rsp.wrapping_add(0x120u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            49 => { rdi = ((P5) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdi) as u64); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rax = ((P6) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), (0x1u8) as u8); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7616); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P8) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x50u64))) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7639); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P10) as u64) as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7649); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f7650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1426680a0; const P2: u64 = 0x14307dfe0; const P3: u64 = 0x142646338; const P4: u64 = 0x14341f118; const P5: u64 = 0x141462ac0; const P6: u64 = 0x1437feb90; const P7: u64 = 0x14183acce; const P8: u64 = 0x1401f76d9;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f7650, funcinfo_rva: 0x35a1554, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), (0xffffffffu32) as u32); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rbx = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f769d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x78u64))) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f76c3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P6) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f76d4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f76e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1; const P2: u64 = 0x1426680a0; const P3: u64 = 0x14307e0f8; const P4: u64 = 0x142646338; const P5: u64 = 0x14341f118; const P6: u64 = 0x141462ac0; const P7: u64 = 0x1437feb90; const P8: u64 = 0x14183acce; const P9: u64 = 0x1401f7769;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f76e0, funcinfo_rva: 0x35a1554, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = (rcx) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f772d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7753); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7764); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f7770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f7650; const P1: u64 = 0x14307e060; const P2: u64 = 0x142513f10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r11 = r8; r10 = rdx; rsi = rcx; rax = ((r9.wrapping_sub(0x30u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { r8 = ((rdi as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rdx = ((((r9 as u8)) as i8 as i32) as u32) as u32 as u64; rdi = ((rdi.wrapping_add(rdi.wrapping_mul(4))) as u32) as u32 as u64; rdi = ((rdi.wrapping_sub(0x18u64)) as u32) as u32 as u64; rdi = ((rdx.wrapping_add(rdi.wrapping_mul(2))) as u32) as u32 as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r9 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x39u8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = rcx; rax = ((rax).wrapping_sub(rsi)) as u64; rsi = rcx; fl_l = rax; fl_r = (0x9u64) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = (0xau64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((rdx.wrapping_sub(0x30u64)) as u32) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = rax; fl_r = (0x7fffffffu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdi = 0x7fffffff; bb = 11; continue; }
            10 => { fl_l = ((rsi).wrapping_add(1)) as u64; rsi = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { fl_l = rsi; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7du8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = (crate::simd_ops::ru64(r11)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), ((xmm1 as u64)) as u64); rax = rsi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = rsi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub(0x41u8)) as u8 as u64)); fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x39u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 29; continue; } bb = 17; continue; }
            17 => { rcx = (0x3ffffff43ffffffu64) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 29; continue; } bb = 18; continue; }
            18 => { r8 = rsi; bb = 19; continue; }
            19 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 20; continue; }
            20 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x61u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7au8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 19; continue; } bb = 22; continue; }
            22 => { bb = 26; continue; }
            23 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x41u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5au8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 19; continue; } bb = 25; continue; }
            25 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5fu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 26; continue; }
            26 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 28; continue; } bb = 27; continue; }
            27 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x39u8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 19; continue; } bb = 28; continue; }
            28 => { rcx = r8; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); rcx = ((rcx).wrapping_sub(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rax = r8; rcx = (crate::simd_ops::ru64(r11)) as u64; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), ((xmm1 as u64)) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            29 => { rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f7910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f7650; const P1: u64 = 0x14307e060; const P2: u64 = 0x142513f10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r11 = r8; r10 = rdx; rsi = rcx; rax = ((r9.wrapping_sub(0x30u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { r8 = ((rdi as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rdx = ((((r9 as u8)) as i8 as i32) as u32) as u32 as u64; rdi = ((rdi.wrapping_add(rdi.wrapping_mul(4))) as u32) as u32 as u64; rdi = ((rdi.wrapping_sub(0x18u64)) as u32) as u32 as u64; rdi = ((rdx.wrapping_add(rdi.wrapping_mul(2))) as u32) as u32 as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r9 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x39u8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = rcx; rax = ((rax).wrapping_sub(rsi)) as u64; rsi = rcx; fl_l = rax; fl_r = (0x9u64) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = (0xau64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((rdx.wrapping_sub(0x30u64)) as u32) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = rax; fl_r = (0x7fffffffu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdi = 0x7fffffff; bb = 11; continue; }
            10 => { fl_l = ((rsi).wrapping_add(1)) as u64; rsi = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { fl_l = rsi; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7du8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = (crate::simd_ops::ru64(r11)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x28u64), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x38u64), ((xmm1 as u64)) as u64); rax = rsi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = rsi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_sub(0x41u8)) as u8 as u64)); fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x39u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 29; continue; } bb = 17; continue; }
            17 => { rcx = (0x3ffffff43ffffffu64) as u64; fl_l = (((rcx) >> (((r9) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 29; continue; } bb = 18; continue; }
            18 => { r8 = rsi; bb = 19; continue; }
            19 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = r8; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 20; continue; }
            20 => { rax = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x61u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7au8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 19; continue; } bb = 22; continue; }
            22 => { bb = 26; continue; }
            23 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x41u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5au8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 19; continue; } bb = 25; continue; }
            25 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x5fu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 26; continue; }
            26 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 28; continue; } bb = 27; continue; }
            27 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x39u8) as u8 as u64; if ((((fl_l as u8)) as i8) <= (((fl_r as u8)) as i8)) { bb = 19; continue; } bb = 28; continue; }
            28 => { rcx = r8; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); rcx = ((rcx).wrapping_sub(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rax = r8; rcx = (crate::simd_ops::ru64(r11)) as u64; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rdx.wrapping_add(0x28u64), (xmm0) as u128); crate::simd_ops::wu64(rdx.wrapping_add(0x38u64), ((xmm1 as u64)) as u64); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            29 => { rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rsi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f7ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401f7d30; const P2: u64 = 0x1426680a0; const P3: u64 = 0x14307df20; const P4: u64 = 0x142646338; const P5: u64 = 0x14341f118; const P6: u64 = 0x141462ac0; const P7: u64 = 0x1437feb90; const P8: u64 = 0x14183acce; const P9: u64 = 0x1401f7b72;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f7ab0, funcinfo_rva: 0x35a1610, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = (rcx) as u64; rax = ((crate::simd_ops::ru8(r9.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xefu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7ae2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { r14 = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7b30); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x98u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x98u64))) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7b5c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f7b6d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f7b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (crate::simd_ops::ru64(rcx)) as u64; r10 = rcx; crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x0u32) as u32); fl_l = r9; fl_r = r9; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = ((((r8 as u32)) as i32 as i64) as u64) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r8.wrapping_mul(8)))) as u128; xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx.wrapping_add(P0), ((xmm1 as u64)) as u64); rax = rdx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xf; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = ((r8.wrapping_mul(4)) as u32) as u32 as u64; rcx = ((r9).wrapping_shr((((rax) as u32) & 63))) as u64; fl_l = ((((rcx as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rcx as u32)) as u32); if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; rcx = ((((r8 as u32)) as i32 as i64) as u64) as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(8)))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); bb = 6; continue; }
            6 => { rax = rdx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f7d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x142513cb0; const P2: u64 = 0x1401f5bc0; const P3: u64 = 0x1401f8a00;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x88u64)) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32 as u64; rbp = (rcx) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r15 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rbx = (r9) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rcx) as u64); rdi = (rdx) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { r10 = ((rax as u32)) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rcx = ((r13 as u32)) as u32 as u64; r15 = ((r13 as u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(r15))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xc0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { r15 = ((r15).wrapping_add(1)) as u64; fl_l = (r15) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { r15 = (rdx) as u64; bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x8u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; rsi = (rsi & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rsi as u8)) as u8); fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), (r12) as u64); r9 = ((P0) as u64) as u64; r12 = (crate::simd_ops::ru64(r8)) as u64; rbx = (0x1u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (r14) as u64); r14 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = ((r14).wrapping_add(r12)) as u64 as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rbp = (r12) as u64; fl_l = (r12) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rsi = ((crate::simd_ops::ru8(rbp)) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 15; continue; }
            15 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 28; continue; } bb = 16; continue; }
            16 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x7fu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 17; continue; }
            17 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x22u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 18; continue; }
            18 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x5cu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 19; continue; }
            19 => { rcx = ((rsi as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 20; continue; }
            20 => { rbp = ((rbp).wrapping_add(1)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 21; continue; }
            21 => { r9 = ((P0) as u64) as u64; bb = 22; continue; }
            22 => { r8 = (r13) as u64; rsi = ((r13 as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rax = (rbx) as u64; fl_l = (r12) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (rbp) as u64; rax = ((rax).wrapping_sub(r12)) as u64; fl_l = ((rax).wrapping_add(rbx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 25; continue; }
            25 => { rbx = (rax) as u64; r12 = (r8) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 26; continue; }
            26 => { rsi = (((rsi as u32)).wrapping_add(0xfffffff7u32)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x53u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 30; continue; } bb = 27; continue; }
            27 => { rcx = ((crate::simd_ops::ru8(r9.wrapping_add(rsi).wrapping_add(0x1f7f6cu64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(r9.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x1f7f64u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(r9)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = (rdx) as u64; bb = 0xffff_fffe; continue; }
            28 => { r8 = ((rbp.wrapping_add(0x1u64)) as u64) as u64; r9 = ((P0) as u64) as u64; bb = 23; continue; }
            29 => { rbx = ((rax.wrapping_add(0x2u64)) as u64) as u64; bb = 32; continue; }
            30 => { fl_l = (rbp) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rbx = ((rbx).wrapping_add(0x4u64)) as u64; rbp = ((rbp).wrapping_add(1)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 32; continue; }
            32 => { fl_l = (r8) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 33; continue; }
            33 => { rsi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64; r13 = ((rbx.wrapping_add(0x1u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 35; continue; }
            34 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); rdx = ((rsp.wrapping_add(0xd8u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), (r13) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r15) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r8 = (rbx) as u64; rdx = (rdi) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rsi as u8)) as u8); rcx = (rbp) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (r15) as u64); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rax) as u64); rax = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r13) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x58u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbp) as u64; rsp = ((rsp).wrapping_add(0x88u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1401f7d30u64) { 0x3e => { bb = 1; continue; } 0x46 => { bb = 2; continue; } 0x60 => { bb = 3; continue; } 0x65 => { bb = 4; continue; } 0x70 => { bb = 5; continue; } 0x78 => { bb = 6; continue; } 0x80 => { bb = 7; continue; } 0x83 => { bb = 8; continue; } 0x90 => { bb = 9; continue; } 0xa9 => { bb = 10; continue; } 0xb1 => { bb = 11; continue; } 0xd7 => { bb = 12; continue; } 0xdf => { bb = 13; continue; } 0xe0 => { bb = 14; continue; } 0xec => { bb = 15; continue; } 0xf1 => { bb = 16; continue; } 0xf6 => { bb = 17; continue; } 0xfb => { bb = 18; continue; } 0x100 => { bb = 19; continue; } 0x10b => { bb = 20; continue; } 0x113 => { bb = 21; continue; } 0x11a => { bb = 22; continue; } 0x120 => { bb = 23; continue; } 0x128 => { bb = 24; continue; } 0x131 => { bb = 25; continue; } 0x13c => { bb = 26; continue; } 0x144 => { bb = 27; continue; } 0x15a => { bb = 28; continue; } 0x167 => { bb = 29; continue; } 0x16d => { bb = 30; continue; } 0x172 => { bb = 31; continue; } 0x17e => { bb = 32; continue; } 0x187 => { bb = 33; continue; } 0x1b2 => { bb = 34; continue; } 0x1de => { bb = 35; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f7fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1401f8d90; const P2: u64 = 0x1426680a0; const P3: u64 = 0x14307e0b0; const P4: u64 = 0x142646338; const P5: u64 = 0x14341f118; const P6: u64 = 0x141462ac0; const P7: u64 = 0x1437feb90; const P8: u64 = 0x14183acce; const P9: u64 = 0x14307e098; const P10: u64 = 0x1401f849d;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f7fc0, funcinfo_rva: 0x35a1698, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xeu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (rax as u32 as i32 as i64) as u64; r8 = ((P0) as u64) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1f84a0u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            2 => { rdi = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { bb = 3; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            9 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8093); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f80b4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f80c4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f80f5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8116); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8126); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8157); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8178); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8188); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f81b9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f81da); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f81ea); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f821b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f823c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f824c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f827d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f829e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f82ae); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            15 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f82df); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8300); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8310); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8341); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8362); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8372); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f83a3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f83c4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f83d4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8405); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8426); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8436); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            19 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8467); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8488); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8498); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1401f7fc0u64) { 0x21 => { bb = 1; continue; } 0x37 => { bb = 2; continue; } 0x42 => { bb = 3; continue; } 0x58 => { bb = 4; continue; } 0x6d => { bb = 5; continue; } 0x79 => { bb = 6; continue; } 0x7b => { bb = 7; continue; } 0x91 => { bb = 8; continue; } 0xa8 => { bb = 9; continue; } 0x10a => { bb = 10; continue; } 0x16c => { bb = 11; continue; } 0x1ce => { bb = 12; continue; } 0x230 => { bb = 13; continue; } 0x292 => { bb = 14; continue; } 0x2f4 => { bb = 15; continue; } 0x356 => { bb = 16; continue; } 0x3b8 => { bb = 17; continue; } 0x41a => { bb = 18; continue; } 0x47c => { bb = 19; continue; } _ => { drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f84e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1401f8e10; const P2: u64 = 0x1426680a0; const P3: u64 = 0x14307e158; const P4: u64 = 0x142646338; const P5: u64 = 0x14341f118; const P6: u64 = 0x141462ac0; const P7: u64 = 0x1437feb90; const P8: u64 = 0x14183acce; const P9: u64 = 0x14307e0c8; const P10: u64 = 0x1401f89bd;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f84e0, funcinfo_rva: 0x35a1698, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xeu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (rax as u32 as i32 as i64) as u64; r8 = ((P0) as u64) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1f89c0u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            2 => { rdi = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { bb = 3; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            9 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f85b3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f85d4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f85e4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8615); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8636); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8646); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8677); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8698); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f86a8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f86d9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f86fa); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f870a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f873b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f875c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f876c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f879d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f87be); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f87ce); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            15 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f87ff); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8820); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8830); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8861); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8882); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8892); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f88c3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f88e4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f88f4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8925); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8946); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8956); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            19 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rdx = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f8987); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f89a8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f89b8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1401f84e0u64) { 0x21 => { bb = 1; continue; } 0x37 => { bb = 2; continue; } 0x42 => { bb = 3; continue; } 0x58 => { bb = 4; continue; } 0x6d => { bb = 5; continue; } 0x79 => { bb = 6; continue; } 0x7b => { bb = 7; continue; } 0x91 => { bb = 8; continue; } 0xa8 => { bb = 9; continue; } 0x10a => { bb = 10; continue; } 0x16c => { bb = 11; continue; } 0x1ce => { bb = 12; continue; } 0x230 => { bb = 13; continue; } 0x292 => { bb = 14; continue; } 0x2f4 => { bb = 15; continue; } 0x356 => { bb = 16; continue; } 0x3b8 => { bb = 17; continue; } 0x41a => { bb = 18; continue; } 0x47c => { bb = 19; continue; } _ => { drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f8cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0xfffffffffffffff0; const P3: u64 = 0x14183acda;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = r8; rax = ((rax).wrapping_shr(62u32)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = 0; fl_l = 0; fl_r = 0; r8 = ((((r8) as i64).wrapping_shr(63u32) as u64)) as u64; r8 = (((r8) & (0xfffffffffffffff8u64))) as u64; r14 = (crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(P1))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; r15 = (crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(P2))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = r15; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = (0xffffffffffffffffu64) as u64; bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { r8 = rbx; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = rbp; bb = 7; continue; }
            7 => { rdx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdi = ((rdi).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 10; continue; }
            10 => { rax = 0xffffffff; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { fl_l = ((rdi).wrapping_add(rdi)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P0))) as u32 as u64; bb = 11; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f8e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1401f9300;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; rbx = (rdx) as u64; rdi = ((crate::simd_ops::ru8(r8.wrapping_add(0x10u64))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xfffffff7u32)) as u32 as u64; r14 = (rcx) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x53u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 17; continue; } bb = 1; continue; }
            1 => { rcx = ((P0) as u64) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rax).wrapping_add(0x1f8ff0u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1f8fdcu64))) as u32 as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = (rdx) as u64; bb = 0xffff_fffe; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = (rdi & !0xffu64) | (((0x6eu8) as u8 as u64)); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; bb = 14; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = (rdi & !0xffu64) | (((0x72u8) as u8 as u64)); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; bb = 14; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rdi = (rdi & !0xffu64) | (((0x74u8) as u8 as u64)); bb = 13; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; bb = 14; continue; }
            14 => { crate::simd_ops::wu8(rcx.wrapping_add(rax), (0x5cu8) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rax.wrapping_add(0x1u64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax), ((rdi as u8)) as u8); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64)) as u64; bb = 21; continue; }
            17 => { rdi = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); rsi = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rax = (((crate::simd_ops::ru8(rdi)) as i8 as i32) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r9 = (((rax as u8)) as u32) as u32 as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = ((rdi).wrapping_add(1)) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 21; continue; }
            21 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (r14) as u64; crate::simd_ops::wu64(r14, (rbx) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1401f8e90u64) { 0x2b => { bb = 1; continue; } 0x46 => { bb = 2; continue; } 0x53 => { bb = 3; continue; } 0x5b => { bb = 4; continue; } 0x68 => { bb = 5; continue; } 0x75 => { bb = 6; continue; } 0x7d => { bb = 7; continue; } 0x8a => { bb = 8; continue; } 0x97 => { bb = 9; continue; } 0x9f => { bb = 10; continue; } 0xa4 => { bb = 11; continue; } 0xb1 => { bb = 12; continue; } 0xb9 => { bb = 13; continue; } 0xc1 => { bb = 14; continue; } 0xd7 => { bb = 15; continue; } 0xdf => { bb = 16; continue; } 0xf1 => { bb = 17; continue; } 0x102 => { bb = 18; continue; } 0x110 => { bb = 19; continue; } 0x12f => { bb = 20; continue; } 0x134 => { bb = 21; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f90f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f9220; const P1: u64 = 0x14240aa00; const P2: u64 = 0x1401f7270; const P3: u64 = 0x14240ab10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f90f0, funcinfo_rva: 0x35a17f8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xa8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = ((rsp.wrapping_sub(0x47u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rsi = (r8) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x23u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x2bu64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x33u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x3bu64), (r14) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x43u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xbu64), (0xffffffffu32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0xfu64), ((r14 as u16)) as u16); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x11u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x12u64), (0x20u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x16u64), (0x1u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), ((r14 as u32)) as u32); rcx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f9158); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::lk_sub_u64(rbx.wrapping_add(0x8u64), (rcx) as u64); rax = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x19u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000002u32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x16u64), (0x2u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x15u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x11u64), ((r14 as u8)) as u8); rdx = (rdi) as u64; rcx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f9193); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), (rax) as u64); rdx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x16u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x19u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x11u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64)) } else { rcx }) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x15u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rax) as u64); r9 = (rsi) as u64; r8 = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x67u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f91ef); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f91f9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); r11 = ((rsp.wrapping_add(0x90u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f9220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401f65a0; const P2: u64 = 0x1426680a0; const P3: u64 = 0x14307df20; const P4: u64 = 0x142646338; const P5: u64 = 0x14341f118; const P6: u64 = 0x141462ac0; const P7: u64 = 0x1437feb90; const P8: u64 = 0x14183acce; const P9: u64 = 0x1401f92f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f9220, funcinfo_rva: 0x35a182c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = (rcx) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (rcx) as u64; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rdx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0xdu32) as u32); r8 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f9272); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xefu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f92b6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x98u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x98u64))) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f92e2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f92f3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f9300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307e178; const P1: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rdx; rdi = ((r9 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; r15 = rcx; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax), (0x5cu8) as u8); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = P0; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax), (0x78u8) as u8); rax = (0x3030303030303030u64) as u64; fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64)) as u64; rcx = rsp.wrapping_add(0x22); crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); bb = 5; continue; }
            5 => { rax = ((rdi as u32)) as u32 as u64; rcx = rcx.wrapping_sub(0x1); rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rdi = (((rdi as u32)).wrapping_shr(4u32)) as u32 as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rsi = rsp.wrapping_add(0x20); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r14 = rsp.wrapping_add(0x22); r14 = ((r14).wrapping_sub(rsi)) as u64; rdx = ((rdx).wrapping_add(r14)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdi = ((rdi).wrapping_sub(rcx)) as u64; fl_l = rdi; fl_r = r14; rdi = (if (((fl_l as u64)) >= ((fl_r as u64))) { r14 } else { rdi }) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; r8 = rdi; rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (rdi) as u64); rax = rsp.wrapping_add(0x22); rsi = ((rsi).wrapping_add(rdi)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = r15; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(r15, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f94b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(r8, rcx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rcx = (crate::simd_ops::ru64(r8)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx, ((rax as u8)) as u8); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x1cu64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x24u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x28u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x30u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x34u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x34u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x3cu64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x6348; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru64(r8)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = ((rcx.wrapping_add(0x40u64)) as u32) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; rcx = rax; r8 = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x1cu64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x24u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x30u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x34u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x34u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x3cu64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx, rcx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rcx); bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f9610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0x3fff;
    fl_l = (crate::simd_ops::ru16(rcx)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64;
    if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (0x3fffffffffffc000u64) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f97a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x78d;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu16(rcx.wrapping_add(P0), (0x0u16) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f97c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x3; const P2: u64 = 0x4; const P3: u64 = 0x1401df850; const P4: u64 = 0x1401e01e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f97c0, funcinfo_rva: 0x35a1aa4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (rcx) as u64; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f9829); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xff000002u32))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x2u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x0u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f985d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f9870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307e5c0; const P1: u64 = 0x1401fa980; const P2: u64 = 0x3; const P3: u64 = 0x4; const P4: u64 = 0x8; const P5: u64 = 0x142409b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f9870, funcinfo_rva: 0x35a1ac8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (rcx) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x7u64) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f98b0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::lk_and_u32(rbx, (0xff000002u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x2u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x2u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x0u32) as u32); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu8(rcx, (0x0u8) as u8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rcx, (0x0u8) as u8); bb = 7; continue; }
            4 => { r9 = (0xffffffffffffffffu64) as u64; bb = 5; continue; }
            5 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f9912); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f99a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307e4c0; const P1: u64 = 0x142407c80; const P2: u64 = 0x143c4aa00; const P3: u64 = 0x1401de830; const P4: u64 = 0x14240ab10; const P5: u64 = 0x142416d60; const P6: u64 = 0x14137e320; const P7: u64 = 0x1401f9ae0; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = ((rdx as u32)) as u32 as u64; rdi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; rcx = P2; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rdi.wrapping_add(0x128u64), ((rax as u32)) as u32); rcx = rdi.wrapping_add(0x778); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x700); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x54u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = rdi.wrapping_add(0x648); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rbx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (0x0u64) as u64); bb = 6; continue; }
            6 => { rbx = rdi.wrapping_add(0x608); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rbx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (0x0u64) as u64); bb = 8; continue; }
            8 => { rcx = rdi.wrapping_add(0x4fc); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x3f4); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x2ec); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x1e4); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x108); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0xa0); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = 0x790; rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f9ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e3450; const P1: u64 = 0x68; const P2: u64 = 0x1401de830; const P3: u64 = 0x58; const P4: u64 = 0x48; const P5: u64 = 0x38; const P6: u64 = 0x28; const P7: u64 = 0x18; const P8: u64 = 0x8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x78u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P7)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P8)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f9b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x141dc3300;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401f9b60, funcinfo_rva: 0x35a23ac, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = (r9) as u64; rdi = (r8) as u64; r9 = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x230u64))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = (r10) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rax) as u64); bb = 5; continue; }
            3 => { rdx = (0x228u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; rcx = (r9) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f9bc5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f9bd2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f9be0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (r10) as u64); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r9 = ((rdx.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 8; continue; }
            7 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; r8 = ((rbx.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 13; continue; }
            12 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(r8)) as u64; bb = 15; continue; }
            15 => { r11 = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdi.wrapping_add(0x8u64)) as u64) as u64; rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 18; continue; }
            17 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r11) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401f9cc9); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1401f9cc9, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1401f9b60) }; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f9d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401fa110; const P1: u64 = 0x48; const P2: u64 = 0x14240ab10;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x98u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f9d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb8; const P1: u64 = 0x1401fa110; const P2: u64 = 0x48; const P3: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = ((rcx).wrapping_add(0x98u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f9d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x38; const P2: u64 = 0x8; const P3: u64 = 0x1401fa1c0; const P4: u64 = 0x14240ab10; const P5: u64 = 0x1401e66f0; const P6: u64 = 0x20; const P7: u64 = 0x18; const P8: u64 = 0x1401e6780;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = ((rax).wrapping_shr(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rcx = rdi; r8 = rdx; rdx = (crate::simd_ops::ru64(rdx)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rsi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rsi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rax, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f9ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    rsi = rcx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rdi = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { loop { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } } crate::simd_ops::wu32(rsi.wrapping_add(P0), (0x0u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f9f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x1401fa380; const P2: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x60u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x0u64) as u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); }
    rdx = rbx;
    rcx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x60u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f9f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64);
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401f9fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x48; const P8: u64 = 0x50; const P9: u64 = 0x40; const P10: u64 = 0x54; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P9), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P10), (0xffffffffu32) as u32);
    crate::simd_ops::lk_and_u32(rcx.wrapping_add(P11), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rcx.wrapping_add(0x58u64), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rcx.wrapping_add(0x5cu64), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(0x60u64), ((rax as u8)) as u8);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(0x5bu64), (0x10u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fa020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x4; const P2: u64 = 0x142409b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x8u8) as u8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rcx, (0x0u8) as u8); bb = 7; continue; }
            4 => { r9 = (0xffffffffffffffffu64) as u64; bb = 5; continue; }
            5 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fa1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401fa6d0; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401e66f0; const P3: u64 = 0x1424b16a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = r8; r15 = rdx; r14 = rcx; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rbx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r13 = (crate::simd_ops::ru64(rdi.wrapping_add(rax.wrapping_mul(8)))) as u64; r12 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); bb = 2; continue; }
            2 => { rdi = rbx; rsi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u64(r14.wrapping_add(0x10u64), (1) as u64); fl_l = rdi; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { fl_l = r13; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r12, rbx); bb = 20; continue; }
            6 => { fl_l = r13; fl_r = r15; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(r12, r13); rax = r13; bb = 9; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(r12.wrapping_add(0x8u64), rax); fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x13u64))) as u32) as u32 as u64; r8 = rbx.wrapping_add(0x18); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 14; continue; }
            13 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { r8 = (crate::simd_ops::ru64(r8)) as u64; bb = 16; continue; }
            16 => { rcx = r8; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (((rax) & (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((r15).wrapping_add(rax)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; r12 = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; bb = 17; continue; }
            17 => { rdi = rbx; rsi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u64(r14.wrapping_add(0x10u64), (1) as u64); fl_l = rdi; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(r15, rbx); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 21; continue; }
            21 => { rax = rbp; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            22 => { crate::simd_ops::wu64(r15, r13); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), r13); fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 23; continue; }
            23 => { bb = 20; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fa380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = rcx.wrapping_add(0x38); r14 = rcx; rcx = (crate::simd_ops::ru64(rbx)) as u64; rbp = rdx; fl_l = rcx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x38u64), rcx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); rsi = rdx.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r15); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); r8 = ((r15 as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = rcx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = r14; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rbx, r15); bb = 7; continue; }
            7 => { rbx = r14.wrapping_add(0x38); rsi = rbp.wrapping_add(0x38); bb = 9; continue; }
            8 => { r8 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); crate::simd_ops::wu64(rbx, r15); rsi = rdx.wrapping_add(0x38); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { fl_l = rcx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rbp; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, r15); bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 15; continue; }
            14 => { crate::simd_ops::wu64(rbx, rcx); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), r15); bb = 15; continue; }
            15 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rax = rsp.wrapping_add(0x20); fl_l = r8; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r8)) as u64; rdx = rbp; rcx = r8; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsi, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            19 => { crate::simd_ops::wu64(rsi, r8); bb = 20; continue; }
            20 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fa640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x98; const P2: u64 = 0x1401fa110; const P3: u64 = 0x48; const P4: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0xb8u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xb8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsi.wrapping_add(P0), (0x0u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fa6d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x1424b16a0; const P4: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rbx = rcx;
    r9 = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    rcx = ((rdx.wrapping_add(P2)) as u64) as u64;
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80;
    if !((((fl_l as u32)) >= ((fl_r as u32)))) { r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; } else { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x7c00; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; }
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64;
    rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if !((((fl_l as u32)) <= ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(rcx)) as u64; }
    rdx = r9;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (((rax) & (crate::simd_ops::ru64(rbx.wrapping_add(P4))))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fa980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423fdfc0; const P1: u64 = 0x143079330; const P2: u64 = 0x141461fb0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 51>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401fa980, funcinfo_rva: 0x35a25f4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x198, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x190u64)) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fa998); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x4u64) as u64); rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x7ffu64) as u64); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (rsi) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); r9 = ((rbx as u32)) as u32 as u64; r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401faa26); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdx = (rcx) as u64; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rsp.wrapping_add(0x70u64)))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64 as u64; rbx = (if (((fl_l as u64)) > ((fl_r as u64))) { rdx } else { rbx }) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64 as u64; fl_r = (rbx) as u64 as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rsp.wrapping_add(0x60u64)) } else { rbx }) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax).wrapping_add(rbx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rax, (0x0u8) as u8); rax = (rsi) as u64; r11 = ((rsp.wrapping_add(0x190u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401faa80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423fe030;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401faa90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff58; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598; const P3: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (crate::simd_ops::ru64(P0)) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x10); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x10); bb = 8; continue; }
            8 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 12; continue; }
            12 => { rax = (0x69fafafafaffu64) as u64; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x20du32) as u32); r8 = rsi.wrapping_add(0x8); rax = r8.wrapping_add(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu8(rax, ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x5u64) as u64); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = ((xmm0 as u64)) as u64; xmm0 = (ymm0_hi) as u128; ymm0_hi = 0; crate::simd_ops::lk_and_u32(rax, (0x1ffu32) as u32); crate::simd_ops::lk_or_u32(rax, (0x200u32) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(r8.wrapping_sub(0x8u64), (((xmm0 >> 64) as u64)) as u64); crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fabe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1401fb750; const P3: u64 = 0x20; const P4: u64 = 0x30; const P5: u64 = 0x3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (0x4924924924924925u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; xmm0 = 0u128; ymm0_hi = 0; rax = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); crate::simd_ops::wu64(rcx, (0x4u64) as u64); crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x4u8) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(P4), ((rax as u8)) as u8); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P0), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(0xffffffffffffffc8u64)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fac70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424b16a0; const P1: u64 = 0x1401e6ff0; const P2: u64 = 0x14240ab10; const P3: u64 = 0x1401e66f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); r15 = rcx; rdi = rdx; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; r8 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x3u64))) as u32) as u32 as u64; rcx = rdi.wrapping_add(0x8); fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((r8.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x7c00; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (((rax) & (crate::simd_ops::ru64(r15.wrapping_add(0x30u64))))) as u64; r14 = r15.wrapping_add(0x8); rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rdx = rbx.wrapping_add(0x10); rbp = (crate::simd_ops::ru64(rsi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rdi; rdx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { bb = 12; continue; }
            11 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { r14 = r15.wrapping_add(0x8); bb = 13; continue; }
            13 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); bb = 21; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); bb = 21; continue; }
            19 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsi, rax); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::lk_sub_u64(r15.wrapping_add(0x10u64), (1) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fb2d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14240ab10; const P3: u64 = 0x1401e66f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rax, (0x0u64) as u64);
    rbx = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi);
    loop { rdi = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rdi; fl_l = rdi; fl_r = rdi; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fb320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x2; const P2: u64 = 0x3; const P3: u64 = 0x4; const P4: u64 = 0x5; const P5: u64 = 0x6; const P6: u64 = 0x7;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64;
    r9 = (0x100000001b3u64) as u64;
    rax = (0xcbf29ce484222325u64) as u64;
    rdx = rcx;
    r8 = (((r8) ^ (rax))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    r8 = (((r8) ^ (rax))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    r8 = (((r8) ^ (rax))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    r8 = (((r8) ^ (rax))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64;
    r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    r8 = (((r8) ^ (rax))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u32) as u32 as u64;
    rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P5))) as u32) as u32 as u64;
    r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rax = (((rax) ^ (r8))) as u64;
    rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rax = (((rax) ^ (rcx))) as u64;
    rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rax = (((rax) ^ (rcx))) as u64;
    rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fb390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x1401fa110; const P2: u64 = 0x48; const P3: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rcx; fl_r = rdx;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rdx;
    rbx = rcx;
    loop { rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xb8u64)) as u64; fl_l = rbx; fl_r = rsi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fb3f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401fb4d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fb560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307e970; const P1: u64 = 0x142645870; const P2: u64 = 0x1401e5730; const P3: u64 = 0x1424b16a0; const P4: u64 = 0x1401e6ff0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (0xfffffffffffffffu64) as u64; r9 = 0x1; let bit_in: u64 = rax; if bit_in != 0 { r8 = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; r13 = rcx; rax = ((r9).wrapping_shl((((r8) as u32) & 63))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1401fb594, rcx, rsp); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rax = rdx.wrapping_sub(0x1); rax = (((rax) | (r9))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rsi = r13.wrapping_add(0x18); rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); r12 = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rbx = ((r9).wrapping_shl((((rcx) as u32) & 63))) as u64; r8 = r12; rcx = rsi; rdx = ((rbx.wrapping_add(rbx)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r13.wrapping_add(0x38u64), rbx); rax = rbx.wrapping_sub(0x1); crate::simd_ops::wu64(r13.wrapping_add(0x30u64), rax); rdi = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rbx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x13u64))) as u32) as u32 as u64; r8 = rbx.wrapping_add(0x18); rdi = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 7; continue; }
            6 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru64(r8)) as u64; bb = 9; continue; }
            9 => { rcx = r8; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (((rax) & (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))))) as u64; r15 = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; r15 = ((r15).wrapping_add(rax)) as u64; rbp = (crate::simd_ops::ru64(r15)) as u64; fl_l = rbp; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(r15, rbx); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rbx); bb = 21; continue; }
            11 => { rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rcx = rbx.wrapping_add(0x10); rdx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = r8; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rdi); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, r8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rbx); bb = 20; continue; }
            15 => { fl_l = rbp; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = rbx.wrapping_add(0x10); rdx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { fl_l = rbp; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rdi); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, rsi); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(r15, rbx); bb = 20; continue; }
            20 => { rsi = r13.wrapping_add(0x18); bb = 21; continue; }
            21 => { rbx = rdi; fl_l = rdi; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 22; continue; }
            22 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 23; continue; }
            23 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            24 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rdi); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, r8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); bb = 20; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fb750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401fc690; const P1: u64 = 0x14240a730; const P2: u64 = 0x1401fc7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; rbp = ((rbp).wrapping_shr(1u32)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = rbp; fl_r = rdx; rbp = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rbp }) as u64; r12 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_l = rbp; fl_r = (0x10u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { r14 = rcx.wrapping_add(0x10); fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbp = 0x10; bb = 4; continue; }
            3 => { rdx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r14 = rax; bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r15 = (((rcx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; r15 = ((r15).wrapping_add(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rbx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); fl_l = rdi; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { r13 = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x4u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((r13 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x28u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(0x28u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x2cu64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x2cu64), rax); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x34u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x34u64), ((rax as u8)) as u8); rbx = ((rbx).wrapping_add(0x38u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rdi = ((rdi).wrapping_add(0x38u64)) as u64; fl_l = rdi; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, r14); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fb850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240a730; const P2: u64 = 0x28; const P3: u64 = 0x2c; const P4: u64 = 0x2e; const P5: u64 = 0x30; const P6: u64 = 0x1401fc280; const P7: u64 = 0x38; const P8: u64 = 0xfffffffffffffff0; const P9: u64 = 0xffffffffffffffc0; const P10: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; r14 = rdx; rsi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbp = ((rbp).wrapping_shl(6u32)) as u64; rbp = ((rbp).wrapping_add(0xffffffffffffffc0u64)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_l = r8; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rbp; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru16(rbp.wrapping_add(0x2cu64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(P3), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x2eu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P4), ((rax as u8)) as u8); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P5), rax); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rax as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (0x0u64) as u64); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rdi = ((rdi).wrapping_shl(6u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u32(rsi.wrapping_add(P0), (1) as u32); crate::simd_ops::wu64(r14, rbx); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fb910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xfffffffffffffff7; const P2: u64 = 0x14240a730; const P3: u64 = 0x28; const P4: u64 = 0x2c; const P5: u64 = 0x2e; const P6: u64 = 0x30; const P7: u64 = 0x142442de0; const P8: u64 = 0xffffffffffffffe8; const P9: u64 = 0xffffffffffffffb8; const P10: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; r15 = rdx; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = ((rsi.wrapping_add(P1)) as u64) as u64; rsi = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = rsi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru16(rsi.wrapping_add(P4))) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(P4), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P5), ((rax as u8)) as u8); rsi = ((rsi).wrapping_add(0x30u64)) as u64; r14 = ((rbx.wrapping_add(P6)) as u64) as u64; fl_l = r14; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsi, (0x0u64) as u64); rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(P0))) as u128; crate::simd_ops::wu128(r14.wrapping_add(P0), (xmm0) as u128); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rsi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P8))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rcx = ((rsi.wrapping_add(P9)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u32(rdi.wrapping_add(P0), (1) as u32); crate::simd_ops::wu64(r15, rbx); rax = r15; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fb9f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240ab10; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x108; const P4: u64 = 0x4; const P5: u64 = 0x100; const P6: u64 = 0xe; const P7: u64 = 0x20; const P8: u64 = 0x18; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
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
pub unsafe extern "win64" fn fn_1401fbac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x143a00240; const P3: u64 = 0x1426455a8; const P4: u64 = 0x142645598; const P5: u64 = 0x141859700; const P6: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; rdx = P1; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x58u64), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x18); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x18); bb = 12; continue; }
            12 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 16; continue; }
            16 => { rax = (0x69fafafafaffu64) as u64; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = P5; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0xcu64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsi, (0xdu64) as u64); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); }
            18 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fbc10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141dc0550; const P1: u64 = 0x141dc05c0; const P2: u64 = 0x143a44548; const P3: u64 = 0x1401e9210; const P4: u64 = 0x14307eae0; const P5: u64 = 0x1401fc460; const P6: u64 = 0x14307eaf0; const P7: u64 = 0x14307eac0; const P8: u64 = 0x1401fc630; const P9: u64 = 0x14307ead0; const P10: u64 = 0x1401fc680; const P11: u64 = 0x14307eab0; const P12: u64 = 0x14247f340; const P13: u64 = 0x1413c1c30; const P14: u64 = 0x14307cb10; const P15: u64 = 0x1426cfff7; const P16: u64 = 0x1401f2430; const P17: u64 = 0x1401eb150; const P18: u64 = 0x14240ab10; const P19: u64 = 0x142409b50; const P20: u64 = 0x1401f22d0; const P21: u64 = 0x14240a730; const P22: u64 = 0x1401e3b60; const P23: u64 = 0x1401e7bd0; const P24: u64 = 0x1401e77c0; const P25: u64 = 0x1401ea5b0; const P26: u64 = 0x1401fbe9a;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 51>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401fbc10, funcinfo_rva: 0x35a287c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x198, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x170u64)) as u64; rbx = (((r8 as u8)) as u32) as u32 as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbc33); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbc38); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rbx as u8)) as u8); rcx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32); r8 = (rax) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbc63); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = ((P4) as u64) as u64; r8 = (0x1u32) as u32 as u64; rdx = (rdi) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbc7b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((rdi.wrapping_add(0x48u64)) as u64) as u64; r9 = ((P6) as u64) as u64; r8 = (0x2u32) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbc94); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x1a0u64), (0x0u8) as u8); rdx = ((rdi.wrapping_add(0x90u64)) as u64) as u64; rax = ((rsp.wrapping_add(0x1a0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r15) as u64); r9 = ((P7) as u64) as u64; r8 = ((r15.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbcd0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((rdi.wrapping_add(0x98u64)) as u64) as u64; r9 = ((P9) as u64) as u64; r8 = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbcea); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsi.wrapping_add(0x40u64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), (r15) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0xb0u64), ((r15 as u32)) as u32); bb = 9; continue; }
            2 => { rbp = (rcx) as u64; r14 = (crate::simd_ops::ru64(rcx)) as u64; r12 = ((P11) as u64) as u64; rdx = (r12) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbd1f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x1eu64))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x10u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(r14)) as u64; r8 = (r12) as u64; rdx = ((rdi.wrapping_add(0xa8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbd46); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbp) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { r9 = (r12) as u64; r8 = ((P14) as u64) as u64; rdx = ((crate::simd_ops::ru8(P15)) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbd6c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsi.wrapping_add(0x20u64)) as u64) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbd78); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbd82); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), (r15) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0xb0u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0xau64) as u64); r9 = (0xau32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbdc4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x3cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = ((rsi.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbdfc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x30u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbe29); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x38u64), (1) as u32); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbe36); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rbp) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x20u64)))) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r11 = ((rsp.wrapping_add(0x170u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbe73); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbe7b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P23) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = ((rsi.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbe8c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P24) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbe95); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P25) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P26, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fbea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401febb0; const P1: u64 = 0x143079330; const P2: u64 = 0x141461fb0; const P3: u64 = 0x8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 53>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401fbea0, funcinfo_rva: 0x35a28b0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x1a8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0xa0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x1a0u64)) as u64; rbx = (rcx) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rcx = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rcx) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0xffu64) as u64); rax = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), (0x7ffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x7ffu64) as u64); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (rdx) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r9)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); r9 = ((rdi as u32)) as u32 as u64; r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fbf6b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64 as u64; fl_r = (rdx) as u64 as u64; rdi = (if (((fl_l as u64)) > ((fl_r as u64))) { rcx } else { rdi }) as u64; rcx = ((r8.wrapping_add(rdx)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdi) as u64 as u64; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdi }) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax).wrapping_add(rdi)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P3), (rcx) as u64); rax = (rbx) as u64; r11 = ((rsp.wrapping_add(0x1a0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = (r11) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fc080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1424811b0; const P1: u64 = 0x14307cb10; const P2: u64 = 0x1426cfff7; const P3: u64 = 0x1401f2430; const P4: u64 = 0x1401eb150; const P5: u64 = 0x14240ab10; const P6: u64 = 0x142409b50; const P7: u64 = 0x1401f22d0; const P8: u64 = 0x14240a730;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401fc080, funcinfo_rva: 0x35a290c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x148, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rbx = (r9) as u64; rsi = (rdx) as u64; rbp = (rcx) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x180u64))) as u64; xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); bb = 11; continue; }
            2 => { r8 = (rsi) as u64; rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc0cb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { r9 = (rbx) as u64; r8 = ((P1) as u64) as u64; rdx = ((crate::simd_ops::ru8(P2)) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc0ee); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x20u64)) as u64) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc0fa); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc104); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x180u64))) as u64; xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; crate::simd_ops::wu64(rsi, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x10000010u64) as u64); rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rsi as u8)) as u8); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rbx as u8)) as u8); bb = 8; continue; }
            5 => { r9 = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc166); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x3cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = ((rbp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc19e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x30u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc1ca); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x38u64), (1) as u32); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc1d7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { r11 = ((rsp.wrapping_add(0x140u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fc300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fc320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14183acc8; const P5: u64 = 0x1401de830;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; fl_l = rsi; fl_r = rdx; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rsi }) as u64; rdi = 0; fl_l = 0; fl_r = 0; rdx = rsi; rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rax); bb = 13; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = rdi; bb = 13; continue; }
            7 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            11 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { r8 = rax; bb = 13; continue; }
            13 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            15 => { rdi = r8; bb = 16; continue; }
            16 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = rdi; rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbx, rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fc460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cdcb0; const P1: u64 = 0x142409b50; const P2: u64 = 0x14240aa00; const P3: u64 = 0x142481430; const P4: u64 = 0x14307cb10; const P5: u64 = 0x1426cfff7; const P6: u64 = 0x1401f2430; const P7: u64 = 0x1401eb150; const P8: u64 = 0x14240ab10; const P9: u64 = 0x1401f22d0; const P10: u64 = 0x14240a730;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 49>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401fc460, funcinfo_rva: 0x35a296c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x188, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x180u64)) as u64; rdi = (r9) as u64; rbx = ((r8 as u32)) as u32 as u64; rsi = (rdx) as u64; rbp = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x8000008u64) as u64); r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r14 as u8)) as u8); rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r14) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc4bb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc4ce); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            2 => { r8 = (rsi) as u64; rdx = (rdi) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc4e2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { r9 = (rdi) as u64; r8 = ((P4) as u64) as u64; rdx = ((crate::simd_ops::ru8(P5)) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc505); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rbp.wrapping_add(0x20u64)) as u64) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc511); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc51b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc528); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), (0x0u8) as u8); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((rdi as u8)) as u8); bb = 8; continue; }
            5 => { r9 = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc581); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x3cu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = ((rbp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc5b9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x30u64)))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc5e6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x38u64), (1) as u32); rcx = ((rsp.wrapping_add(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc5f3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc5fe); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r11 = ((rsp.wrapping_add(0x180u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fc630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x54; const P1: u64 = 0x20; const P2: u64 = 0x40; const P3: u64 = 0x1401fd120;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    r11 = r9;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64;
    r9 = rdx;
    r10 = rcx;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rdx, ((rcx as u8)) as u8); rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
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
pub unsafe extern "win64" fn fn_1401fc7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240ab10; const P2: u64 = 0x10; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x108; const P5: u64 = 0x4; const P6: u64 = 0x100; const P7: u64 = 0xe; const P8: u64 = 0x20; const P9: u64 = 0x18; const P10: u64 = 0x1425371a0; const P11: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x38u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { rax = ((rdi.wrapping_add(P2)) as u64) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { rcx = ((r9.wrapping_add(P3)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P4)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P9), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P8), r9); if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = rdx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            8 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            9 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi.wrapping_add(0x10); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), (0x10u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401fc890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401fce60; const P1: u64 = 0x1401fcd50; const P2: u64 = 0x14309f824; const P3: u64 = 0x14183acc2; const P4: u64 = 0x1401fe1d0; const P5: u64 = 0x1401ee280; const P6: u64 = 0x1401fd260; const P7: u64 = 0x14307f0b0; const P8: u64 = 0x1426cfff7; const P9: u64 = 0x1401f2430; const P10: u64 = 0x1401eb150; const P11: u64 = 0x14240ab10; const P12: u64 = 0x1401fdd10; const P13: u64 = 0x142409b50; const P14: u64 = 0x1401f22d0; const P15: u64 = 0x14240a730; const P16: u64 = 0x1401fa110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 121>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401fc890, funcinfo_rva: 0x35a29bc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x3c8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x290u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x390u64)) as u64; r14 = (r9) as u64; r12 = (rdx) as u64; rdi = (rcx) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (r9) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc8dc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = (rsi) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc8f4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { r15 = (((rbx) as i64).wrapping_mul((0xb8u64) as i64) as u64) as u64; r13 = (rsi) as u64; r13 = ((r13).wrapping_sub(rbx)) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = ((rsi.wrapping_sub(0x1u64)) as u64) as u64; r14 = ((P2) as u64) as u64; bb = 4; continue; }
            4 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0x310u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc92d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rcx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), (0x10u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), (0x6u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), (rsi) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x280u64), (rsi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), (rsi) as u64); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x60u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); rax = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), (r14) as u64); crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), (rsi) as u64); crate::simd_ops::wu32(rbx.wrapping_sub(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), (rsi) as u64); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x270u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc9bc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_sub(0x18u64), (rax) as u64); bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rbp.wrapping_add(0x288u64), ((rsi as u32)) as u32); rcx = (crate::simd_ops::ru64(r12)) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc9db); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fc9e6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = ((r15).wrapping_add(0xb8u64)) as u64; fl_l = (r13) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r13 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 8; continue; }
            8 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x2e8u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x2d0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2e0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = (r14) as u64; r8 = (rsi) as u64; rdx = (crate::simd_ops::ru64(r12)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fca27); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            10 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2d0u64), (rax) as u64); r9 = (r14) as u64; r8 = ((P7) as u64) as u64; rdx = ((crate::simd_ops::ru8(P8)) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fca5f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = ((rdi.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fca6b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fca75); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fca82); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rdi.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((rsi as u8)) as u8); fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((r14 as u8)) as u8); bb = 15; continue; }
            12 => { rdi = (0xffffffffffffffffu64) as u64; bb = 13; continue; }
            13 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdi) as u64); r9 = (rdi) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fcad4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fcb0b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fcb36); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fcb43); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401fcb4e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x3d8u64))) as u64; rsp = ((rsp).wrapping_add(0x390u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401ff210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x78; const P2: u64 = 0x1401f2bc0; const P3: u64 = 0x68; const P4: u64 = 0x58; const P5: u64 = 0x1401ffa20; const P6: u64 = 0x48; const P7: u64 = 0x1401de830; const P8: u64 = 0x30; const P9: u64 = 0x1401ffd50;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x8cu64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P8)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401ff270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ff310; const P1: u64 = 0x58; const P2: u64 = 0x1401ff760; const P3: u64 = 0x68; const P4: u64 = 0x1401ff5b0; const P5: u64 = 0x78; const P6: u64 = 0x1401ff410; const P7: u64 = 0x88; const P8: u64 = 0x8c; const P9: u64 = 0x3; const P10: u64 = 0x4; const P11: u64 = 0x8; const P12: u64 = 0x14240a730;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rdx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rdx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(P8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000005u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x5u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P9), (0x5u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P10), (0x0u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P11), (0x0u8) as u8); rdx = rbx.wrapping_add(0x8c); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xbcu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xbcu64), ((rax as u8)) as u8); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401ff310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x14240a730; const P4: u64 = 0x30; const P5: u64 = 0x38; const P6: u64 = 0x3c; const P7: u64 = 0x40; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x54;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::lk_and_u32(rcx, (0xff000005u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x5u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x5u8) as u8); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rsi as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; crate::simd_ops::wu128(rdi.wrapping_add(P4), (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(P4), rsi); crate::simd_ops::wu64(rdi.wrapping_add(P5), rsi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P4), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P4), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P5), rsi); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); crate::simd_ops::wu128(rdi.wrapping_add(P8), (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(P8), rsi); crate::simd_ops::wu64(rdi.wrapping_add(P9), rsi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P8), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P8), rsi); crate::simd_ops::wu64(rbx.wrapping_add(P9), rsi); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401ff3c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de830; const P1: u64 = 0x30; const P2: u64 = 0x1401ffd50; const P3: u64 = 0x14240ab10;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x48u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401ff410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401decf0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1425371a0; const P3: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, r14); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r14); fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rcx, r14); rcx = ((r14 as u32)) as u32 as u64; rax = ((r14 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xc0000000u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 19; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rbp = ((r14 as u32)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 3; continue; }
            3 => { rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((rax as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rcx = r9.wrapping_sub(0x1); rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rcx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { rcx = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            13 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx, rbp); fl_l = ((((rsi as u32)) & (0x3fffffffu32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rsi as u32)) as u32); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { r8 = ((rax as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 20; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) ^ (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); rcx = ((rax as u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))))) as u32 as u64; rcx = ((((rcx as u32)) & (0x40000000u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rcx as u32)) as u32); rcx = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))))) as u32 as u64; fl_l = ((((rcx as u32)) >> (31u32)) & 1) as u64; rcx = (((rcx as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rdx, r14); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r14); bb = 20; continue; }
            20 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401ff5b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401dec00; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1425371a0; const P3: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, r14); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r14); fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rcx, r14); rcx = ((r14 as u32)) as u32 as u64; rax = ((r14 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xc0000000u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 19; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rbp = ((r14 as u32)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 3; continue; }
            3 => { rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((rax as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rcx = r9.wrapping_sub(0x1); rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rcx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { rcx = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            13 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx, rbp); fl_l = ((((rsi as u32)) & (0x3fffffffu32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rsi as u32)) as u32); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { r8 = ((rax as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 20; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) ^ (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); rcx = ((rax as u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))))) as u32 as u64; rcx = ((((rcx as u32)) & (0x40000000u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rcx as u32)) as u32); rcx = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))))) as u32 as u64; fl_l = ((((rcx as u32)) >> (31u32)) & 1) as u64; rcx = (((rcx as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rdx, r14); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r14); bb = 20; continue; }
            20 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401ff760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ffbc0; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x1401ffc30; const P4: u64 = 0x1401ffe70; const P5: u64 = 0x1401f2bc0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbx); fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((((rcx as u32)) >> (30u32)) & 1) as u64; if ((fl_l as u32) != 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { r10 = r9.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            7 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rbx); fl_l = (crate::simd_ops::lk_and_u32(rdi.wrapping_add(0xcu64), (0xc0000000u32) as u32)) as u32 as u64; fl_r = fl_l; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 35; continue; } bb = 12; continue; }
            12 => { r15 = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 31; continue; } bb = 13; continue; }
            13 => { rdx = ((r15 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r13 = rax; rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 14; continue; }
            14 => { rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rsi = rax.wrapping_add(0x18); bb = 16; continue; }
            16 => { rcx = rsi.wrapping_sub(0x18); rdx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_sub(0x8u64), rax); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi, ((rax as u8)) as u8); rsi = rsi.wrapping_add(0x20); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = (((rbp as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rbp = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 30; continue; } bb = 22; continue; }
            22 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 23; continue; }
            23 => { r10 = rbx.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rbx; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rbx); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            27 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 29; continue; }
            28 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            29 => { rdx = rcx; r9 = rbx; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rdi, r13); fl_l = ((((r15 as u32)) & (0x3fffffffu32))) as u32 as u64; fl_r = fl_l; r15 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r15 as u32)) as u32); bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rsi = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(r14)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 34; continue; } bb = 32; continue; }
            32 => { rbx = rax.wrapping_add(0x18); fl_l = rbp; fl_r = rax; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 33; continue; }
            33 => { rdx = rbp.wrapping_sub(0x18); rdx = ((rdx).wrapping_add(rbx)) as u64; rcx = rbx.wrapping_sub(0x18); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(rbp).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), rax); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(rbp))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx, ((rax as u8)) as u8); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = (((rsi as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 33; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = r14; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 36; continue; }
            35 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rdi, rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) ^ (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); rdx = ((rax as u32)) as u32 as u64; rdx = ((((rdx as u32)) ^ (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))))) as u32 as u64; rdx = ((((rdx as u32)) & (0x40000000u32))) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; fl_l = ((((rcx as u32)) >> (31u32)) & 1) as u64; rcx = (((rcx as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((((rcx as u32)) ^ (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rcx as u32)) as u32); crate::simd_ops::wu64(r14, rbx); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rbx); bb = 36; continue; }
            36 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401ffad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x1401decf0; const P2: u64 = 0x8; const P3: u64 = 0x14183acc8; const P4: u64 = 0xffffffffffffffff; const P5: u64 = 0x108; const P6: u64 = 0x4; const P7: u64 = 0x100; const P8: u64 = 0xe; const P9: u64 = 0x20; const P10: u64 = 0x18; const P11: u64 = 0x1425371a0; const P12: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rdx; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; rbx = rcx; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rsi = rax; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = ((rcx as u32)) as u32 as u64; rcx = rax; r8 = ((r8).wrapping_shl(2u32)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rcx = ((r9.wrapping_add(P4)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P5)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P10), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P9), r9); if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = rdx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            10 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            11 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { fl_l = ((((rdi as u32)) & (0x3fffffffu32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdi as u32)) as u32); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401ffe00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rbx = rdx; rbp = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx, rbp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402001e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = 0xffffffff;
    rbx = rcx;
    rax = ((rdi as u32)) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rdi = (xa_old) as u32 as u64;
    fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1402050d0; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rax = r8;
    rcx = rsp.wrapping_add(0x28);
    r8 = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rbx = rdx;
    xmm0 = 0u128; ymm0_hi = 0;
    xmm1 = 0u128;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    let y_addr: u64 = rsp.wrapping_add(0x28u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64);
    rax = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), (0x0u64) as u64);
    let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x5u8) as u8);
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402002c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1402050d0; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    xmm0 = 0u128; ymm0_hi = 0;
    xmm1 = 0u128;
    rax = r8;
    rbx = rdx;
    crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm1) as u128);
    r8 = (0xffffffffffffffffu64) as u64;
    loop { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    rdx = rax;
    rcx = rsp.wrapping_add(0x28);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    let y_addr: u64 = rsp.wrapping_add(0x28u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64);
    rax = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64);
    let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x5u8) as u8);
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x141834d30; const P2: u64 = 0x1426467f0; const P3: u64 = 0x140200425;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140200340, funcinfo_rva: 0x35a2f90, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx, ((rdi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rdi) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rdi) as u64); rcx = ((rdi.wrapping_add(0x58u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140200366); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (0x7u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (0x8u64) as u64); crate::simd_ops::wu32(rbx, (0x3f800000u32) as u32); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rdi as u32)) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rcx = (0x80u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402003af); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_mul(8)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = (r8) as u64; bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402003f4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rdi) as u64); rax = ((rdi.wrapping_add(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (rax) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdi, (rsi) as u64); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { bb = 12; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020041f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            10 => { rcx = ((rcx).wrapping_add(0x7u64)) as u64; rcx = ((rcx).wrapping_shr(3u32)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (rsi) as u64; while rcx != 0 { core::ptr::write_unaligned(rdi as *mut u64, rax); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); } bb = 12; continue; }
            12 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140202a90; const P1: u64 = 0x8; const P2: u64 = 0x140205280; const P3: u64 = 0x141834d30;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rdx = 0x58;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x142646950;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_r = (0xfu64) as u64; rbx = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(r8)) as u64; rbx = rbp; fl_l = ((rbp).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(P1)))) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 3; continue; }
            2 => { rbp = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = ((rbp).wrapping_add(rbx)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r15 = (crate::simd_ops::ru64(rdx)) as u64; r14 = r15; fl_l = ((r15).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P1)))) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 6; continue; }
            5 => { r15 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; r14 = rdx; fl_l = ((r15).wrapping_add(rdx)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); fl_l = r14; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 8; continue; }
            8 => { rcx = (((crate::simd_ops::ru8(r14)) as i8 as i32) as u32) as u32 as u64; rsi = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi as u32)) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 9; continue; }
            9 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rsi = (((crate::simd_ops::ru8(r14)) as i8 as i32) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi as u32)) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 10; continue; }
            10 => { r14 = ((r14).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 11; continue; }
            11 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            13 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d1d8; const P1: u64 = 0x143c8d190; const P2: u64 = 0x143c8d188; const P3: u64 = 0x141834a78; const P4: u64 = 0x141834d64; const P5: u64 = 0x14266b520; const P6: u64 = 0x1425d59b0; const P7: u64 = 0x141834d18; const P8: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140200590, funcinfo_rva: 0x35a2fdc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (rcx) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x74u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdi, (rsi) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rsi) as u64); rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rcx) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = ((P0) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020060b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 5; continue; }
            5 => { rcx = (0x8u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020061e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); rcx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140200630); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rax, (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbx, (rax) as u64); xmm0 = 0u128; crate::simd_ops::wu128(P2, (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); rcx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140200659); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x1u32) as u32); rdx = ((P5) as u64) as u64; crate::simd_ops::wu64(rax, (rdx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(P2, (rbx) as u64); crate::simd_ops::wu64(P1, (rax) as u64); rcx = ((P6) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020069c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P0) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402006a9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 1; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402006c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x143c8d1d8; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x143c8d190; const P6: u64 = 0x8; const P7: u64 = 0x143c8d188; const P8: u64 = 0x141834a78; const P9: u64 = 0x141834d64; const P10: u64 = 0xc; const P11: u64 = 0x14266b520; const P12: u64 = 0x1425d59b0; const P13: u64 = 0x141834d18; const P14: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; xmm0 = 0u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); crate::simd_ops::wu16(rcx.wrapping_add(P1), ((rax as u16)) as u16); rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdi.wrapping_add(P3), rsi); crate::simd_ops::wu64(rdi.wrapping_add(P4), rsi); rax = (crate::simd_ops::ru64(P5)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P6), (1) as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(P7)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P3), rax); rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P4), rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 5; continue; }
            5 => { rcx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; rcx = 0x18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax, rsi); crate::simd_ops::wu64(rax.wrapping_add(P6), rsi); crate::simd_ops::wu64(rax.wrapping_add(P0), rsi); crate::simd_ops::wu64(rbx, rax); xmm0 = 0u128; crate::simd_ops::wu128(P7, (xmm0) as u128); rcx = 0x18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax.wrapping_add(P6), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P10), (0x1u32) as u32); rdx = P11; crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(P7, rbx); crate::simd_ops::wu64(P5, rax); rcx = P12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645868; const P1: u64 = 0x1401f2f00; const P2: u64 = 0x1402066f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = 0x1; rbx = rcx; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rcx, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = (0x7ffffffffff0bdbfu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rdi = (0x7fffffffffffffffu64) as u64; bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx, ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x64u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rsi; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax).wrapping_add(0xf4240u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); bb = 11; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); bb = 11; continue; }
            11 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acda; const P1: u64 = 0x140000000;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; r14 = (r8) as u64; rdi = (rdx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x0u8) as u8); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rdi) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rcx = ((P1) as u64) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0xffu8) as u8); rax = (((crate::simd_ops::ru8(rbx.wrapping_add(0x30u64))) as i8 as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x200a54u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = (rax) as u64; bb = 0xffff_fffe; continue; }
            8 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi, ((rax as u8)) as u8); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x1u8) as u8); bb = 6; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x2u8) as u8); bb = 6; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x3u8) as u8); bb = 6; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x4u8) as u8); bb = 6; continue; }
            12 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x5u8) as u8); bb = 6; continue; }
            15 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x6u8) as u8); bb = 6; continue; }
            18 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), (0x7u8) as u8); bb = 6; continue; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x140200910u64) { 0x23 => { bb = 1; continue; } 0x27 => { bb = 2; continue; } 0x30 => { bb = 3; continue; } 0x3f => { bb = 4; continue; } 0x48 => { bb = 5; continue; } 0x4c => { bb = 6; continue; } 0x65 => { bb = 7; continue; } 0x81 => { bb = 8; continue; } 0x8d => { bb = 9; continue; } 0x9a => { bb = 10; continue; } 0xa7 => { bb = 11; continue; } 0xb4 => { bb = 12; continue; } 0xc6 => { bb = 13; continue; } 0xca => { bb = 14; continue; } 0xe2 => { bb = 15; continue; } 0xf4 => { bb = 16; continue; } 0xf8 => { bb = 17; continue; } 0x110 => { bb = 18; continue; } 0x122 => { bb = 19; continue; } 0x126 => { bb = 20; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834fb8; const P1: u64 = 0x14183acbc; const P2: u64 = 0x140000000; const P3: u64 = 0x140203460; const P4: u64 = 0x1402077f0; const P5: u64 = 0x140207940; const P6: u64 = 0x140207a90;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (r8) as u64; rsi = (rdx) as u64; r15 = (rcx) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rbp) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rbp) as u64); rbx = ((rcx.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), ((rbp as u8)) as u8); rdi = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(r15, (rax) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x18u64), (rdi) as u64); r8 = (rdi) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); rax = (((crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as i8 as i64) as u64) as u64; rcx = ((P2) as u64) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x200c4cu64))) as u32 as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = (rax) as u64; bb = 0xffff_fffe; continue; }
            1 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), (0xffu8) as u8); bb = 19; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 3; continue; }
            3 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), (0x0u8) as u8); bb = 19; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; crate::simd_ops::wu8(rbx, ((rax as u8)) as u8); bb = 19; continue; }
            6 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), (0xffu8) as u8); rax = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; crate::simd_ops::wu8(rbx, ((rax as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), (0x1u8) as u8); bb = 19; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); bb = 19; continue; }
            9 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), (0xffu8) as u8); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), (0x2u8) as u8); bb = 19; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); bb = 19; continue; }
            12 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), (0xffu8) as u8); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), (0x3u8) as u8); bb = 19; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); bb = 19; continue; }
            15 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), (0xffu8) as u8); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), (0x4u8) as u8); bb = 19; continue; }
            16 => { rdx = (r14) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 19; continue; }
            17 => { rdx = (r14) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 19; continue; }
            18 => { rdx = (r14) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 19; continue; }
            19 => { crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (rbp) as u64); rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), (rbp) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { rdi = (0xffffffffu32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 23; continue; }
            23 => { rax = (r15) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x140200a80u64) { 0x76 => { bb = 1; continue; } 0x87 => { bb = 2; continue; } 0x91 => { bb = 3; continue; } 0xa2 => { bb = 4; continue; } 0xa8 => { bb = 5; continue; } 0xb3 => { bb = 6; continue; } 0xce => { bb = 7; continue; } 0xd4 => { bb = 8; continue; } 0xdf => { bb = 9; continue; } 0xf7 => { bb = 10; continue; } 0xfd => { bb = 11; continue; } 0x105 => { bb = 12; continue; } 0x11d => { bb = 13; continue; } 0x123 => { bb = 14; continue; } 0x12b => { bb = 15; continue; } 0x143 => { bb = 16; continue; } 0x152 => { bb = 17; continue; } 0x161 => { bb = 18; continue; } 0x16e => { bb = 19; continue; } 0x17f => { bb = 20; continue; } 0x190 => { bb = 21; continue; } 0x1a2 => { bb = 22; continue; } 0x1ac => { bb = 23; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = P0; crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0xffu8) as u8); rax = (((crate::simd_ops::ru8(rdx.wrapping_add(0x10u64))) as i8 as i64) as u64) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x200d64u64))) as u32 as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = rax; bb = 0xffff_fffe; continue; }
            1 => { crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x0u8) as u8); rax = rcx; return rax; }
            2 => { rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rax = rcx; crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x1u8) as u8); return rax; }
            3 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = rcx; crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x2u8) as u8); return rax; }
            4 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = rcx; crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x3u8) as u8); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = rcx; crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x4u8) as u8); return rax; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = rcx; crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x5u8) as u8); return rax; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = rcx; crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x6u8) as u8); return rax; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x7u8) as u8); bb = 15; continue; }
            15 => { rax = rcx; return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x140200c80u64) { 0x1d => { bb = 1; continue; } 0x25 => { bb = 2; continue; } 0x32 => { bb = 3; continue; } 0x40 => { bb = 4; continue; } 0x4e => { bb = 5; continue; } 0x5c => { bb = 6; continue; } 0x6e => { bb = 7; continue; } 0x72 => { bb = 8; continue; } 0x88 => { bb = 9; continue; } 0x9a => { bb = 10; continue; } 0x9e => { bb = 11; continue; } 0xb4 => { bb = 12; continue; } 0xc6 => { bb = 13; continue; } 0xca => { bb = 14; continue; } 0xdc => { bb = 15; continue; } _ => { return crate::simd_ops::call_fwd(j_tgt, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x14266a3d0; const P2: u64 = 0x8; const P3: u64 = 0x18; const P4: u64 = 0xc; const P5: u64 = 0x14266ad30; const P6: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rcx = 0x8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    rax = P1;
    crate::simd_ops::wu64(rbx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdi, rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rax);
    rcx = ((rax.wrapping_add(P3)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rax.wrapping_add(P2), (0x1u32) as u32);
    crate::simd_ops::wu32(rax.wrapping_add(P4), (0x1u32) as u32);
    rcx = P5;
    crate::simd_ops::wu64(rax, rcx);
    crate::simd_ops::wu64(rax.wrapping_add(P6), rbx);
    crate::simd_ops::wu64(rdi, rbx);
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rax);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P0), (1) as u32)) as u32 as u64; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140200fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = ((rdx as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = 0xffffffff; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = 0x18; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140201120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d1d0; const P1: u64 = 0x143c8d1e0; const P2: u64 = 0x141834a78; const P3: u64 = 0x141834d64; const P4: u64 = 0x143080800; const P5: u64 = 0x1430808f0; const P6: u64 = 0x8; const P7: u64 = 0x10; const P8: u64 = 0x18; const P9: u64 = 0xc; const P10: u64 = 0x143080750; const P11: u64 = 0x143080858; const P12: u64 = 0x143c8d1e8; const P13: u64 = 0x1425d5a00; const P14: u64 = 0x141834d18; const P15: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = P1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = 0x18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rax = P4; crate::simd_ops::wu64(rdi, rax); rcx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; rax = P5; crate::simd_ops::wu64(rbx, rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(P6), rax); crate::simd_ops::wu64(rdi.wrapping_add(P7), rax); rcx = ((rax.wrapping_add(P8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax.wrapping_add(P6), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P9), (0x1u32) as u32); rcx = P10; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(P7), rbx); crate::simd_ops::wu64(rdi.wrapping_add(P6), rbx); crate::simd_ops::wu64(rdi.wrapping_add(P7), rax); xmm0 = 0u128; crate::simd_ops::wu128(P1, (xmm0) as u128); rcx = 0x18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax.wrapping_add(P6), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P9), (0x1u32) as u32); rdx = P11; crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(P1, rdi); crate::simd_ops::wu64(P12, rax); rcx = P13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402012a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140203690; const P1: u64 = 0x18; const P2: u64 = 0x140202a90; const P3: u64 = 0x8; const P4: u64 = 0x140205280; const P5: u64 = 0x141834d30;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x48u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rdx = 0x58;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140201310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d198; const P1: u64 = 0x143c8d1a8; const P2: u64 = 0x141834a78; const P3: u64 = 0x141834d64; const P4: u64 = 0x142670ec8; const P5: u64 = 0x8; const P6: u64 = 0xc; const P7: u64 = 0x142670f78; const P8: u64 = 0x10; const P9: u64 = 0x143c8d1b0; const P10: u64 = 0x1425d5a50; const P11: u64 = 0x141834d18; const P12: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = P1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; rax = P4; crate::simd_ops::wu64(rbx, rax); xmm0 = 0u128; crate::simd_ops::wu128(P1, (xmm0) as u128); rcx = 0x18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P6), (0x1u32) as u32); rdx = P7; crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rax.wrapping_add(P8), rbx); crate::simd_ops::wu64(P1, rbx); crate::simd_ops::wu64(P9, rax); rcx = P10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402013e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1402018f0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0xfffffffffffffff0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx, rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P1), (1) as u32)) as u32 as u64; }
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rbx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140201440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1402018f0; const P1: u64 = 0x140201710; const P2: u64 = 0x1425d5aa0; const P3: u64 = 0x141834e48; const P4: u64 = 0x140202f00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r12 = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax)) as u64; r13 = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((r13 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(r12)) as u64; bb = 8; continue; }
            2 => { rcx = ((rcx).wrapping_add(rcx)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(r12)) as u64; rcx = rdi; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; bb = 8; continue; }
            8 => { fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 15; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru64(r12)) as u64; rcx = rdx; rcx = ((rcx).wrapping_shl(4u32)) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; r8 = ((r8).wrapping_add(0xfffffffffffffff0u64)) as u64; fl_l = ((r8).wrapping_add(rcx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { fl_l = r9; fl_r = (crate::simd_ops::ru64(r8)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { r8 = ((r8).wrapping_sub(0x10u64)) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; r14 = 0xa3564; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rax))) as u64; rdi = 0xa3568; fl_l = ((rdi).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; r15 = rdi; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); crate::simd_ops::wu64(r15, r13); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), r13); crate::simd_ops::wu64(r15.wrapping_add(0x10u64), r13); rcx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = r15; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = r13; rbp = (crate::simd_ops::ru64(r12)) as u64; bb = 27; continue; }
            21 => { rcx = ((rcx).wrapping_add(rcx)) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(rsi.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r12)) as u64; rcx = rsi; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; bb = 27; continue; }
            27 => { fl_l = rbp; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); crate::simd_ops::wu64(rdi, r13); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r13); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), r13); rcx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 31; continue; }
            31 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff0u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 32; continue; }
            32 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::lk_sub_u64(rdi, (1) as u64)) as u64; bb = 17; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402016a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1402018f0; const P1: u64 = 0x1402017a0; const P2: u64 = 0x141834d64; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = rbx;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = 0x10;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P3), (1) as u32)) as u32 as u64; }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdi, rcx);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140201710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x140202f00; const P2: u64 = 0x8; const P3: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rdx = rsp.wrapping_add(0x20); rax = ((rax).wrapping_add(0xfffffffffffffff0u64)) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; xmm0 = 0u128; rcx = ((rcx).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = 0xffffffff; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_sub_u64(rdi, (1) as u64)) as u64; bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402017a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834fb8; const P1: u64 = 0x140200d90; const P2: u64 = 0x1402013d0; const P3: u64 = 0x141834fc0; const P4: u64 = 0x140202f80; const P5: u64 = 0x141834ee8; const P6: u64 = 0x141834ee0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r13 = (rdx) as u64; rdi = (rcx) as u64; r14 = (crate::simd_ops::ru64(rcx)) as u64; rbp = ((r14.wrapping_add(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rcx, (rbp) as u64); fl_l = (rbp) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbp).wrapping_add(rbp)) as u64 as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; rcx = (rbp) as u64; rbx = (rbp) as u64; r12 = (rbp) as u64; if (((fl_l as u64)) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbp = (0x2u32) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; rbx = ((rbp as u32)) as u32 as u64; r12 = ((rbp as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (0x10u32) as u32 as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rcx = (0xffffffffffffffffu64) as u64; rax = (if (fl_l != 0) { rcx } else { rax }) as u64; let ad_s: u64 = 0x8u64; fl_r = ad_s as u64; fl_l = (rax).wrapping_add(ad_s) as u64; rax = ((fl_l as u64)) as u64; rax = (if ((fl_l as u64) < (fl_r as u64)) { rcx } else { rax }) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rax, (rbp) as u64); r15 = ((rax.wrapping_add(0x8u64)) as u64) as u64; rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((P2) as u64) as u64; r8 = (rbp) as u64; rdx = (0x10u32) as u32 as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { r12 } else { r14 }) as u64; fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rbx = ((rsi as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rcx = ((rbx.wrapping_add(r15)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = ((rsi).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rbx = ((rcx.wrapping_sub(0x8u64)) as u64) as u64; r9 = ((P1) as u64) as u64; r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (r15) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rbp) as u64); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff0u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = (r13) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402018f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425d5aa0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x141834e48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rdx = 0xa3564;
    r8 = 0xa3568;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rbx = ((rcx.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; rcx = P0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = ((rcx.wrapping_add(r8)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140201950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x21; const P5: u64 = 0x19; const P6: u64 = 0x28; const P7: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(P1), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(P5), ((rax as u8)) as u8);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P0), (1) as u32)) as u32 as u64; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402019a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14266b498; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x19; const P6: u64 = 0x21; const P7: u64 = 0x28; const P8: u64 = 0x30; const P9: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rdi = rcx; crate::simd_ops::wu64(rcx, rax); xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P4), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P6), ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); crate::simd_ops::wu64(rcx.wrapping_add(P8), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P1), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P8), rax); rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = 0xffffffff; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P9), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140201a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = ((rdx as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = 0xffffffff; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = 0x38; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140201ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140201310; const P1: u64 = 0x1426e0ee0; const P2: u64 = 0x141834d64; const P3: u64 = 0x140200a80; const P4: u64 = 0x1430808c8; const P5: u64 = 0x140203460; const P6: u64 = 0x140202f80; const P7: u64 = 0x140000000; const P8: u64 = 0x1402001e0; const P9: u64 = 0x140200d90;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = rdx; rbp = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (0xffffffffffffffffu64) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); rax = rsi; bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x5u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); rcx = 0x38; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); r8 = rsp.wrapping_add(0x60); rdx = rsp.wrapping_add(0x50); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; rcx = 0x18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x1u32) as u32); rcx = P4; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x6; rcx = P7; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x201d08u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = rax; bb = 0xffff_fffe; continue; }
            17 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsp.wrapping_add(0x20); rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp; r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x140201ae0u64) { 0x2b => { bb = 1; continue; } 0x30 => { bb = 2; continue; } 0x4d => { bb = 3; continue; } 0x59 => { bb = 4; continue; } 0x6d => { bb = 5; continue; } 0x76 => { bb = 6; continue; } 0x7f => { bb = 7; continue; } 0x83 => { bb = 8; continue; } 0xa0 => { bb = 9; continue; } 0xa9 => { bb = 10; continue; } 0xb3 => { bb = 11; continue; } 0xb7 => { bb = 12; continue; } 0x134 => { bb = 13; continue; } 0x140 => { bb = 14; continue; } 0x154 => { bb = 15; continue; } 0x15d => { bb = 16; continue; } 0x193 => { bb = 17; continue; } 0x198 => { bb = 18; continue; } 0x1a4 => { bb = 19; continue; } 0x1b6 => { bb = 20; continue; } 0x1bf => { bb = 21; continue; } 0x1cd => { bb = 22; continue; } 0x1d1 => { bb = 23; continue; } 0x1eb => { bb = 24; continue; } 0x1f3 => { bb = 25; continue; } 0x1fd => { bb = 26; continue; } 0x202 => { bb = 27; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140201d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080880; const P1: u64 = 0x143080818; const P2: u64 = 0x8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r11 = rsp;
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x80u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r11.wrapping_sub(0x60u64), r9);
    crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rax);
    r9 = r11.wrapping_sub(0x28);
    crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), rax);
    r8 = r11.wrapping_sub(0x18);
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax);
    rbx = rdx;
    crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), rax);
    rdx = r11.wrapping_sub(0x38);
    rax = P0;
    crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), rdx);
    crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rax);
    rdx = rbx;
    rax = r11.wrapping_sub(0x58);
    crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), rax);
    rax = P1;
    crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rax);
    rax = r11.wrapping_sub(0x48);
    crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x80u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140201dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426e0ee0; const P1: u64 = 0x140201310; const P2: u64 = 0x14183acda; const P3: u64 = 0x140000000; const P4: u64 = 0x140200c80; const P5: u64 = 0x141834d64; const P6: u64 = 0x1402006c0; const P7: u64 = 0x14266b498; const P8: u64 = 0x14266b548;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r15 = (rcx) as u64; r12 = ((P0) as u64) as u64; rdi = (0xffffffffffffffffu64) as u64; rsi = (rdi) as u64; bb = 1; continue; }
            1 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r12.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbp.wrapping_sub(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x40u64), (0x0u8) as u8); bb = 13; continue; }
            13 => { fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(r14.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(r14.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x40u64))) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 18; continue; }
            18 => { r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r15, (r14) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (r14) as u64); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; crate::simd_ops::wu64(r15, (rax) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (rbx) as u64); rax = (((crate::simd_ops::ru8(rbp.wrapping_sub(0x40u64))) as i8 as i64) as u64) as u64; rcx = ((P3) as u64) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x202068u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = (rax) as u64; bb = 0xffff_fffe; continue; }
            21 => { rdx = ((rbx.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            22 => { rcx = (0x38u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; rcx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x38u64))) as u128; crate::simd_ops::wu128(rsi.wrapping_add(0x8u64), (xmm0) as u128); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), (rcx) as u64); rcx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x20u64), ((rcx as u8)) as u8); rcx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x1fu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x21u64), ((rcx as u8)) as u8); r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), (r14) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), (r14) as u64); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), (rcx) as u64); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 28; continue; }
            28 => { crate::simd_ops::wu64(r15, (r14) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (r14) as u64); rcx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x1u32) as u32); rcx = ((P8) as u64) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(r15, (rsi) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (rax) as u64); rax = (((crate::simd_ops::ru8(rbp.wrapping_sub(0x40u64))) as i8 as i64) as u64) as u64; rcx = ((P3) as u64) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x20208cu64))) as u32 as u64; fl_l = ((rax).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = (rax) as u64; bb = 0xffff_fffe; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; bb = 30; continue; }
            30 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 31; continue; }
            31 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 34; continue; }
            34 => { rax = (r15) as u64; r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x140201dd0u64) { 0x40 => { bb = 1; continue; } 0x4a => { bb = 2; continue; } 0x58 => { bb = 3; continue; } 0x5c => { bb = 4; continue; } 0x71 => { bb = 5; continue; } 0x7d => { bb = 6; continue; } 0x91 => { bb = 7; continue; } 0x9a => { bb = 8; continue; } 0xa7 => { bb = 9; continue; } 0xb0 => { bb = 10; continue; } 0xc3 => { bb = 11; continue; } 0xcc => { bb = 12; continue; } 0xd0 => { bb = 13; continue; } 0xd5 => { bb = 14; continue; } 0xe2 => { bb = 15; continue; } 0xf7 => { bb = 16; continue; } 0x100 => { bb = 17; continue; } 0x106 => { bb = 18; continue; } 0x119 => { bb = 19; continue; } 0x121 => { bb = 20; continue; } 0x144 => { bb = 21; continue; } 0x156 => { bb = 22; continue; } 0x1aa => { bb = 23; continue; } 0x1b2 => { bb = 24; continue; } 0x1c3 => { bb = 25; continue; } 0x1cf => { bb = 26; continue; } 0x1e7 => { bb = 27; continue; } 0x1f1 => { bb = 28; continue; } 0x23d => { bb = 29; continue; } 0x241 => { bb = 30; continue; } 0x246 => { bb = 31; continue; } 0x252 => { bb = 32; continue; } 0x264 => { bb = 33; continue; } 0x26d => { bb = 34; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402020d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x19; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(r8.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdx.wrapping_add(P2), ((rax as u8)) as u8);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(P5), (1) as u32)) as u32 as u64; }
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140202c60; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rcx = ((rcx).wrapping_add(0x18u64)) as u64; rbp = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = 0xffffffff; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = 0x20; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402021b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d1d4; const P1: u64 = 0x143c8d1f8; const P2: u64 = 0x143c8d1f0; const P3: u64 = 0x1425d5b00; const P4: u64 = 0x141834d18; const P5: u64 = 0x141834a0c; const P6: u64 = 0x141834a78; const P7: u64 = 0x141834d64; const P8: u64 = 0x140200180; const P9: u64 = 0x142670ef0; const P10: u64 = 0x1402006c0; const P11: u64 = 0x142670fa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = rdx; rbx = rcx; rbp = 0; fl_l = 0; fl_r = 0; rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r14, rbp); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rbp); rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rcx); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            5 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 6; continue; }
            6 => { rcx = 0x20; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdx.wrapping_add(0x8u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x140202297, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = P9; crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbx); rcx = 0x30; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); xmm0 = 0u128; crate::simd_ops::wu128(P2, (xmm0) as u128); rcx = 0x18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x1u32) as u32); rcx = P11; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(P2, rdi); crate::simd_ops::wu64(P1, rax); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 14; continue; }
            14 => { rdi = 0xffffffff; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xc; const P2: u64 = 0x8; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = 0x18; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d1dc; const P1: u64 = 0x143c8d1c0; const P2: u64 = 0x141834d64; const P3: u64 = 0x143080728; const P4: u64 = 0x143c8d1c8; const P5: u64 = 0x1425d5b50; const P6: u64 = 0x141834d18; const P7: u64 = 0x141834a0c; const P8: u64 = 0x141834a78; const P9: u64 = 0x1430807a0; const P10: u64 = 0x1430808a0; const P11: u64 = 0x1430809b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = P1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rsi); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rbx); xmm0 = 0u128; crate::simd_ops::wu128(P1, (xmm0) as u128); rcx = 0x18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x1u32) as u32); rdx = P3; crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r14); crate::simd_ops::wu64(P1, r14); crate::simd_ops::wu64(P4, rax); rcx = P5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            3 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rcx = 0x18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rax = P9; crate::simd_ops::wu64(r14, rax); rcx = 0x18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rax); rax = P10; crate::simd_ops::wu64(rsi, rax); rcx = 0x18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x1u32) as u32); rax = P11; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rsi); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rsi); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); rdi = 0xffffffff; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402026a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140203690; const P1: u64 = 0xe0; const P2: u64 = 0xc0; const P3: u64 = 0xa0; const P4: u64 = 0x80; const P5: u64 = 0x60; const P6: u64 = 0x38; const P7: u64 = 0x8; const P8: u64 = 0x140206d80; const P9: u64 = 0x141834d30;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x100u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64;
    rdx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    r8 = (crate::simd_ops::ru64(r8.wrapping_add(P7))) as u64;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64;
    rdx = 0x60;
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140203690; const P1: u64 = 0xe8; const P2: u64 = 0xc8; const P3: u64 = 0xa8; const P4: u64 = 0x88; const P5: u64 = 0x68; const P6: u64 = 0x38; const P7: u64 = 0x8; const P8: u64 = 0x140206d80; const P9: u64 = 0x141834d30;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x108u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64;
    rdx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    r8 = (crate::simd_ops::ru64(r8.wrapping_add(P7))) as u64;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64;
    rdx = 0x60;
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402027c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1402020c0; const P1: u64 = 0x140200d90;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rax).wrapping_add(0x10u64)) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402027f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    if (((fl_l as u64)) == 0) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi);
    rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rdi; fl_r = rdi;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rbx = 0xffffffff;
    rax = ((rbx as u32)) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    let xa_src: u32 = (rbx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rbx = (xa_old) as u32 as u64;
    fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x38; const P2: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    rbx = rcx;
    fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    if (((fl_l as u64)) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rbx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402028b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402028d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140203870; const P1: u64 = 0x141834d30; const P2: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbp = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_sub(1)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_sub(1)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = (((rdx) & (rax))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rcx); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbp); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_r = rbp; rsi = rdi.wrapping_add(0x8); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = 0x10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 7; continue; } bb = 10; continue; }
            10 => { rsi = rdi.wrapping_add(0x8); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rcx = r8; bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0x10; crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsi, rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            16 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1402029c4, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402029e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140203690; const P2: u64 = 0x10; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x141834d30; const P5: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x38u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = (0x4924924924924925u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = (((rdx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; rax = ((r8.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { r8 = rcx; bb = 6; continue; }
            6 => { rcx = r8; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); crate::simd_ops::wu64(rsi.wrapping_add(P2), rax); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140202a8d, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x141834d30; const P3: u64 = 0x8; const P4: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140202aeb, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202af0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x140203690; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rsi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    rsi = rcx;
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rax, (0x0u64) as u64);
    rbx = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0x48; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi);
    loop { rdi = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = 0x48; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rdi; fl_l = rdi; fl_r = rdi; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsi)) as u64;
    rdx = 0x48;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rsi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140204bd0; const P2: u64 = 0x10; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x141834d30; const P5: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff0u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140202bc7, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140204c50; const P2: u64 = 0x10; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x141834d30; const P5: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (0x6666666666666667u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140202c56, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = 0xffffffff; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { rdx = 0x30; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x140203870; const P2: u64 = 0x28; const P3: u64 = 0x8; const P4: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = 0xffffffff; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140201310; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0xc; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P1), (1) as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = rsi; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = 0xffffffff; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rdx = 0x10; rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645868; const P1: u64 = 0x1401f2f00; const P2: u64 = 0x1402066f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = 0x1; crate::simd_ops::wu64(rcx, rdx); let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rdx, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); rbx = rdx; rdi = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rbp = (0x7ffffffffff0bdbfu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsi = (0x7fffffffffffffffu64) as u64; bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx, ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x64u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rbp; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax).wrapping_add(0xf4240u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); bb = 11; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); bb = 11; continue; }
            11 => { rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 12; continue; }
            12 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rax = rdi; rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140202f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = rcx; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rcx); crate::simd_ops::wu64(rdx.wrapping_add(P0), rcx); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), r8); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = 0xffffffff; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645a78;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x140200230; const P3: u64 = 0x141834ee8; const P4: u64 = 0x141834ee0; const P5: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rdi; fl_r = rdi;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rbx = ((rcx.wrapping_add(P1)) as u64) as u64; r9 = P2; r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0x10; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rdx = 0x18;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140205280; const P1: u64 = 0x141834d30;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = 0x58;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402030e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0xfu64) as u64;
    rax = rcx;
    if !((((fl_l as u64)) <= ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; }
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x1402036f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    fl_l = rcx; fl_r = rdx;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = (0xfu64) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    if !((((fl_l as u64)) <= ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rdx)) as u64; }
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x141834d30; const P3: u64 = 0x10; const P4: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = r8; bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), (0xfu64) as u64); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu8(rbx, ((rax as u8)) as u8); let y_addr: u64 = rdi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rdi.wrapping_add(P3), rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), (0xfu64) as u64); crate::simd_ops::wu8(rdi, ((rax as u8)) as u8); bb = 7; continue; }
            7 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1402031c8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402031d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x1402050d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    r8 = (0xffffffffffffffffu64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    loop { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x1; const P3: u64 = 0x1401e9110; const P4: u64 = 0x14183acbc; const P5: u64 = 0x140200110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; r14 = rdx; xmm0 = 0u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rbx = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_r = (0xfu64) as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru64(rdx)) as u64; bb = 2; continue; }
            2 => { rdi = (0x7fffffffffffffffu64) as u64; fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = rsi; fl_r = (0xfu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rcx.wrapping_add(P0), rsi); crate::simd_ops::wu64(rcx.wrapping_add(P1), (0xfu64) as u64); xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); bb = 8; continue; }
            5 => { rax = rsi; rax = (((rax) | (0xfu64))) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = 0x16; rdi = rax; fl_l = rax; fl_r = rcx; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rdi }) as u64; bb = 7; continue; }
            7 => { rcx = ((rdi.wrapping_add(P2)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rsi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rbx, rax); rdx = r14; crate::simd_ops::wu64(rbx.wrapping_add(P0), rsi); rcx = rax; crate::simd_ops::wu64(rbx.wrapping_add(P1), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1402032d3, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30; const P1: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; r14 = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rdx = 0x18; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rdx = 0x10; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rdx = 0x18; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 20; continue; }
            20 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 24; continue; } bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = r8; bb = 24; continue; }
            24 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rax); bb = 25; continue; }
            25 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            26 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140203450, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (((crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as i8 as i64) as u64) as u64; rdx = ((P0) as u64) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x2034c8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = (rax) as u64; bb = 0xffff_fffe; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbx = (0xffffffffu32) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); let xa_src: u32 = (rbx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x140203460u64) { 0x22 => { bb = 1; continue; } 0x2b => { bb = 2; continue; } 0x3c => { bb = 3; continue; } 0x4e => { bb = 4; continue; } 0x57 => { bb = 5; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402034f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x141834d30; const P2: u64 = 0x1426467f0; const P3: u64 = 0x1402035d7;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1402034f0, funcinfo_rva: 0x35a3794, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rdi) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (rdi) as u64); rcx = ((rdi.wrapping_add(0x58u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203518); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (0x7u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (0x8u64) as u64); crate::simd_ops::wu32(rbx, (0x3f800000u32) as u32); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rdi as u32)) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rcx = (0x80u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203561); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_mul(8)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = (r8) as u64; bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402035a6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rdi) as u64); rax = ((rdi.wrapping_add(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (rax) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdi, (rsi) as u64); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { bb = 12; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402035d1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            10 => { rcx = ((rcx).wrapping_add(0x7u64)) as u64; rcx = ((rcx).wrapping_shr(3u32)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (rsi) as u64; while rcx != 0 { core::ptr::write_unaligned(rdi as *mut u64, rax); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); } bb = 12; continue; }
            12 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x140203690; const P2: u64 = 0x140203cf0; const P3: u64 = 0x140202a90; const P4: u64 = 0x140203c60; const P5: u64 = 0x140203c00;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (((crate::simd_ops::ru8(rcx.wrapping_add(0x20u64))) as i8 as i64) as u64) as u64; rdx = P0; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x203648u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = rax; bb = 0xffff_fffe; continue; }
            1 => { return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { return rax; }
            3 => { return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            5 => { return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            7 => { return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            8 => { return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            9 => { return crate::simd_ops::call_fwd(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x140203600u64) { 0x18 => { bb = 1; continue; } 0x1d => { bb = 2; continue; } 0x1e => { bb = 3; continue; } 0x23 => { bb = 4; continue; } 0x28 => { bb = 5; continue; } 0x2d => { bb = 6; continue; } 0x32 => { bb = 7; continue; } 0x37 => { bb = 8; continue; } 0x3c => { bb = 9; continue; } _ => { return crate::simd_ops::call_fwd(j_tgt, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x141834d30; const P3: u64 = 0x10; const P4: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; fl_l = rdx; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1402036ee, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402036f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc8; const P1: u64 = 0x1401e9110; const P2: u64 = 0x14183acbc; const P3: u64 = 0x141834d30; const P4: u64 = 0x1426467f0; const P5: u64 = 0x140200110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rsi = r8; r15 = rdx; rbx = rcx; fl_l = r8; fl_r = rbp; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = rcx; fl_l = rbp; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rcx)) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rsi); rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi.wrapping_add(rsi), (0x0u8) as u8); bb = 14; continue; }
            4 => { rdi = (0x7fffffffffffffffu64) as u64; fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 5; continue; }
            5 => { rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rcx = (((rcx) | (0xfu64))) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = rbp; rax = rdi; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((rdx.wrapping_add(rbp)) as u64) as u64; rdi = rcx; fl_l = rcx; fl_r = rax; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; bb = 8; continue; }
            8 => { rcx = rdi.wrapping_add(0x1); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = rsi; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rsi); rdx = r15; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); rcx = rax; r14 = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(r14.wrapping_add(rsi), (0x0u8) as u8); fl_l = rbp; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp.wrapping_add(0x1); fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rcx = r8; bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx, r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 14; continue; }
            14 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1402037f6, rcx, rsp); return rax; }
            16 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1402037fc, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (0xfu64) as u64);
    crate::simd_ops::wu8(rcx, (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0xfu64) as u64;
    if !((((fl_l as u64)) <= ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; return rax; }
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x141834d30; const P2: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = rdx; fl_l = r8; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r8 = ((r8).wrapping_add(0x27u64)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = rcx; bb = 3; continue; }
            3 => { rdx = r8; rcx = rax; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14020386d, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x141834d30; const P2: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = rdx; rdx = (((r8) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = rcx; bb = 3; continue; }
            3 => { rcx = rax; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14020395e, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401e9110; const P2: u64 = 0x10; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x141834d30; const P5: u64 = 0x80; const P6: u64 = 0x1426467f0; const P7: u64 = 0x1401e6780;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = rdx; rax = ((rax).wrapping_sub(rcx)) as u64; rbx = r8; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = rax; fl_r = (0x10u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { rcx = 0x80; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rdi = rax; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_mul(8)) as u64) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rcx = r8; bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = ((rdi.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu64(rsi, rdi); crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); crate::simd_ops::wu64(rsi.wrapping_add(P2), rax); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi, rbx); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140203a22, rcx, rsp); return rax; }
            11 => { r8 = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x141834d30; const P2: u64 = 0x1426467f0; const P3: u64 = 0x1401e6780; const P4: u64 = 0x1401e6f50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rbx = r8; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = rcx; r9 = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rax).wrapping_sub(r9)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { rax = (0x1fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); r14 = ((rdx.wrapping_mul(8)) as u64) as u64; rcx = r14; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rdi = rax; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rcx = r8; bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = ((r14.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu64(rsi, rdi); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rax); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi, rbx); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140203b1b, rcx, rsp); return rax; }
            11 => { rdx = rcx; r8 = rsp.wrapping_add(0x40); rcx = r9; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140203b3c, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x141834d30; const P3: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = rdx; rdx = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = rcx; bb = 3; continue; }
            3 => { rcx = rax; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140203bfe, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x141834d30; const P3: u64 = 0x8; const P4: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140203c57, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140203690; const P2: u64 = 0x10; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x141834d30; const P5: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xffffffffffffffe0u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = r8; bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); crate::simd_ops::wu64(rsi.wrapping_add(P2), rax); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140203ceb, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x141834d30; const P3: u64 = 0x8; const P4: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140203d4b, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x14183acc8; const P2: u64 = 0x141834d64; const P3: u64 = 0x1426467f0; const P4: u64 = 0x140203e75; const P5: u64 = 0x141834d30; const P6: u64 = 0x1401e6f50; const P7: u64 = 0x1402032e0; const P8: u64 = 0x140203f1a;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140203da0, funcinfo_rva: 0x35a38ec, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = (rdx) as u64; r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(r8)) as u64; r15 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); rsi = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r12) as u64); rdi = ((r12 as u32)) as u32 as u64; rbx = ((r12 as u32)) as u32 as u64; r13 = (0x7fffffffffffffffu64) as u64; fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (rbp) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 20; continue; } bb = 2; continue; }
            2 => { rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203dfc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rsi = ((rax.wrapping_add(rbp)) as u64) as u64; rdi = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); r8 = (rbp) as u64; rdx = (r15) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203e1e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r14, (r12) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (r12) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (r12) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rbx) as u64 as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = (rdi) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 21; continue; } bb = 5; continue; }
            5 => { fl_l = (rdi) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = ((rdi.wrapping_add(0x27u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203e57); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r15 = ((rax.wrapping_add(0x27u64)) as u64) as u64; fl_l = (((r15) & (0xffffffffffffffe0u64))) as u64 as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r15.wrapping_sub(0x8u64), (rax) as u64); bb = 11; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203e6f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P4, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203e79); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (rax) as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(r14, (r15) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (r15) as u64); rcx = ((r15.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rcx) as u64); r8 = (rdi) as u64; rdx = (rbx) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203e99); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((r15.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); bb = 12; continue; }
            12 => { crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0xeu8) as u8); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { rsi = ((rsi).wrapping_sub(rbx)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rsi = ((rsi).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rbx = ((rbx).wrapping_sub(rcx)) as u64; rax = ((rbx.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { rbx = (rcx) as u64; bb = 16; continue; }
            16 => { rdx = (rsi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203eda); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { rax = (r14) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203f02); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            19 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203f09); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 20; continue; }
            20 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203f0f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            21 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203f15); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140203f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x14183acc8; const P2: u64 = 0x141834d30; const P3: u64 = 0x1426467f0; const P4: u64 = 0x1402032e0; const P5: u64 = 0x140204092;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140203f20, funcinfo_rva: 0x35a3930, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = (rdx) as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(r8)) as u64; rdi = ((rdi).wrapping_shl(2u32)) as u64; r12 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); rsi = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r13) as u64); rbx = (rdi) as u64; rbx = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; r15 = ((r13 as u32)) as u32 as u64; rbp = ((r13 as u32)) as u32 as u64; rax = (0x3fffffffffffffffu64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx.wrapping_mul(4)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203f8f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rsi = ((rbx.wrapping_add(rax)) as u64) as u64; r15 = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); r8 = (rdi) as u64; rdx = (r12) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203fb1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); rax = (0x3fffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r14, (r13) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (r13) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (r13) as u64); r15 = ((r15).wrapping_sub(rbp)) as u64; rdi = (r15) as u64; rdi = ((((rdi) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdi = ((rdi.wrapping_mul(4)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140203ff3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); rcx = ((rdi.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rcx) as u64); r8 = (r15) as u64; rdx = (rbp) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204013); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); bb = 6; continue; }
            6 => { crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0x7u8) as u8); fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rsi = ((rsi).wrapping_sub(rbp)) as u64; rsi = (((rsi) & (0xfffffffffffffffcu64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rsi = ((rsi).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).wrapping_sub(rcx)) as u64; rax = ((rbp.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rbp = (rcx) as u64; bb = 10; continue; }
            10 => { rdx = (rsi) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204058); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (r14) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204080); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204087); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020408d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1402040a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x14183acc8; const P2: u64 = 0x141834d30; const P3: u64 = 0x1426467f0; const P4: u64 = 0x1402032e0; const P5: u64 = 0x140204212;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1402040a0, funcinfo_rva: 0x35a3930, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = (rdx) as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(r8)) as u64; rdi = ((rdi).wrapping_shl(2u32)) as u64; r12 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); rsi = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r13) as u64); rbx = (rdi) as u64; rbx = ((((rbx) as i64).wrapping_shr(2u32) as u64)) as u64; r15 = ((r13 as u32)) as u32 as u64; rbp = ((r13 as u32)) as u32 as u64; rax = (0x3fffffffffffffffu64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx.wrapping_mul(4)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020410f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rsi = ((rbx.wrapping_add(rax)) as u64) as u64; r15 = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); r8 = (rdi) as u64; rdx = (r12) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204131); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); rax = (0x3fffffffffffffffu64) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r14, (r13) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (r13) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (r13) as u64); r15 = ((r15).wrapping_sub(rbp)) as u64; rdi = (r15) as u64; rdi = ((((rdi) as i64).wrapping_shr(2u32) as u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdi = ((rdi.wrapping_mul(4)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204173); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); rcx = ((rdi.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rcx) as u64); r8 = (r15) as u64; rdx = (rbp) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204193); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rax) as u64); bb = 6; continue; }
            6 => { crate::simd_ops::wu8(r14.wrapping_add(0x20u64), (0x8u8) as u8); fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rsi = ((rsi).wrapping_sub(rbp)) as u64; rsi = (((rsi) & (0xfffffffffffffffcu64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rsi = ((rsi).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; rbp = ((rbp).wrapping_sub(rcx)) as u64; rax = ((rbp.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rbp = (rcx) as u64; bb = 10; continue; }
            10 => { rdx = (rsi) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1402041d8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (r14) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204200); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140204207); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14020420d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}
