#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_141b6f6f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x141b71150; const P2: u64 = 0x142409b50; const P3: u64 = 0x14052f030; const P4: u64 = 0x1420ecd00; const P5: u64 = 0x14062e6d0; const P6: u64 = 0x1406562b0; const P7: u64 = 0x1403b8050; const P8: u64 = 0x14240ab10; const P9: u64 = 0x1421158a0; const P10: u64 = 0x141b6f91e; const P11: u64 = 0x141b6f930;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b6f6f0, funcinfo_rva: 0x3737914, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xd8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x47u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbx = (rcx) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdi = ((P0) as u64) as u64; rcx = (rdi) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r14 = ((rsi as u32)) as u32 as u64; bb = 3; continue; }
            2 => { r14 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 4; continue; }
            4 => { fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 6; continue; }
            6 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6f760); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (rax) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 7; continue; }
            7 => { fl_l = (rdx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rax).wrapping_add(0x18u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), (rax) as u64); r9 = (0xffffffffffffffffu64) as u64; bb = 10; continue; }
            10 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), (r9) as u64); rax = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x39u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64; rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x36u64), (0x8u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x31u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x59u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_sub(0x59u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6f7e3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_add(0xfu64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_add(0x2fu64), (xmm0) as u128); rax = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), (0x4u32) as u32); rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6f810); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (r8) as u64; rcx = ((rcx).wrapping_sub(r9)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = ((r9.wrapping_add(0x30u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = ((rbp.wrapping_sub(0x59u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6f85c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(core::ptr::read_unaligned((rdi) as *const u64))))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((rax as u32)) as u32); r9 = ((rbp.wrapping_add(0x67u64)) as u64) as u64; r8 = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; rdx = (r15) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6f882); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            15 => { crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((rsi as u32)) as u32); r9 = ((rbp.wrapping_add(0x67u64)) as u64) as u64; r8 = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; rdx = (r15) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6f89a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { xmm0 = 0u128; ymm0_hi = 0; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as i32) as f64).to_bits() as u128); ymm0_hi = 0; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x3u32) as u32); bb = 18; continue; }
            17 => { crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rsi as u32)) as u32); bb = 18; continue; }
            18 => { rsi = (0x1u32) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64)) as u64; fl_r = fl_l; bb = 19; continue; }
            19 => { rcx = ((rbp.wrapping_add(0xfu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6f8ce); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6f8d8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rsi as u32)) as u32 as u64; r11 = ((rsp.wrapping_add(0xc0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { rdx = (0x3u32) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6f907); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            21 => { rdx = (0x2u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6f919); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { rdx = (0x1u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6f92b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b6f940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x141b71150; const P2: u64 = 0x142409b50; const P3: u64 = 0x14052f030; const P4: u64 = 0x14062e8e0; const P5: u64 = 0x14063a470; const P6: u64 = 0x1403b8050; const P7: u64 = 0x14240ab10; const P8: u64 = 0x1421158a0; const P9: u64 = 0x141b6fb77; const P10: u64 = 0x141b6fb89;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b6f940, funcinfo_rva: 0x3737978, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x47u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rbx = (rcx) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r15 = ((P0) as u64) as u64; rcx = (r15) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsi = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 28; continue; } bb = 4; continue; }
            4 => { fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 6; continue; }
            6 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6f9af); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 7; continue; }
            7 => { fl_l = (rdx) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rax).wrapping_add(0x18u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), (rax) as u64); r9 = (0xffffffffffffffffu64) as u64; bb = 10; continue; }
            10 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (r9) as u64); rax = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x39u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64; rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x36u64), (0x8u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x31u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x49u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fa33); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_add(0xfu64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu128(rbp.wrapping_add(0x2fu64), (xmm0) as u128); rax = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), (0x4u32) as u32); rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fa60); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (r8) as u64; rcx = ((rcx).wrapping_sub(r9)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 12; continue; }
            12 => { rax = ((r9.wrapping_add(0x30u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 13; continue; }
            13 => { fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 20; continue; }
            19 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { crate::simd_ops::wu8(rbp.wrapping_add(0x67u64), ((rax as u8)) as u8); r9 = ((rbp.wrapping_add(0x67u64)) as u64) as u64; r8 = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; rdx = (r14) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fadc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 25; continue; }
            21 => { crate::simd_ops::wu8(rbp.wrapping_add(0x67u64), (0x0u8) as u8); r9 = ((rbp.wrapping_add(0x67u64)) as u64) as u64; r8 = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; rdx = (r14) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6faf5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x67u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rcx, ((rdi as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x1u32) as u32); bb = 24; continue; }
            23 => { crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdi as u32)) as u32); bb = 24; continue; }
            24 => { rdi = (0x1u32) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64)) as u64; fl_r = fl_l; bb = 25; continue; }
            25 => { rcx = ((rbp.wrapping_add(0xfu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fb26); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fb30); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rdi as u32)) as u32 as u64; r11 = ((rsp.wrapping_add(0xb0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { rdx = (0x3u32) as u32 as u64; r8 = ((rdx.wrapping_sub(0x2u64)) as u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fb60); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 27; continue; }
            27 => { rdx = (0x2u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fb72); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            28 => { rdx = (0x1u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fb84); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P10, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b6fb90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x141b71150; const P2: u64 = 0x142409b50; const P3: u64 = 0x14052f030; const P4: u64 = 0x1405e37f0; const P5: u64 = 0x1403b8050; const P6: u64 = 0x14240ab10; const P7: u64 = 0x1421158a0; const P8: u64 = 0x141b6fd3e; const P9: u64 = 0x141b6fd50;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b6fb90, funcinfo_rva: 0x37379d8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rbx = (rcx) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = ((P0) as u64) as u64; rcx = (rbp) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = (rax) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fbf4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = (rdx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rax).wrapping_add(0x18u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (0xffffffffffffffffu64) as u64; bb = 10; continue; }
            10 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64; rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x33u64), (0x8u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fc8b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x78u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x98u64), (xmm0) as u128); rax = ((rsp.wrapping_add(0x88u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), (0x0u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x84u64), (0x4u32) as u32); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fccf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (rsi) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fce0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (((rax as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64); rcx = ((rsp.wrapping_add(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fcff); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fd0a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0x1u32) as u32 as u64; r11 = ((rsp.wrapping_add(0xb0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rdx = (0x1u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fd39); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rdx = (0x2u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b6fd4b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b6fd60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140657030; const P1: u64 = 0x1429868d0; const P2: u64 = 0x14310c1c0; const P3: u64 = 0x1406569f0; const P4: u64 = 0x1420ecd00; const P5: u64 = 0x143426b90; const P6: u64 = 0x143426514; const P7: u64 = 0x140675a90; const P8: u64 = 0x140275010; const P9: u64 = 0x1421158a0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x100u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rbx = rcx; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm10) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r14 = P1; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = rax; rax = r8.wrapping_add(0x18); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 3; continue; }
            3 => { r8 = P2; rdx = 0x2; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rsi = rax; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = rsp.wrapping_add(0x30); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; ymm0_hi = 0; bb = 11; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru64(P5)) as u64) as u128; ymm0_hi = 0; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x48u64)) as u64; xmm1 = (xmm0 & !0xffff_ffffu128) | (((f64::from_bits(xmm0 as u64)) as f32).to_bits() as u128); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 12; continue; }
            12 => { fl_l = rax; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 15; continue; }
            15 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 20; continue; }
            19 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = 0x0; let y_addr: u64 = rsi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 48; continue; }
            22 => { crate::simd_ops::wu32(rdi.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), (0x0u8) as u8); crate::simd_ops::wu8(rdi.wrapping_add(0x30u64), ((rax as u8)) as u8); bb = 47; continue; }
            23 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = r14; bb = 25; continue; }
            25 => { rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x4; rcx = ((rsi as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) == ((fl_r as u32))) { rax } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 26; continue; }
            26 => { xmm8 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; xmm9 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32) as u128; rax = r8.wrapping_add(0x30); rcx = r14; fl_l = rax; fl_r = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 28; continue; }
            28 => { xmm7 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x48u64)) as u64; xmm7 = crate::simd_ops::vcvtpd2ps(xmm7); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 35; continue; } bb = 29; continue; }
            29 => { fl_l = rax; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 35; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 32; continue; }
            32 => { rdx = rsp.wrapping_add(0x30); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 34; continue; }
            34 => { xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; ymm0_hi = 0; bb = 36; continue; }
            35 => { xmm0 = ((crate::simd_ops::ru64(P5)) as u64) as u128; ymm0_hi = 0; bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x60u64)) as u64; xmm6 = (xmm0 & !0xffff_ffffu128) | (((f64::from_bits(xmm0 as u64)) as f32).to_bits() as u128); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 44; continue; } bb = 37; continue; }
            37 => { fl_l = rax; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 44; continue; } bb = 39; continue; }
            39 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 49; continue; } bb = 40; continue; }
            40 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 44; continue; } bb = 41; continue; }
            41 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 43; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 44; continue; } bb = 43; continue; }
            43 => { r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 45; continue; }
            44 => { r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 45; continue; }
            45 => { xmm2 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm3 = 0u128; xmm1 = 0u128; rcx = rsp.wrapping_add(0x80); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((xmm9 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), ((xmm10 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x50); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0xcu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x5cu64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64) as u128; ymm0_hi = 0; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rcx as u32)) as u32); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x34u64))) as u64) as u128; ymm0_hi = 0; xmm0 = crate::simd_ops::vinsertps(xmm0, ((crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u128), 0x20u8); ymm0_hi = 0; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x74u64))) as u64) as u128; xmm0 = crate::simd_ops::vinsertps(xmm0, ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u128), 0x30u8); ymm0_hi = 0; xmm1 = crate::simd_ops::vinsertps(xmm1, ((crate::simd_ops::ru32(rsp.wrapping_add(0x7cu64))) as u128), 0x20u8); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 48; continue; } bb = 46; continue; }
            46 => { let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu32(rdi.wrapping_add(0x34u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x4u64), ((rsi as u8)) as u8); crate::simd_ops::wu8(rdi.wrapping_add(0x30u64), ((r14 as u8)) as u8); bb = 47; continue; }
            47 => { rsi = 0x1; crate::simd_ops::wu32(rdi, ((rsi as u32)) as u32); let y_addr: u64 = rdi.wrapping_add(0x10u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax, ((rsi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x1u32) as u32); rax = 0x1; fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (0x18u64) as u64)) as u64; fl_r = fl_l; r11 = rsp.wrapping_add(0x100); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            49 => { rdx = 0x5; rcx = rbx; r8 = ((rdx.wrapping_sub(0x4u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b7008f, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            50 => { rdx = 0x3; rcx = rbx; r8 = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b700a0, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            51 => { rdx = 0x4; rcx = rbx; r8 = ((rdx.wrapping_sub(0x3u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b700b2, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            52 => { rdx = 0x2; rcx = rbx; r8 = ((rdx.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b700c4, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            53 => { rdx = 0x3; rcx = rbx; r8 = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b700d5, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            54 => { rdx = 0x4; rcx = rbx; r8 = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b700e7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b700f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x141b71150; const P4: u64 = 0x141ba21f0; const P5: u64 = 0x1; const P6: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = P1; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rbx = r8; fl_l = rax; fl_r = rdx; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rdx = 0x1; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141b70157, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b70160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x141b71150; const P2: u64 = 0x143a333a0; const P3: u64 = 0x1403f4140; const P4: u64 = 0x143a33330; const P5: u64 = 0x1427a7bc0; const P6: u64 = 0x1402b8b60; const P7: u64 = 0x1427a7bb0; const P8: u64 = 0x1402b8d70; const P9: u64 = 0x1427a7bb8; const P10: u64 = 0x140241da0; const P11: u64 = 0x1427a7bc8; const P12: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r13 = P0; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = rcx; fl_l = rax; fl_r = rdx; rcx = r13; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r14 = ((rbp as u32)) as u32 as u64; bb = 3; continue; }
            2 => { r14 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 30; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r12 = rax; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdi = r13; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdi = rbp; bb = 9; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 31; continue; } bb = 10; continue; }
            10 => { fl_l = rax; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); r8 = P2; rdx = 0x3; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r15 = ((rax as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = rdx.wrapping_add(0x48); fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 13; continue; }
            13 => { fl_l = rax; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 20; continue; } bb = 15; continue; }
            15 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 16; continue; }
            16 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 21; continue; }
            20 => { rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            21 => { rax = rdx.wrapping_add(0x60); fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 25; continue; } bb = 22; continue; }
            22 => { fl_l = rax; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 25; continue; } bb = 24; continue; }
            24 => { r8 = P4; rdx = 0x5; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; bb = 25; continue; }
            25 => { rax = 0x3fff; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (0x3fffffffffffc000u64) as u64; fl_l = rax; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { r9 = rsp.wrapping_add(0x88); rdx = r12; r8 = P5; rcx = r14; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r9 = rsp.wrapping_add(0x78); rdx = r12; r8 = P7; rcx = r14; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r9 = rsp.wrapping_add(0x70); rdx = r12; r8 = P9; rcx = r14; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); r9 = rsp.wrapping_add(0x80); rdx = r12; r8 = P11; rcx = r14; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            29 => { rdx = 0x4; rcx = rbx; r8 = ((rdx.wrapping_sub(0x3u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141b7035c, rcx, rsp); return rax; }
            30 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141b7036e, rcx, rsp); return rax; }
            31 => { rdx = 0x2; rcx = rbx; r8 = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141b7037f, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b703b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x1401fa730; const P2: u64 = 0x1404056a0; const P3: u64 = 0x1420ecd00; const P4: u64 = 0x1426cf238; const P5: u64 = 0x141ef4820; const P6: u64 = 0x1406574b0; const P7: u64 = 0x1406573b0; const P8: u64 = 0x142116bc0; const P9: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbx = P0; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdi = rcx; fl_l = rax; fl_r = rdx; rcx = rbx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r15 = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rcx); rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; bb = 9; continue; }
            8 => { r8 = rsp.wrapping_add(0x70); rcx = rsi; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x58u64)))) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x58u64)))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(r15)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rbx }) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(crate::simd_ops::ru64(rbx))))) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x58u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { r8 = P4; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b704ee, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x78); rcx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b70522, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b70534, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { rdx = 0x2; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b70546, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b70550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1406571f0; const P1: u64 = 0x1429868d0; const P2: u64 = 0x140696a30; const P3: u64 = 0x1421158a0; const P4: u64 = 0x141b70675;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b70550, funcinfo_rva: 0x3737b44, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b70562); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { rbp = ((P1) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rdx).wrapping_add(0x18u64)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rcx = (0xffffffffffffffffu64) as u64; bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rcx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 10; continue; }
            7 => { rax = (0xcbf29ce484222325u64) as u64; r8 = ((rcx.wrapping_add(rdx)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { r9 = (0x100000001b3u64) as u64; bb = 9; continue; }
            9 => { rcx = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rcx = (((rcx) | (0x20u64))) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rdi.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b70632); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rax; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rdx = (0x2u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b70670); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P4, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b70680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1406571f0; const P1: u64 = 0x1429868d0; const P2: u64 = 0x1420ecc90; const P3: u64 = 0x14210c770; const P4: u64 = 0x1401fc320; const P5: u64 = 0x1421158a0; const P6: u64 = 0x141b7082a; const P7: u64 = 0x141b7083c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b70680, funcinfo_rva: 0x3737b7c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7069d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsi = (rax) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 28; continue; } bb = 1; continue; }
            1 => { r14 = ((P1) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 3; continue; }
            3 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x1u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), (rdi) as u64); bb = 6; continue; }
            6 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b706f9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (rbx) as u64; bb = 12; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b70718); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; rdx = (r14) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rcx).wrapping_add(0x18u64)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x30u64)) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = (r14) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rbp = (rbx) as u64; bb = 15; continue; }
            14 => { rbp = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (rbp) as u64 as u64; fl_r = (rbp) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 16; continue; }
            16 => { fl_l = (rax) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 18; continue; }
            18 => { rax = (0xffffffffffffffffu64) as u64; bb = 19; continue; }
            19 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 20; continue; }
            20 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rbx = (0xcbf29ce484222325u64) as u64; rdx = ((rax.wrapping_add(rcx)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 22; continue; }
            22 => { r8 = (0x100000001b3u64) as u64; bb = 23; continue; }
            23 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rax = (((rax) | (0x20u64))) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b707e2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            27 => { rdx = (0x3u32) as u32 as u64; r8 = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b70825); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            28 => { rdx = (0x2u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b70837); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b70850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1429868d0; const P2: u64 = 0x8; const P3: u64 = 0x141b71150; const P4: u64 = 0x1427a71f8; const P5: u64 = 0x140247170; const P6: u64 = 0x1427a71e0; const P7: u64 = 0x1427a71e8; const P8: u64 = 0x1427a71c0; const P9: u64 = 0x1427a71d0; const P10: u64 = 0x1427a71d8; const P11: u64 = 0x1; const P12: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = P1; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = r9; fl_l = rax; fl_r = r8; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = P4; rdx = rax; rcx = rdi; rbx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = P6; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = P7; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = P8; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = P9; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = P10; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rdx = 0x1; r8 = ((rdx.wrapping_add(P11)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141b70931, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b70940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x141b71150; const P2: u64 = 0x14310c1c0; const P3: u64 = 0x1406569f0; const P4: u64 = 0x1427a71e8; const P5: u64 = 0x1402b8b60; const P6: u64 = 0x1427a71d8; const P7: u64 = 0x140241fb0; const P8: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r14 = P0; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = rcx; fl_l = rax; fl_r = rdx; rcx = r14; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsi = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 6; continue; }
            6 => { rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = r14; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rcx; fl_r = r8; rbp = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rdx }) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rcx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = P2; rdx = 0x2; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); r8 = P4; rdx = rbp; rcx = rsi; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; r9 = rsp.wrapping_add(0x20); rdx = rbp; r8 = P6; rcx = rsi; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rdx = 0x2; rcx = rbx; r8 = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b70a57, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b70a69, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b70a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x141b71150; const P2: u64 = 0x14310c1c0; const P3: u64 = 0x1406569f0; const P4: u64 = 0x140656b10; const P5: u64 = 0x143a333a0; const P6: u64 = 0x1403f4140; const P7: u64 = 0x1427a71c0; const P8: u64 = 0x1402b8d70; const P9: u64 = 0x143428460; const P10: u64 = 0x143428120; const P11: u64 = 0x143428050; const P12: u64 = 0x1427a71c8; const P13: u64 = 0x140241fb0; const P14: u64 = 0x1427a71f0; const P15: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r8 = P0; rbp = rcx; rdx = r8; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { r14 = (crate::simd_ops::ru64(rdx)) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbx); rcx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P2; rdx = 0x2; rcx = rbp; rsi = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x3; rcx = rbp; rdi = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P5; rdx = 0x4; rcx = rbp; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x40u64))) as u128; xmm8 = crate::simd_ops::vsubps(xmm0, crate::simd_ops::ru128(rbx)); xmm6 = crate::simd_ops::vdpps(xmm8, xmm8, 0x7fu8); r9 = rsp.wrapping_add(0x80); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rax as u32)) as u32); r8 = P7; rdx = rsi; rcx = r14; xmm7 = crate::simd_ops::vrsqrtps(xmm6); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(P9)) as u128; xmm0 = crate::simd_ops::vmulps(xmm7, xmm6); xmm2 = crate::simd_ops::vmulps(xmm0, xmm7); xmm0 = crate::simd_ops::vmulps(xmm7, crate::simd_ops::ru128(P10)); xmm3 = crate::simd_ops::vsubps(xmm1, xmm2); xmm1 = crate::simd_ops::vmulps(xmm3, xmm0); xmm3 = crate::simd_ops::vmulps(xmm1, xmm8); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P11)); xmm1 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(rbx)); r9 = rsp.wrapping_add(0x20); rdx = rsi; r8 = P12; rcx = r14; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x10u64))) as u128; r9 = rsp.wrapping_add(0x20); rdx = rsi; r8 = P14; rcx = r14; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rax = 0; fl_l = 0; fl_r = 0; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rdx = 0x1; rcx = rbp; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b70c16, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b70c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1429868d0; const P1: u64 = 0x141b71150; const P2: u64 = 0x14310c1c0; const P3: u64 = 0x1406569f0; const P4: u64 = 0x143a333a0; const P5: u64 = 0x1403f4140; const P6: u64 = 0x1427a71f8; const P7: u64 = 0x1402b8b60; const P8: u64 = 0x1427a71c0; const P9: u64 = 0x1402b8d70; const P10: u64 = 0x1427a71d0; const P11: u64 = 0x140241fb0; const P12: u64 = 0x1427a71e0; const P13: u64 = 0x143427ab0; const P14: u64 = 0x1433e4530; const P15: u64 = 0x140247170; const P16: u64 = 0x1421158a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r12 = P0; rbx = rcx; rdi = r12; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rcx; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = ((rsi as u32)) as u32 as u64; bb = 3; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 30; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rbp = rax; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = r12; rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r14 = rsi; bb = 9; continue; }
            8 => { r14 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 31; continue; } bb = 10; continue; }
            10 => { fl_l = rax; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 12; continue; }
            12 => { r8 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); rdx = 0x2; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = rax; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 15; continue; }
            15 => { r8 = P4; rdx = 0x3; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rax as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x48u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 17; continue; }
            17 => { fl_l = rax; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 19; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 20; continue; }
            20 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 25; continue; }
            24 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 25; continue; }
            25 => { r9 = rsp.wrapping_add(0x68); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rsi as u32)) as u32); r8 = P6; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); rdx = rbp; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rsp.wrapping_add(0x60); rdx = rbp; r8 = P8; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(r15.wrapping_add(0x10u64))) as u128; r9 = rsp.wrapping_add(0x20); rdx = rbp; r8 = P10; rcx = rdi; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P12; rdx = rbp; rcx = rdi; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { xmm3 = (crate::simd_ops::ru128(r15)) as u128; xmm0 = crate::simd_ops::vpermilps(xmm3, 0xc4u8); xmm2 = crate::simd_ops::vpermilps(xmm3, 0xdfu8); xmm1 = crate::simd_ops::vpermilps(xmm3, 0xcau8); xmm1 = crate::simd_ops::vmulps(xmm1, xmm0); xmm4 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P13)); xmm0 = crate::simd_ops::vpermilps(xmm3, 0xd1u8); xmm1 = crate::simd_ops::vmulps(xmm2, xmm0); xmm3 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P13)); xmm1 = (crate::simd_ops::ru128(P14)) as u128; xmm2 = crate::simd_ops::vaddps(xmm3, xmm4); xmm0 = crate::simd_ops::vsubps(xmm4, xmm3); xmm3 = crate::simd_ops::vshufps(xmm2, xmm0, 0x50u8); xmm2 = crate::simd_ops::vsubps(xmm1, xmm2); xmm0 = crate::simd_ops::vshufps(xmm3, xmm2, 0xe8u8); r9 = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            27 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            29 => { rdx = 0x4; rcx = rbx; r8 = ((rdx.wrapping_sub(0x3u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b70e51, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            30 => { rdx = 0x1; rcx = rbx; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b70e63, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            31 => { rdx = 0x2; rcx = rbx; r8 = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b70e74, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b71280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b70e80; const P1: u64 = 0x143222878; const P2: u64 = 0x143222888; const P3: u64 = 0x141b70c20; const P4: u64 = 0x143222848; const P5: u64 = 0x141b70a70; const P6: u64 = 0x143222868; const P7: u64 = 0x141b70940; const P8: u64 = 0x143222820; const P9: u64 = 0x141b70850; const P10: u64 = 0x143222830; const P11: u64 = 0x141b70680; const P12: u64 = 0x1432226d8; const P13: u64 = 0x141b70550; const P14: u64 = 0x1432226f0; const P15: u64 = 0x141b703b0; const P16: u64 = 0x1432226b0; const P17: u64 = 0x141b703a0; const P18: u64 = 0x1432226c0; const P19: u64 = 0x141b70390; const P20: u64 = 0x143222680; const P21: u64 = 0x141b70160; const P22: u64 = 0x143222698; const P23: u64 = 0x141b700f0; const P24: u64 = 0x143222650; const P25: u64 = 0x141b6fd60; const P26: u64 = 0x143222660; const P27: u64 = 0x141b6f0c0; const P28: u64 = 0x143222798; const P29: u64 = 0x141b6eee0; const P30: u64 = 0x1432227a8; const P31: u64 = 0x141b6eed0; const P32: u64 = 0x143222758; const P33: u64 = 0x141b6ed80; const P34: u64 = 0x143222778; const P35: u64 = 0x141b6ed60; const P36: u64 = 0x143222730; const P37: u64 = 0x141b6ebf0; const P38: u64 = 0x143222748; const P39: u64 = 0x141b6eb70; const P40: u64 = 0x143222700; const P41: u64 = 0x141b6eb30; const P42: u64 = 0x143222718; const P43: u64 = 0x141b6ead0; const P44: u64 = 0x143222948; const P45: u64 = 0x141b6fb90; const P46: u64 = 0x143222960; const P47: u64 = 0x141b6f940; const P48: u64 = 0x143222910; const P49: u64 = 0x141b6f6f0; const P50: u64 = 0x143222928; const P51: u64 = 0x141b6f490; const P52: u64 = 0x1432228d8; const P53: u64 = 0x141b6f270; const P54: u64 = 0x1432228f8; const P55: u64 = 0x141b6ea90; const P56: u64 = 0x142119080; const P57: u64 = 0x142118cd0; const P58: u64 = 0x142103330; const P59: u64 = 0x1420ec690;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 71>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x120); rsp = ((rsp).wrapping_sub(0x220u64)) as u64; rax = P0; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbx = P1; rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = P6; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = P7; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = P8; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = P9; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = P10; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = P11; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); rax = P12; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); rax = P13; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rax); rax = P14; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); rax = P15; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rax); rax = P16; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); rax = P17; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), rax); rax = P18; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rax = P19; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rax); rax = P20; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); rax = P21; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rax); rax = P22; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rax = P23; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); rax = P24; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); rax = P25; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); rax = P26; crate::simd_ops::wu64(rbp, rax); rax = P27; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); rax = P28; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); rax = P29; crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rax); rax = P30; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rax); rax = P31; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = P32; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); rax = P33; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rax); rax = P34; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rax); rax = P35; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), rax); rax = P36; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rax); rax = P37; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rax); rax = P38; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rax); rax = P39; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rax); rax = P40; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), rax); r14 = rsp.wrapping_add(0x30); rax = P41; crate::simd_ops::wu64(rbp.wrapping_add(0xf0u64), r15); crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), rax); r8 = 0; fl_l = 0; fl_r = 0; rax = P42; crate::simd_ops::wu64(rbp.wrapping_add(0xf8u64), r15); crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), rax); rdx = 0; fl_l = 0; fl_r = 0; rax = P43; crate::simd_ops::wu64(rbp.wrapping_add(0x88u64), rax); rax = P44; crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), rax); rax = P45; crate::simd_ops::wu64(rbp.wrapping_add(0x98u64), rax); rax = P46; crate::simd_ops::wu64(rbp.wrapping_add(0xa0u64), rax); rax = P47; crate::simd_ops::wu64(rbp.wrapping_add(0xa8u64), rax); rax = P48; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), rax); rax = P49; crate::simd_ops::wu64(rbp.wrapping_add(0xb8u64), rax); rax = P50; crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), rax); rax = P51; crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), rax); rax = P52; crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), rax); rax = P53; crate::simd_ops::wu64(rbp.wrapping_add(0xd8u64), rax); rax = P54; crate::simd_ops::wu64(rbp.wrapping_add(0xe0u64), rax); rax = P55; crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), rax); rax = crate::simd_ops::call_stk(P56, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rcx = rdi; rax = crate::simd_ops::call_stk(P57, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = (0xffffffffffffffffu64) as u64; bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(r8))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P58, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rbp.wrapping_add(0x100); r9 = ((r9).wrapping_sub(0x18u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x100u64), rax); rdx = rsi.wrapping_sub(0x30); crate::simd_ops::wu32(rbp.wrapping_add(0x110u64), (0x5u32) as u32); rcx = rdi; rax = crate::simd_ops::call_stk(P59, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x8u64), (0xffffffffffffffe8u64) as u64); r14 = ((r14).wrapping_add(0x10u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax.wrapping_sub(0x18u64))) as u64; r11 = rsp.wrapping_add(0x220); rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x4u64), (0x1u8) as u8); rax = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b71610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x28; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rdx; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (((xmm0 >> 64) as u64)) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; bb = 12; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (((xmm0 >> 64) as u64)) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { rax = 0x2; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b71e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141bab7d0; const P1: u64 = 0x141bd32c0; const P2: u64 = 0x143427f30; const P3: u64 = 0x1434283b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = r8; rbp = rcx; r8 = ((rdx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xb0u64))) as u64; rdx = (((rbx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rsi = ((rax.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x48u64)))) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = rbx; r8 = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0xa0u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu128(rbp.wrapping_add(0x90u64), (xmm2) as u128); xmm1 = 0u128; xmm1 = crate::simd_ops::vinsertps(xmm1, xmm0, 0x10u8); xmm0 = crate::simd_ops::vaddps(xmm1, xmm2); crate::simd_ops::wu128(rbp.wrapping_add(0x80u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rsi)) as u64; r9 = rsp.wrapping_add(0x30); r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x100u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(0xa8u64), (0x40000000u32) as u32); rax = rsp.wrapping_add(0x70); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x80u64))) as u128; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x90u64))) as u128; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xacu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((r8.wrapping_add(0x3u64)) as u32) as u32 as u64; bb = 11; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0x4; bb = 11; continue; }
            7 => { xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(0x60u64))) as u128; xmm2 = crate::simd_ops::vsubps(xmm1, crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))); xmm1 = (crate::simd_ops::ru128(P2)) as u128; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x84u64))) as u32) as u32 as u64; xmm4 = crate::simd_ops::vdpps(xmm2, xmm2, 0x7fu8); xmm0 = crate::simd_ops::vsqrtps(xmm4); xmm3 = crate::simd_ops::vdivps(xmm2, xmm0); xmm2 = crate::simd_ops::vcmpps(xmm1, xmm4, 0x2u8); xmm3 = (xmm3) & (xmm2); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P3)); crate::simd_ops::wu8(rdx.wrapping_add(0x50u64), ((rax as u8)) as u8); rax = 0x1; crate::simd_ops::wu128(rdx.wrapping_add(0x20u64), (xmm1) as u128); bb = 11; continue; }
            8 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x84u64))) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rdx.wrapping_add(0x50u64), ((rax as u8)) as u8); xmm2 = 0u128; xmm0 = 0u128; xmm1 = crate::simd_ops::vmulps(xmm2, xmm0); crate::simd_ops::wu128(rdx.wrapping_add(0x20u64), (xmm1) as u128); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x18u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x28u64), ((rcx as u32)) as u32); crate::simd_ops::wu128(rax.wrapping_add(0x30u64), (xmm2) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x84u64), rcx); crate::simd_ops::wu32(rax.wrapping_add(0x80u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x8cu64), rcx); crate::simd_ops::wu8(rax.wrapping_add(0xb8u64), ((rcx as u8)) as u8); crate::simd_ops::wu32(rax.wrapping_add(0xacu64), ((rcx as u32)) as u32); bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b72190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba4aa0; const P1: u64 = 0x143222c88; const P2: u64 = 0x1430f6b98; const P3: u64 = 0x38; const P4: u64 = 0x8; const P5: u64 = 0x40; const P6: u64 = 0xc; const P7: u64 = 0x44; const P8: u64 = 0x10; const P9: u64 = 0x48; const P10: u64 = 0x14; const P11: u64 = 0x4c; const P12: u64 = 0x143222d20; const P13: u64 = 0x1431f8560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = P1; crate::simd_ops::wu64(rdi, rax); rax = P2; crate::simd_ops::wu64(rdi.wrapping_add(P3), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P11), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x50u64), ((rax as u8)) as u8); rax = P12; crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = P13; crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0xa0u64), ((rax as u32)) as u32); rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b72220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa8; const P1: u64 = 0xa0; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x10; const P4: u64 = 0xb1; const P5: u64 = 0x141b72430; const P6: u64 = 0x28; const P7: u64 = 0x8; const P8: u64 = 0x140657cc0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rsi = rdx; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; r10 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(r10.wrapping_mul(8)).wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = rdi; rbx = ((crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((rdi.wrapping_add(P6)) as u64) as u64; rdx = rsi; rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b722a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b72220;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xa8u64))) as u32 as u64; fl_r = 0x0; rsi = rdx; r11 = rcx; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 22; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rbp = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; r8 = (crate::simd_ops::ru64(r11.wrapping_add(0xa0u64))) as u64; rcx = rdx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(rdi).wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            6 => { rdx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(rdi).wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 11; continue; }
            11 => { r8 = rdx; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r11; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 16; continue; }
            16 => { r9 = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64; r10 = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; rax = ((r9 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r10)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rdx = ((r9 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0xfffffffffffffff0u64)) as u64; rdx = ((rdx).wrapping_add(r10)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_sub_u32(r8.wrapping_add(0x40u64), (1) as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(r11.wrapping_add(0xa8u64))) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 21; continue; }
            21 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu32(r11.wrapping_add(0xa8u64), (0x0u32) as u32); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            22 => { crate::simd_ops::wu32(rcx.wrapping_add(0xa8u64), (0x0u32) as u32); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b723d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b72220; const P1: u64 = 0x14023e450; const P2: u64 = 0x141bcd380;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rax = P0;
    rcx = ((rcx).wrapping_add(0xa0u64)) as u64;
    rdx = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rbx = r8;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    r8 = rsp.wrapping_add(0x20);
    rdx = rbx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b72430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b722a0; const P1: u64 = 0x14023bf50; const P2: u64 = 0x141b72220; const P3: u64 = 0x14023e450; const P4: u64 = 0x141bcd380; const P5: u64 = 0x14023cdf0; const P6: u64 = 0x140200070;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r15 = rdx; rsi = rcx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsi.wrapping_add(0xb1u64), (0x0u8) as u8); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x78u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0xd8u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdx); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rdx = rsp.wrapping_add(0x40); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rcx = rsi.wrapping_add(0xa0); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = rsp.wrapping_add(0x20); rcx = r15; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x78u64))) as u32 as u64; rbp = 0x1; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 11; continue; } bb = 2; continue; }
            2 => { r14 = ((rbp.wrapping_add(0x17u64)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; rbx = (crate::simd_ops::ru64(r15.wrapping_add(0xe8u64))) as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0xa0u64))) as u64; r8 = ((r8).wrapping_add(0xffffffffffffffe8u64)) as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(rbp.wrapping_mul(8)))) as u64; r8 = ((r8).wrapping_add(r14)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0xb0u64)))) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0xb0u64)))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x30u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = rsp.wrapping_add(0x58); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rdx = rsp.wrapping_add(0x58); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rcx = rsi.wrapping_add(0xa0); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = rsp.wrapping_add(0x20); rcx = r15; rdx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x78u64))) as u32 as u64; rbp = ((rbp).wrapping_add(1)) as u64; r14 = ((r14).wrapping_add(0x18u64)) as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 10; continue; }
            10 => { rcx = ((rax as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xa8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rsi.wrapping_add(0xb1u64), ((rax as u8)) as u8); bb = 13; continue; }
            13 => { r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141b725f4, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b72600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240aa00; const P1: u64 = 0x48; const P2: u64 = 0x140636a20; const P3: u64 = 0x141b72430;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rdx;
    rbp = rcx;
    rdx = r8;
    rcx = ((rcx).wrapping_add(0x28u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rcx = rbp.wrapping_add(0x70);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rsi;
    rcx = rbp;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu8(rbp.wrapping_add(0xb0u64), (0x1u8) as u8);
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b72670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b722a0; const P1: u64 = 0xb0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x0u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b726a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143222d60; const P1: u64 = 0x1401de830; const P2: u64 = 0x70; const P3: u64 = 0x1403b8050; const P4: u64 = 0x28; const P5: u64 = 0x14240ab10; const P6: u64 = 0x18; const P7: u64 = 0x1401e3450; const P8: u64 = 0x8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0xa0u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P8)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b72700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143222d60; const P1: u64 = 0x80; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x2b; const P8: u64 = 0x2c; const P9: u64 = 0x30; const P10: u64 = 0x78; const P11: u64 = 0x88;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = ((rcx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rdx);
    crate::simd_ops::lk_and_u32(rcx.wrapping_add(P6), (0xff000008u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rcx.wrapping_add(P6), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu8(rcx.wrapping_add(P7), (0x8u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P8), ((rdx as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P9), ((rdx as u8)) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rax);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(0x78u64), ((rdx as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(0x7cu64), (0x4u32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), rdx);
    crate::simd_ops::wu16(rcx.wrapping_add(0xb0u64), ((rdx as u16)) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b72780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1a0; const P1: u64 = 0x14052f984; const P2: u64 = 0x1401dffd0; const P3: u64 = 0xf0; const P4: u64 = 0x141b722a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rdi = rdx;
    rbx = rcx;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = ((rcx).wrapping_add(0xf8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x0u8) as u8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b727f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa0; const P1: u64 = 0x14240aa00; const P2: u64 = 0x48; const P3: u64 = 0x160; const P4: u64 = 0x140636a20; const P5: u64 = 0xf0; const P6: u64 = 0x141b72430; const P7: u64 = 0x14052f984; const P8: u64 = 0x1a0; const P9: u64 = 0xf8; const P10: u64 = 0x1401e2660;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, r14);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = ((r8.wrapping_add(P0)) as u64) as u64;
    rsi = rdx;
    r14 = rcx;
    rdx = rbx;
    rcx = ((rcx).wrapping_add(0x118u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((r14.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = rsi;
    rcx = ((r14.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rax = P7;
    crate::simd_ops::wu8(r14.wrapping_add(P8), (0x1u8) as u8);
    rcx = ((r14.wrapping_add(P9)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14);
    rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    r14 = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b728f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x14240aa00; const P2: u64 = 0x48; const P3: u64 = 0xa8; const P4: u64 = 0x140636a20; const P5: u64 = 0x38; const P6: u64 = 0x141b72430; const P7: u64 = 0x14052f984; const P8: u64 = 0xe8; const P9: u64 = 0x40; const P10: u64 = 0x1401e2660;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, r14);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = ((r8.wrapping_add(P0)) as u64) as u64;
    rsi = rdx;
    r14 = rcx;
    rdx = rbx;
    rcx = ((rcx).wrapping_add(0x60u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((r14.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = rsi;
    rcx = ((r14.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rax = P7;
    crate::simd_ops::wu8(r14.wrapping_add(P8), (0x1u8) as u8);
    rcx = ((r14.wrapping_add(P9)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14);
    rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    r14 = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b72990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14052f984; const P1: u64 = 0x1401dffd0; const P2: u64 = 0x141b722a0; const P3: u64 = 0x141b728f0; const P4: u64 = 0x141b72780; const P5: u64 = 0x141b727f0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xe9u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = rcx; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r14); r14 = rdx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xe8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r15); r15 = (crate::simd_ops::ru64(rdx.wrapping_add(0xf0u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = P0; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rcx); rcx = ((rcx).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rax); rdx = r11.wrapping_sub(0x28); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = r15; rcx = rdi.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xe8u64), (0x0u8) as u8); bb = 3; continue; }
            3 => { r8 = rbp; rdx = r14; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0xa4u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x1a1u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0xf0u64))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r8 = rbp; rdx = r14; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = r14; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1a8u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rdi.wrapping_add(0x1a8u64), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((if (((fl_l as u32)) <= ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            11 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x24u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu8(rcx.wrapping_add(0x41u64), (0x1u8) as u8); bb = 15; continue; }
            15 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b72aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xd8; const P2: u64 = 0xe8; const P3: u64 = 0x28; const P4: u64 = 0x1405e37f0; const P5: u64 = 0x8; const P6: u64 = 0xa4; const P7: u64 = 0xa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rsi = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; r8 = ((rdi.wrapping_add(P3)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; r8 = ((rdi.wrapping_add(P7)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b72b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x20; const P3: u64 = 0xe8; const P4: u64 = 0x14052f984; const P5: u64 = 0x40; const P6: u64 = 0x1401dffd0; const P7: u64 = 0x38; const P8: u64 = 0x141b722a0; const P9: u64 = 0x18; const P10: u64 = 0xa4; const P11: u64 = 0x141b72780;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); rbp = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rsi = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rbx = rax; rdi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rcx = ((rsi.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = ((rsi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsi.wrapping_add(P3), (0x0u8) as u8); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P9))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P10))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b72c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x20; const P3: u64 = 0x18; const P4: u64 = 0x8; const P5: u64 = 0x1a8; const P6: u64 = 0x141b728f0; const P7: u64 = 0xa4; const P8: u64 = 0x141b727f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); rsi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r14 = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rbx = rax; rdi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = rsi; rbx = (crate::simd_ops::ru64(r14.wrapping_add(P3))) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; rdx = rsi; rcx = r14; crate::simd_ops::wu32(r14.wrapping_add(P5), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = rbx; rdx = rsi; rcx = r14; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b72ca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141bc8a50; const P1: u64 = 0xe8; const P2: u64 = 0x8; const P3: u64 = 0x14052f984; const P4: u64 = 0x40; const P5: u64 = 0x1401dffd0; const P6: u64 = 0x38; const P7: u64 = 0x141b722a0; const P8: u64 = 0x18; const P9: u64 = 0xa4; const P10: u64 = 0x141b72780;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rsi = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbp = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rbp; rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P1), (0x0u8) as u8); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P9))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b734b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e7c10; const P1: u64 = 0x143c8f4d0; const P2: u64 = 0x142489ee0; const P3: u64 = 0x143426d28;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; r15 = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x8u64))) as u128; r13 = rdx; rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = r15.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; rdx = (((xmm1 >> 64) as u64)) as u64; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x58u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), ((xmm0 as u64)) as u64); fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; r11 = (crate::simd_ops::ru64(r15.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; r12 = ((rcx as u32)) as u32 as u64; bb = 5; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; r12 = ((rcx as u32)) as u32 as u64; bb = 5; continue; }
            4 => { r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rbx); fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rbx = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(r15).wrapping_add(0x180acu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(r15).wrapping_add(0x18098u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = rbx.wrapping_add(0xc03); rax = ((rax).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r15).wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax).wrapping_add(r11)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 11; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 11; continue; }
            11 => { fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru64(P3)) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), rsi); bb = 13; continue; }
            13 => { xmm1 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) * f64::from_bits(crate::simd_ops::ru64(r13.wrapping_add(0x10u64)))).to_bits() as u128); rcx = (((r14) as i64).wrapping_mul((0xe0u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(1)) as u64; rdx = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm1 as u64)))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rax).wrapping_add(0xd0u64), rdx); fl_l = r14; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { r11 = ((r11).wrapping_add(0x2u64)) as u64; fl_l = r11; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rcx = ((crate::simd_ops::ru16(r11)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { r11 = ((r11).wrapping_add(0x2u64)) as u64; fl_l = r11; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 19; continue; }
            19 => { bb = 28; continue; }
            20 => { fl_l = r11; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 21; continue; }
            21 => { rsi = ((crate::simd_ops::ru16(r11)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(r15.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 24; continue; }
            23 => { rax = 0; fl_l = 0; fl_r = 0; bb = 24; continue; }
            24 => { r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rcx = rsi; rcx = ((rcx).wrapping_shl(5u32)) as u64; r14 = 0; fl_l = 0; fl_r = 0; r12 = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(r15).wrapping_add(0x180acu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(r15).wrapping_add(0x18098u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = rsi.wrapping_add(0xc03); rax = ((rax).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r15).wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 26; continue; }
            26 => { fl_l = ((rax).wrapping_add(rbp)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 28; continue; }
            27 => { rax = 0; fl_l = 0; fl_r = 0; bb = 28; continue; }
            28 => { fl_l = r14; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 29; continue; }
            29 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; bb = 30; continue; }
            30 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rsp = ((rsp).wrapping_add(0xa8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b736f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc2; const P1: u64 = 0x143426780; const P2: u64 = 0x143426650; const P3: u64 = 0x143426fc8; const P4: u64 = 0x140658330; const P5: u64 = 0x1425371a0; const P6: u64 = 0x142533200; const P7: u64 = 0x14384e018; const P8: u64 = 0x14384e028; const P9: u64 = 0x1402250d0; const P10: u64 = 0x14023d390; const P11: u64 = 0x14023d120; const P12: u64 = 0x14023d300; const P13: u64 = 0x14384e030; const P14: u64 = 0x14384e038; const P15: u64 = 0x14384e040;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 75>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b736f0, funcinfo_rva: 0x3738150, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x258, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x150u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x250u64)) as u64; rbx = (r8) as u64; rdi = (rdx) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0xe0u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b73727); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128; ymm2_hi = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm2 as u32)) as u32); xmm3 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm3 as u32)) as u32); rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rsi) as u64); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = 0u128; ymm1_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm1) as u128); r14 = (0x20u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (r14) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rsi) as u64); let y_addr: u64 = rbp.wrapping_sub(0x50u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x50u64), ((rsi as u8)) as u8); crate::simd_ops::wu8(rbp.wrapping_sub(0x40u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rsi) as u64); xmm1 = 0u128; ymm1_hi = 0; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((r14) as i64) as f32).to_bits() as u128); ymm1_hi = 0; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); ymm2_hi = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (0x8000000000000000u64) as u64; xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; ymm0_hi = 0; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); ymm2_hi = 0; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (rdx) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm2 as u32)))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); xmm2 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); ymm2_hi = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); ymm2_hi = 0; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (rdx) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm2 as u32)))) as u64; rax = ((rax).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), ((rsi as u8)) as u8); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), (rsi) as u64); crate::simd_ops::wu64(rbp, (rsi) as u64); xmm0 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b73836); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rcx = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b738a8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            11 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            12 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b738ba); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), (rsi) as u64); r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b73931); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            19 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            20 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b73943); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            22 => { xmm2 = 0u128; ymm2_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm2) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (0xffffffffffffffffu64) as u64); xmm0 = (crate::simd_ops::ru128(P7)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(P8)) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x30u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu8(rbx.wrapping_add(0x19u64), (0x1u8) as u8); let y_addr: u64 = rbp.wrapping_add(0x40u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbx.wrapping_add(0x60u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x80u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm2); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm2_hi); crate::simd_ops::wu128(rbx.wrapping_add(0xa0u64), (xmm1) as u128); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7399e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), (rax) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 23; continue; }
            23 => { bb = 24; continue; }
            24 => { let y_addr: u64 = rbx.wrapping_add(0x60u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rax; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x80u64); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rax.wrapping_add(0x20u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm1); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm1_hi); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0xa0u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax.wrapping_add(0x40u64), (xmm0) as u128); rax = ((rax).wrapping_add(0x50u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { r8 = (r14) as u64; rdx = (rbx) as u64; rcx = ((rbp.wrapping_add(0x90u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b73a03); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_add(0x90u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b73a12); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x90u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b73a1e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 27; continue; }
            27 => { crate::simd_ops::wu8(rbx.wrapping_add(0x1au64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(P13)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (rax) as u64); rax = (crate::simd_ops::ru64(P14)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (rax) as u64); rax = (crate::simd_ops::ru64(P15)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (rax) as u64); r11 = ((rsp.wrapping_add(0x250u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b73a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14023bf50; const P1: u64 = 0x14384e040; const P2: u64 = 0x14384e030; const P3: u64 = 0x140200070;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r14 = (crate::simd_ops::ru32(r9.wrapping_add(0x50u64))) as u32 as u64; rbx = r8; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((r14 as u32)) as u32); rsi = rdx; r12 = rcx; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { r15 = (crate::simd_ops::ru64(r9.wrapping_add(0x48u64))) as u64; rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); rbp = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rcx = rax; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 22; continue; } bb = 2; continue; }
            2 => { r13 = 0x1; bb = 3; continue; }
            3 => { rcx = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = (crate::simd_ops::ru64(P1)) as u64; rdi = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; fl_l = ((r9).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0xb0u64)))) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 17; continue; }
            6 => { r15 = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; rcx = rsp.wrapping_add(0x48); rbp = 0; fl_l = 0; fl_r = 0; r14 = r15.wrapping_sub(0x1); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xb0u64))) as u64; r10 = rbp.wrapping_sub(0x1); r11 = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; r9 = rax; r9 = (((r9) & (r14))) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = r11; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = ((rcx.wrapping_add(rax)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = r10; fl_r = (0xffffffffffffffffu64) as u64; r8 = rcx; r10 = (if (((fl_l as u64)) == ((fl_r as u64))) { r9 } else { r10 }) as u64; bb = 12; continue; }
            10 => { rdx = ((rax.wrapping_add(rcx)) as u64) as u64; bb = 11; continue; }
            11 => { r8 = rcx; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rbp = ((rbp).wrapping_add(1)) as u64; r9 = ((r9).wrapping_add(rbp)) as u64; r9 = (((r9) & (r14))) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = r11; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 13; continue; }
            13 => { rcx = r8; bb = 15; continue; }
            14 => { fl_l = r9; fl_r = (0xffffffffffffffffu64) as u64; r9 = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { r9 = ((r15.wrapping_add(r15.wrapping_mul(4))) as u64) as u64; bb = 16; continue; }
            16 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; fl_l = ((r9).wrapping_add(rcx)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0xb0u64)))) as u64; fl_l = r9; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(r9.wrapping_add(0x40u64))) as u64; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(r9.wrapping_add(0x30u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 20; continue; }
            20 => { r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; rcx = rsp.wrapping_add(0x30); rdi = (crate::simd_ops::ru64(r15.wrapping_add(r13.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); r13 = ((r13).wrapping_add(1)) as u64; rax = ((r14 as u32)) as u32 as u64; fl_l = r13; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 21; continue; }
            21 => { rbp = rdi; rsi = r9; bb = 22; continue; }
            22 => { crate::simd_ops::wu64(r12, rsi); crate::simd_ops::wu64(r12.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(r12.wrapping_add(0x10u64), rcx); bb = 24; continue; }
            23 => { xmm0 = (crate::simd_ops::ru128(P2)) as u128; xmm1 = ((crate::simd_ops::ru64(P1)) as u64) as u128; crate::simd_ops::wu128(r12, (xmm0) as u128); crate::simd_ops::wu64(r12.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = r12; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141b73c9d, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b73cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241c0a0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x14241c190; const P3: u64 = 0x14023cdf0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rdx; rbx = rcx; r14 = 0; fl_l = 0; fl_r = 0; r15 = ((r14 as u32)) as u32 as u64; r9 = ((r14 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0xb0u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(rsi)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rsi = ((rsi).wrapping_add(0x50u64)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { r10 = (0x2aaaaaaaaaaaaaabu64) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0xb0u64)))) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), ((xmm1 as u64)) as u64); rax = ((xmm0 as u64)) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 9; continue; }
            9 => { rax = ((r15 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rcx.wrapping_mul(8)) as u64) as u64; r12 = ((rdx.wrapping_add(r14)) as u64) as u64; rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx.wrapping_mul(8)) as u64) as u64; r8 = ((rcx.wrapping_add(r14)) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { r12 = ((r12).wrapping_sub(r14)) as u64; rax = r10; let prod = (rax as i64 as i128).wrapping_mul((r12) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r15 = rdx; r15 = ((((r15) as i64).wrapping_shr(2u32) as u64)) as u64; rax = r15; rax = ((rax).wrapping_shr(63u32)) as u64; r15 = ((r15).wrapping_add(rax)) as u64; rcx = r15.wrapping_add(0x1); r8 = ((r8).wrapping_sub(r14)) as u64; rax = r10; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = rdx; rax = ((rax).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rax; fl_r = rcx; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rdx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = rax; r8 = r12; rdx = r14; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { r14 = r13; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64) as u128; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r10 = (0x2aaaaaaaaaaaaaabu64) as u64; bb = 13; continue; }
            13 => { rax = ((r15 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu128(r14.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); crate::simd_ops::wu64(r14.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = (((r15 as u32)).wrapping_add(1)) as u32 as u64; r15 = ((fl_l as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rsi = ((rsi).wrapping_add(0x50u64)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 19; continue; }
            19 => { rsi = ((rsi).wrapping_add(0x50u64)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 20; continue; }
            20 => { bb = 7; continue; }
            21 => { rbx = r14; rax = ((r15 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((r14.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r14; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 22; continue; }
            22 => { bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0xb0u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 28; continue; }
            25 => { r8 = rbx; rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xb0u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; bb = 27; continue; }
            27 => { rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0xb0u64)))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rdx, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); crate::simd_ops::wu8(rdx.wrapping_add(0x20u64), (0x0u8) as u8); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x30u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu8(rdx.wrapping_add(0x30u64), (0x0u8) as u8); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(rdx.wrapping_add(0x40u64), (0xffffffffffffffffu64) as u64); fl_l = (crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x40u64), (1) as u64)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x18u64), (0x1u8) as u8); bb = 32; continue; }
            32 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 33; continue; }
            33 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b74140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xf0; const P2: u64 = 0x18; const P3: u64 = 0x98; const P4: u64 = 0x41;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((if (((fl_l as u32)) <= ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P4), (0x1u8) as u8); bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b74190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x99; const P2: u64 = 0xd8; const P3: u64 = 0xe8; const P4: u64 = 0x1405e37f0; const P5: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdi = ((crate::simd_ops::ru8(r8.wrapping_add(P1))) as u32) as u32 as u64; } else { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); }
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    r8 = ((r8).wrapping_add(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = 0x2;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b74200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x20; const P3: u64 = 0x14052f984; const P4: u64 = 0x1401dffd0; const P5: u64 = 0xf0; const P6: u64 = 0x141b722a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); rsi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rbx = rax; rdi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rcx = rbp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; rcx = rbp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0xe8u64), (0x0u8) as u8); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b742a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x20; const P3: u64 = 0x18; const P4: u64 = 0x8; const P5: u64 = 0xf0; const P6: u64 = 0x60; const P7: u64 = 0x14240aa00; const P8: u64 = 0x68; const P9: u64 = 0xa8; const P10: u64 = 0x140636a20; const P11: u64 = 0x38; const P12: u64 = 0x141b72430; const P13: u64 = 0x14052f984; const P14: u64 = 0x1401e2660;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); rbp = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r14 = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rbx = rax; rdi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = rbp; rbx = (crate::simd_ops::ru64(r14.wrapping_add(P3))) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu32(r14.wrapping_add(P5), ((rax as u32)) as u32); rcx = ((r14.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P8)) as u64) as u64; rcx = ((r14.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = rbp; rcx = ((r14.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = P13; crate::simd_ops::wu8(r14.wrapping_add(0xe8u64), (0x1u8) as u8); rcx = r14.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b74380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141bc8a50; const P1: u64 = 0xe8; const P2: u64 = 0x14052f984; const P3: u64 = 0x40; const P4: u64 = 0x1401dffd0; const P5: u64 = 0x8; const P6: u64 = 0x38; const P7: u64 = 0x141b722a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P1), (0x0u8) as u8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b74b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b74c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba4aa0; const P1: u64 = 0x143223570; const P2: u64 = 0x1430f6b98; const P3: u64 = 0x38; const P4: u64 = 0x8; const P5: u64 = 0x40; const P6: u64 = 0xc; const P7: u64 = 0x44; const P8: u64 = 0x10; const P9: u64 = 0x48; const P10: u64 = 0x14; const P11: u64 = 0x4c; const P12: u64 = 0x1430f6d50; const P13: u64 = 0x1431f8560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    rax = P2;
    crate::simd_ops::wu64(rdi.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P11), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(0x50u64), ((rax as u8)) as u8);
    rax = P12;
    crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rax);
    rax = P13;
    crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax);
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b74e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e7c10; const P1: u64 = 0x140659130; const P2: u64 = 0x143c8f4d4; const P3: u64 = 0x14020e8f0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rbp = rsp.wrapping_sub(0x37); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; r11 = (crate::simd_ops::ru64(rcx)) as u64; r8 = rbp.wrapping_sub(0x49); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x8u64))) as u128; r13 = rdx; rcx = rbp.wrapping_sub(0x39); r11 = ((r11).wrapping_add(0x40u64)) as u64; rdx = r11; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64) as u128; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), r11); rax = (((xmm1 >> 64) as u64)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rdi); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), rdi); fl_l = rax; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 4; continue; }
            3 => { rax = ((rdi as u32)) as u32 as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rax); bb = 5; continue; }
            5 => { rcx = rbp.wrapping_sub(0x19); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = rsi; fl_r = r12; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 33; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), r15); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(rsi.wrapping_mul(4)))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x27u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; r14 = (crate::simd_ops::ru64(r13)) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = rbx; rcx = ((rcx).wrapping_shr(32u32)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = rdi; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = 0x1; bb = 12; continue; }
            12 => { crate::simd_ops::lk_add_u64(r14.wrapping_add(0x68u64), (rax) as u64); r15 = r14.wrapping_add(0x160); crate::simd_ops::wu64(r14.wrapping_add(0x60u64), rbx); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rdi = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x4000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64), ((rbx as u32)) as u32); rbx = r14.wrapping_add(0x178); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(r15.wrapping_add(0x10u64), (1) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (0x8000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x10u64), ((rdi as u16)) as u16); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64); rdi = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rsi = ((rsi).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), rsi); fl_l = rsi; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rcx); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; bb = 22; continue; }
            22 => { rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rcx = ((rcx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rcx); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 25; continue; }
            25 => { bb = 31; continue; }
            26 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 27; continue; }
            27 => { rdx = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 30; continue; }
            29 => { rax = ((rdi as u32)) as u32 as u64; bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), rdi); bb = 31; continue; }
            31 => { rcx = rbp.wrapping_sub(0x19); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = rsi; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 32; continue; }
            32 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 33; continue; }
            33 => { rsp = ((rsp).wrapping_add(0xa0u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b75070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e7c10; const P1: u64 = 0x140659130; const P2: u64 = 0x1406598d0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; r11 = (crate::simd_ops::ru64(rcx)) as u64; r8 = rbp.wrapping_add(0x47); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x8u64))) as u128; rsi = rdx; rcx = rbp.wrapping_add(0x27); r11 = ((r11).wrapping_add(0x40u64)) as u64; rdx = r11; crate::simd_ops::wu128(rbp.wrapping_add(0x47u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x27u64))) as u128; xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64) as u128; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r11); rax = (((xmm1 >> 64) as u64)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x21u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), r14); fl_l = rax; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 4; continue; }
            3 => { rax = ((r14 as u32)) as u32 as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); bb = 5; continue; }
            5 => { rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; rdx = rbp.wrapping_add(0x47); rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x2fu64), ((r14 as u8)) as u8); rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), rax); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x27u64))) as u64) as u128; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2fu64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x47u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x4fu64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rbx = ((rbx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rbx); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rcx); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = ((rcx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rcx); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { bb = 21; continue; }
            16 => { fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 17; continue; }
            17 => { rdx = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 20; continue; }
            19 => { rax = ((r14 as u32)) as u32 as u64; bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), r14); bb = 21; continue; }
            21 => { rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 22; continue; }
            22 => { r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b75220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14278ce90; const P1: u64 = 0x1410d6ca0; const P2: u64 = 0x140659210; const P3: u64 = 0x143823510; const P4: u64 = 0x14020e980; const P5: u64 = 0x14020afa0; const P6: u64 = 0x1420a6b70; const P7: u64 = 0x140659570;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbx = rcx.wrapping_add(0x8); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x58u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x68); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rsi = rbx.wrapping_add(0x8); r15 = 0xffff; r12 = (0x2aaaaaaaaaaaaaabu64) as u64; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = r12; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rdi.wrapping_add(0x1); rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r14, ((rdi as u16)) as u16); bb = 4; continue; }
            4 => { r9 = 0x1; r8 = (((rbp as u16)) as u32) as u32 as u64; rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x68); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = r12; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rdi.wrapping_add(0x1); rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r14, ((rdi as u16)) as u16); bb = 8; continue; }
            8 => { r9 = 0x1; r8 = (((rbp as u16)) as u32) as u32 as u64; rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b754c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b75510;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    xmm1 = (xmm2) as u128;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !((((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0)) { xmm2 = (xmm3) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rbx as u32)) as i32) as f32).to_bits() as u128); xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = 0u128;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b75510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143426d28;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm0 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm1 as u32)) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128); xmm1 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) * f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128); r10 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm0 as u64)))) as u64; r11 = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm1 as u64)))) as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = ((((r8 as u32)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = r11; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdx.wrapping_add(P0)))) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_sub(0x10u64)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { rcx = ((r9 as u32)) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rax = ((rcx.wrapping_add(r9)) as u32) as u32 as u64; fl_l = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b755f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143224e78; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); xmm1 = (crate::simd_ops::ru128(rcx)) as u128; rcx = (crate::simd_ops::ru64(rcx)) as u64; rbx = r9; rax = (((xmm1 >> 64) as u64)) as u64; rdi = r8; rsi = ((rdx as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (((xmm1 >> 64) as u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); rbp = P0; bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru64(rax)) as u64; r11 = r10; r11 = ((r11).wrapping_shr(32u32)) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rdx = ((r10 as u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; r9 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rsi as u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(r8.wrapping_mul(8)), rbp); rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P1), ((rdx as u8)) as u8); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P2), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P3), ((rdx as u32)) as u32); fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b759c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14; const P2: u64 = 0x4; const P3: u64 = 0x10; const P4: u64 = 0x143c8fd28; const P5: u64 = 0x580e0; const P6: u64 = 0x40; const P7: u64 = 0x18; const P8: u64 = 0x48; const P9: u64 = 0x4c; const P10: u64 = 0x143c8d11c;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = rcx; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rax.wrapping_add(P2), ((xmm1 as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = (((crate::simd_ops::ru32(rax.wrapping_add(P3))) as i32 as i64) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rdx = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; r8 = ((r8).wrapping_add(r8)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P6))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P9), ((xmm1 as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P8), (0x1u8) as u8); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; rdx = ((crate::simd_ops::ru16(P10)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(r10)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(0x40u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 9; continue; }
            9 => { return rax; }
            10 => { crate::simd_ops::wu32(rax.wrapping_add(P3), ((xmm1 as u32)) as u32); bb = 11; continue; }
            11 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b75a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b759c0; const P1: u64 = 0x143076988; const P2: u64 = 0x143c8d11c; const P3: u64 = 0x141b75dc0; const P4: u64 = 0x143428b00; const P5: u64 = 0x1434268d0; const P6: u64 = 0x143c8fd28; const P7: u64 = 0x14211e0e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm2_hi: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = rcx; xmm2 = (xmm1) as u128; ymm2_hi = 0; xmm1 = 0u128; crate::simd_ops::wu8(rax.wrapping_add(0x30u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r11, ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x14u64), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0xffffffffu32) as u32); rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm6) as u128); xmm6 = (crate::simd_ops::ru128(rcx.wrapping_add(0x30u64))) as u128; xmm1 = crate::simd_ops::vsubps(xmm6, crate::simd_ops::ru128(rax.wrapping_add(0x20u64))); xmm0 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P1)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 17; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x14u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rax.wrapping_add(0x14u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rdi); r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; bb = 6; continue; }
            6 => { rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_shl(6u32)) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(rdx).wrapping_add(0x18u64))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { bb = 13; continue; }
            9 => { r8 = rdx.wrapping_add(0x40); r9 = ((r9).wrapping_add(rdx)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = r8.wrapping_sub(0x28); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; r8 = ((r8).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdx, rax); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(0x8u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x58u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((rax as u32)) as u32); rdx = rdx.wrapping_add(0x40); fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(0xffffffffffffffc0u64)) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x20); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm3 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rax)) as u32) as u128; ymm2_hi = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = P4; xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm3 = ((crate::simd_ops::ru32(P5)) as u32) as u128; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x3cu64))) as u64) as u128; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x6cu64), ((xmm0 as u64)) as u64); xmm0 = (ymm2_hi) as u128; xmm1 = (ymm2_hi) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (((xmm0 >> 64) as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x84u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), (((xmm6 >> 32) as u32) as u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x8cu64), (((xmm6 >> 64) as u32) as u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm2 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (((xmm2 >> 64) as u32) as u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), (((xmm1 >> 32) as u32) as u32) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(r8.wrapping_mul(8)))) as u64; rdx = rsp.wrapping_add(0x60); rax = ((rax).wrapping_add(0x40u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu8(rdx.wrapping_add(0x80u64), (0x0u8) as u8); bb = 17; continue; }
            17 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b75ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14; const P2: u64 = 0x143c8d11c; const P3: u64 = 0x580e0; const P4: u64 = 0xffffffffffffffff; const P5: u64 = 0x18; const P6: u64 = 0x40; const P7: u64 = 0xffffffffffffffd8; const P8: u64 = 0x38; const P9: u64 = 0xfffffffffffffff8; const P10: u64 = 0x48; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rax.wrapping_add(P1), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r8 = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; rcx = ((rax.wrapping_add(P4)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; bb = 3; continue; }
            3 => { rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_shl(6u32)) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(rdx).wrapping_add(P5))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { r8 = ((rdx.wrapping_add(P6)) as u64) as u64; r9 = ((r9).wrapping_add(rdx)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = ((r8.wrapping_add(P7)) as u64) as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; r8 = ((r8).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rdx, rax); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P10))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(P0), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P11))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((rax as u32)) as u32); rdx = rdx.wrapping_add(0x40); fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(0xffffffffffffffc0u64)) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b75dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426abc; const P1: u64 = 0x143c8d11c; const P2: u64 = 0x14065b330; const P3: u64 = 0x143224e48; const P4: u64 = 0x143428b00; const P5: u64 = 0x1434268d0; const P6: u64 = 0x143c8fd28; const P7: u64 = 0x14211c8c0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm2_hi: u128 = 0; let mut xmm4: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); xmm7 = (xmm3) as u128; r14 = rdx; rbx = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x14u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 1; continue; }
            1 => { xmm8 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; xmm6 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(0xcu64)))).to_bits() as u128); r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(r8.wrapping_mul(8)))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = rsi.wrapping_add(0x1); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rax); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rcx = P3; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(0x30u64), ((xmm8 as u32)) as u32); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rsi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax, ((xmm8 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x14u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((xmm7 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rax.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { xmm4 = (xmm0) as u128; let y_addr: u64 = P4; xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); xmm3 = ((crate::simd_ops::ru32(P5)) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm2 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (((xmm2 >> 64) as u32) as u32) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x3cu64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x6cu64), ((xmm0 as u64)) as u64); xmm1 = (ymm2_hi) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), (((xmm1 >> 32) as u32) as u32) as u32); xmm0 = (ymm2_hi) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (((xmm0 >> 64) as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x84u64), ((xmm4 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), (((xmm4 >> 32) as u32) as u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x8cu64), (((xmm4 >> 64) as u32) as u32) as u32); rdx = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm8 as u32)) as u32); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; bb = 7; continue; }
            6 => { crate::simd_ops::wu8(rax.wrapping_add(0x10u64), (0x1u8) as u8); crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((xmm6 as u32)) as u32); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rcx as u32)) as u32); bb = 8; continue; }
            8 => { r11 = rsp.wrapping_add(0xc0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b75fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14; const P2: u64 = 0x4; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rax, ((rdx as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P1), ((rdx as u8)) as u8);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P2), ((rdx as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P3), (0xffffffffu32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b75ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x40u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { return rax; }
            4 => { crate::simd_ops::wu32(rax.wrapping_add(P2), ((xmm2 as u32)) as u32); bb = 5; continue; }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b76020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffd8; const P3: u64 = 0x38; const P4: u64 = 0xfffffffffffffff8; const P5: u64 = 0x40; const P6: u64 = 0x48; const P7: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; r9 = ((rax as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(r8).wrapping_add(P1))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { r9 = ((r9).wrapping_add(r8)) as u64; r8 = ((r8).wrapping_add(0x40u64)) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = ((r8.wrapping_add(P2)) as u64) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; r8 = ((r8).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rdx, rax); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P6))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(P0), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32); rdx = ((rdx.wrapping_add(P5)) as u64) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(0xffffffffffffffc0u64)) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P0), (1) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b760d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x14065b330; const P5: u64 = 0x143224e48; const P6: u64 = 0x18; const P7: u64 = 0x20; const P8: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm6 = (xmm3) as u128; xmm7 = (xmm2) as u128; rbx = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = ((rdi.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); xmm8 = (crate::simd_ops::ru128(rdx)) as u128; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rax = P5; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P6), rdi); crate::simd_ops::wu128(rcx.wrapping_add(P7), (xmm8) as u128); crate::simd_ops::wu32(rcx.wrapping_add(P8), ((xmm7 as u32)) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P2), (1) as u32); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b761d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d910; const P1: u64 = 0x1401e7c10; const P2: u64 = 0x14065a9e0; const P3: u64 = 0x141b75dc0; const P4: u64 = 0x141b759c0; const P5: u64 = 0x141b75a80; const P6: u64 = 0x143c8d11c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rax.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0x100u64)) as u64; r11 = (crate::simd_ops::ru64(rdx)) as u64; r8 = rbp.wrapping_sub(0x39); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x8u64))) as u128; ymm0_hi = 0; rbx = rcx; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rcx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; r11 = ((r11).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r12); rdx = r11; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r14); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x580e0u64))) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); r15 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64) as u128; ymm0_hi = 0; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); rax = (((xmm1 >> 64) as u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), r12); fl_l = rax; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 4; continue; }
            3 => { rax = ((r12 as u32)) as u32 as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), rax); bb = 5; continue; }
            5 => { rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x120u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x71u64))) as u64; xmm6 = 0u128; fl_l = rsi; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; rcx = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; rdi = rsi; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rbx); rdi = ((rdi).wrapping_shl(6u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_sub(0x61u64)))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rdi); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; rcx = rsi; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rdx); let y_addr: u64 = rcx.wrapping_add(rax); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; let y_addr: u64 = rbp.wrapping_add(0x17u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rcx = (crate::simd_ops::ru32(rax.wrapping_add(rsi.wrapping_mul(4)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rdi.wrapping_add(0x34u64), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x30u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 9; continue; }
            9 => { rcx = rbp.wrapping_sub(0x9); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x14u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32) as u128; xmm3 = (xmm6) as u128; rdx = rbp.wrapping_sub(0x39); crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (ymm0_hi) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            12 => { xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32) as u128; bb = 14; continue; }
            13 => { xmm1 = (xmm6) as u128; bb = 14; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { crate::simd_ops::wu8(rdi.wrapping_add(0x30u64), ((r12 as u8)) as u8); crate::simd_ops::wu64(rdi, r12); crate::simd_ops::wu8(rdi.wrapping_add(0x14u64), ((r12 as u8)) as u8); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), (0xffffffffu32) as u32); bb = 16; continue; }
            16 => { xmm1 = ((crate::simd_ops::ru32(r15)) as u32) as u128; rcx = rbp.wrapping_sub(0x9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), rsi); fl_l = rsi; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 19; continue; }
            19 => { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 22; continue; }
            22 => { bb = 28; continue; }
            23 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 24; continue; }
            24 => { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 27; continue; }
            26 => { rax = ((r12 as u32)) as u32 as u64; bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), r12); bb = 28; continue; }
            28 => { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x71u64))) as u64; fl_l = rsi; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 29; continue; }
            29 => { rcx = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x580e0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; rsi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 38; continue; } bb = 30; continue; }
            30 => { rdi = ((((rsi as u32)) as i32 as i64) as u64) as u64; fl_l = ((rdi).wrapping_shl(6u32)) as u64; rdi = ((fl_l as u64)) as u64; bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(rax).wrapping_add(0x10u64))) as u32) as u128; ymm0_hi = 0; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 37; continue; } bb = 32; continue; }
            32 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(r15))).to_bits() as u128); ymm0_hi = 0; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rax).wrapping_add(0x10u64), ((xmm0 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 37; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = rax.wrapping_add(0x40); r8 = ((r8).wrapping_add(rdi)) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rdx = r8.wrapping_sub(0x28); bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; r8 = ((r8).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdx, rax); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x48u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_add(0x8u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x58u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((rax as u32)) as u32); rdx = rdx.wrapping_add(0x40); fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(0xffffffffffffffc0u64)) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 37; continue; }
            37 => { rdi = ((rdi).wrapping_sub(0x40u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; rsi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 31; continue; } bb = 38; continue; }
            38 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xf0u64))) as u128; rsp = ((rsp).wrapping_add(0x100u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b76510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14278cfa0; const P1: u64 = 0x1410d6ca0; const P2: u64 = 0x14065abc0; const P3: u64 = 0x143823a58; const P4: u64 = 0x14020e980; const P5: u64 = 0x14020afa0; const P6: u64 = 0x1420a6b70; const P7: u64 = 0x14387cae0; const P8: u64 = 0x14065b300; const P9: u64 = 0x1402116e0; const P10: u64 = 0x143c8d11c; const P11: u64 = 0x143cc12b8; const P12: u64 = 0x14065b910;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r15 = rcx; rbx = rcx.wrapping_add(0x8); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rsi = rbx.wrapping_add(0x8); r12 = 0xffff; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rdi.wrapping_add(0x1); rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r14, ((rdi as u16)) as u16); bb = 4; continue; }
            4 => { r9 = 0x2; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r15)) as u64; rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = (crate::simd_ops::ru32(P7)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P8; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0xb0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0; fl_l = 0; fl_r = 0; r8 = ((crate::simd_ops::ru16(P10)) as u32) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x90u64)))) as u64; rax = (crate::simd_ops::ru32(P11)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = P12; crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), rax); crate::simd_ops::wu8(rdx.wrapping_add(0x38u64), (0x1u8) as u8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), r9); rax = (crate::simd_ops::ru64(r15.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(r8.wrapping_mul(8)), rdx); r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b76720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P0)))).to_bits() as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm0 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b76740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b76cc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P5), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P6))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P6), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b76d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432251b0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b76ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143426564;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x51u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; xmm1 = crate::simd_ops::vsubps(xmm0, crate::simd_ops::ru128(r8)); xmm2 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm4 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) as u32) as u128; bb = 19; continue; }
            2 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm4 = 0u128; crate::simd_ops::wu32(rcx.wrapping_add(0x4cu64), (0x0u32) as u32); bb = 20; continue; }
            4 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x5cu64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm4 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x44u64))) as u32) as u128; bb = 19; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x58u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm4 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) as u32) as u128; bb = 19; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x120u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; xmm2 = (crate::simd_ops::ru128(r9)) as u128; xmm1 = 0u128; xmm0 = (crate::simd_ops::ru128(rax)) as u128; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm3 = 0u128; bb = 11; continue; }
            10 => { xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x44u64))) as u32) as u128; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x121u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 13; continue; }
            13 => { xmm4 = crate::simd_ops::vdpps(xmm2, xmm2, 0x7fu8); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; xmm5 = (xmm0) as u128; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 15; continue; } bb = 14; continue; }
            14 => { if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm4 as u32)).to_bits() as u128); bb = 16; continue; }
            16 => { xmm0 = crate::simd_ops::vdpps(xmm2, xmm5, 0x7fu8); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = crate::simd_ops::vbroadcastss((xmm1 as u32)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm2); bb = 17; continue; }
            17 => { xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x118u64)))).to_bits() as u128); xmm0 = crate::simd_ops::vdpps(xmm0, xmm0, 0x7fu8); xmm1 = crate::simd_ops::vsqrtps(xmm0); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x118u64)))).to_bits() as u128); xmm0 = 0u128; xmm4 = (xmm0 & !0xffff_ffffu128) | (xmm2 & 0xffff_ffffu128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) }).to_bits() as u128); bb = 19; continue; }
            18 => { xmm4 = (xmm3) as u128; bb = 19; continue; }
            19 => { xmm1 = ((crate::simd_ops::ru32(P1)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x128u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; xmm0 = (xmm4) as u128; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x41u64), (0x1u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { xmm0 = (xmm4) as u128; bb = 23; continue; }
            23 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b76fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xd8; const P2: u64 = 0xe8; const P3: u64 = 0x20; const P4: u64 = 0x140436250; const P5: u64 = 0xc8; const P6: u64 = 0x30; const P7: u64 = 0x134; const P8: u64 = 0x10; const P9: u64 = 0x120; const P10: u64 = 0x98; const P11: u64 = 0x143426564;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; r8 = ((rdi.wrapping_add(P3)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; xmm2 = ((crate::simd_ops::ru32(rdi.wrapping_add(P7))) as u32) as u128; rcx = ((rcx).wrapping_shl(5u32)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P8))) as u128; xmm0 = crate::simd_ops::vsubps(xmm0, crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))); xmm1 = crate::simd_ops::vdpps(xmm0, xmm0, 0x7fu8); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; r8 = ((rdi.wrapping_add(P10)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; r9 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = crate::simd_ops::vdpps(xmm0, xmm0, 0x7fu8); xmm0 = ((crate::simd_ops::ru32(P11)) as u32) as u128; rcx = 0; fl_l = 0; fl_r = 0; rax = 0x2; xmm2 = crate::simd_ops::vsqrtps(xmm1); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; rax = (if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = 0x2; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b770c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b54570; const P1: u64 = 0x18; const P2: u64 = 0x12c; const P3: u64 = 0x58; const P4: u64 = 0x130; const P5: u64 = 0x5c; const P6: u64 = 0x134; const P7: u64 = 0x60;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32) as u128;
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128);
    crate::simd_ops::wu32(rbx.wrapping_add(P3), ((xmm1 as u32)) as u32);
    xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32) as u128;
    xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128);
    crate::simd_ops::wu32(rbx.wrapping_add(P5), ((xmm0 as u32)) as u32);
    xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32) as u128;
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128);
    crate::simd_ops::wu32(rbx.wrapping_add(P7), ((xmm2 as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b778c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x30; const P5: u64 = 0x38; const P6: u64 = 0x40; const P7: u64 = 0x44; const P8: u64 = 0x48; const P9: u64 = 0x4c; const P10: u64 = 0x50; const P11: u64 = 0x51;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P2), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P5), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P10))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P10), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P11))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(0x51u64), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x58u64))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(0x58u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x5cu64))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(0x5cu64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x60u64))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(0x60u64), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b77af0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x5c; const P2: u64 = 0x58; const P3: u64 = 0x59;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32) as u128;
    rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8);
    rdx = (rdx & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rdx as u8)) as u8);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x0u8) as u8); }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b77b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r10 = r9; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; xmm1 = crate::simd_ops::vsubps(xmm0, crate::simd_ops::ru128(r8)); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x5cu64))) as u32) as u128; xmm2 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; r9 = (r9 & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), ((r9 as u8)) as u8); r8 = (r8 & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x59u64), ((r8 as u8)) as u8); rax = (((r8 as u8)) as u32) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rcx.wrapping_add(0x59u64), (0x0u8) as u8); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x51u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x123u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) as u32) as u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x120u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = 0u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x120u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; xmm2 = (crate::simd_ops::ru128(r10)) as u128; xmm1 = 0u128; xmm0 = (crate::simd_ops::ru128(rax)) as u128; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm4 = 0u128; bb = 13; continue; }
            12 => { xmm4 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x44u64))) as u32) as u128; bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x121u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 15; continue; }
            15 => { xmm3 = crate::simd_ops::vdpps(xmm2, xmm2, 0x7fu8); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; xmm5 = (xmm0) as u128; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 17; continue; } bb = 16; continue; }
            16 => { if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 18; continue; } bb = 17; continue; }
            17 => { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); bb = 18; continue; }
            18 => { xmm0 = crate::simd_ops::vdpps(xmm2, xmm5, 0x7fu8); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = crate::simd_ops::vbroadcastss((xmm1 as u32)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm2); bb = 19; continue; }
            19 => { xmm3 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x118u64)))).to_bits() as u128); xmm0 = crate::simd_ops::vdpps(xmm0, xmm0, 0x7fu8); xmm1 = crate::simd_ops::vsqrtps(xmm0); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x118u64)))).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm2 & 0xffff_ffffu128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) }).to_bits() as u128); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b77c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b54570; const P1: u64 = 0x18; const P2: u64 = 0x58; const P3: u64 = 0x128; const P4: u64 = 0x5c; const P5: u64 = 0x12c; const P6: u64 = 0x60;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu16(rbx.wrapping_add(P2), (0x0u16) as u16);
    xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32) as u128;
    xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128);
    crate::simd_ops::wu32(rbx.wrapping_add(P4), ((xmm3 as u32)) as u32);
    xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32) as u128;
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128);
    xmm4 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128);
    crate::simd_ops::wu32(rbx.wrapping_add(P6), ((xmm4 as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b78430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x30; const P5: u64 = 0x38; const P6: u64 = 0x40; const P7: u64 = 0x44; const P8: u64 = 0x48; const P9: u64 = 0x4c; const P10: u64 = 0x50; const P11: u64 = 0x51;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P0), rax); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P1), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P2), rax); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128; crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P5), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P8), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P10))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P10), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P11))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x51u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x58u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x58u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x59u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x59u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x5cu64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x5cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x60u64), ((rax as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b78670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1405e37f0; const P1: u64 = 0x14062d980; const P2: u64 = 0x143426fd0; const P3: u64 = 0x143426ff0; const P4: u64 = 0x14065d520;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rbx = rdx; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0xe8u64))) as u64; r8 = rdi.wrapping_add(0x28); rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; r8 = rdi.wrapping_add(0xa0); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; r9 = rsp.wrapping_add(0x50); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; r8 = rdi.wrapping_add(0x28); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); xmm6 = 0u128; rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; r9 = rsp.wrapping_add(0x50); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; r8 = rdi.wrapping_add(0xa0); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm6 as u32)) as u32); xmm7 = (xmm0) as u128; rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x118u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm6 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 10; continue; }
            7 => { xmm6 = ((crate::simd_ops::ru32(P2)) as u32) as u128; bb = 10; continue; }
            8 => { xmm6 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 10; continue; }
            9 => { xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm0 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (!(xmm1)) & (xmm0); bb = 10; continue; }
            10 => { xmm2 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x11cu64))) as u32) as u128; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; xmm0 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; bb = 12; continue; }
            11 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b78820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14065d070; const P1: u64 = 0x1432258c8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r9 = P1;
    rax = rcx;
    crate::simd_ops::wu64(rcx, r9);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b79230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x58; const P2: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbp = ((r9 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r15 = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(r14)) as u64; xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P2))) as u128; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = (((xmm1 >> 64) as u64)) as u64; rbx = (((xmm1 >> 64) as u64)) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x2; rsi = (rsi & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (((rsi as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { r14 = ((r14).wrapping_add(0x8u64)) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 7; continue; }
            7 => { rax = ((rbp as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b79300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x58; const P2: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P2))) as u128; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = (((xmm1 >> 64) as u64)) as u64; rbx = (((xmm1 >> 64) as u64)) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b79d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x141bcd1d0; const P2: u64 = 0x18; const P3: u64 = 0x141bccd80; const P4: u64 = 0x141bccc70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rdi = r8;
    rsi = rdx;
    rbx = rcx;
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x4u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r8 = rsp.wrapping_add(0x20);
    rdx = rdi;
    rcx = rsi;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rax, (0x0u64) as u64);
    rdx = rdi;
    rcx = rsi;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b79dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241c190; const P1: u64 = 0x14065ec00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = r8; rbp = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rax)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 1; continue; }
            1 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; bb = 6; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x41u64))) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 8; continue; }
            8 => { bb = 17; continue; }
            9 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rsp.wrapping_add(0x30); rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), r15); crate::simd_ops::wu64(rax, r15); rax = rax.wrapping_add(0x18); rcx = rax.wrapping_sub(0x10); fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 14; continue; }
            14 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); bb = 17; continue; }
            17 => { rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 18; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r8); r9 = rsp.wrapping_add(0x70); r8 = rsp.wrapping_add(0x88); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 20; continue; }
            20 => { rcx = rax.wrapping_add(0x10); bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rcx.wrapping_sub(0x8u64), r15); crate::simd_ops::wu64(rcx, r15); rcx = rcx.wrapping_add(0x18); rax = rcx.wrapping_sub(0x10); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 23; continue; }
            23 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b79fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140674a80; const P1: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = rdx; r14 = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; r15 = ((r15).wrapping_shl(5u32)) as u64; r15 = ((r15).wrapping_add(rdi)) as u64; r12 = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 8; continue; }
            8 => { rdi = ((rdi).wrapping_add(0x20u64)) as u64; fl_l = rdi; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru32(r14.wrapping_add(0x60u64))) as u32 as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(r14.wrapping_add(0x60u64), ((r12 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7a0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14065e660; const P1: u64 = 0x1403d9450; const P2: u64 = 0x140674a80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r15 = r8; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbx); rdi = rcx.wrapping_add(0x58); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_r = r12; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 18; continue; } bb = 1; continue; }
            1 => { r13 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { r14 = (crate::simd_ops::ru64(rdi)) as u64; r14 = ((r14).wrapping_add(r13)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), r12); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(r12.wrapping_mul(8)))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rax = rdx; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = r8.wrapping_add(0x1); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r14))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x2u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = (crate::simd_ops::lk_add_u32(r15.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 16; continue; }
            16 => { crate::simd_ops::wu8(rbp.wrapping_add(0x41u64), (0x1u8) as u8); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 17; continue; }
            17 => { r12 = ((r12).wrapping_add(1)) as u64; r13 = ((r13).wrapping_add(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = r12; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7a2d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = 0x3; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7a2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba3270; const P1: u64 = 0x58; const P2: u64 = 0x60; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); r14 = rdx; xmm6 = (xmm2) as u128; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rbp = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rbp = ((rbp).wrapping_shl(5u32)) as u64; rbp = ((rbp).wrapping_add(rsi)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r14; xmm2 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rsi = ((rsi).wrapping_add(0x20u64)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7a3b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b79dd0; const P1: u64 = 0x141b79d50; const P2: u64 = 0x141b79fc0; const P3: u64 = 0x141b7a0e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = ((r9 as u32)) as u32 as u64; rsi = r8; r14 = rdx; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x20u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = rbp; r8 = rsi; rdx = r14; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((rbx.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 10; continue; }
            4 => { rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; r8 = rsi; rdx = r14; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { r8 = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rsi; rdx = r14; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); bb = 9; continue; }
            9 => { rax = ((rbx as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7a4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba3170; const P1: u64 = 0x141b79fc0; const P2: u64 = 0x10; const P3: u64 = 0x28; const P4: u64 = 0xfffffffffffffff8; const P5: u64 = 0x18; const P6: u64 = 0xfffffffffffffff0; const P7: u64 = 0x14241c190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = rdx; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rdx = rsp.wrapping_add(0x20); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbx = rsi; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = ((rsi.wrapping_add(P2)) as u64) as u64; bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rax.wrapping_add(P4), r14); crate::simd_ops::wu64(rax, r14); rax = ((rax.wrapping_add(P5)) as u64) as u64; rdx = ((rax.wrapping_add(P6)) as u64) as u64; fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7a580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b7a0e0; const P1: u64 = 0x14241c190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rdx; r14 = rcx; crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rbp = ((rbx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); r8 = rsp.wrapping_add(0x20); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbx = rbp; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rcx.wrapping_mul(8)) as u64) as u64; rdi = ((rdi).wrapping_add(rbp)) as u64; fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x41u64), (0x1u8) as u8); bb = 8; continue; }
            8 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 9; continue; }
            9 => { fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = rbp.wrapping_add(0x10); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), r15); crate::simd_ops::wu64(rax, r15); rax = rax.wrapping_add(0x18); rdx = rax.wrapping_sub(0x10); fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7a6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b79dd0; const P1: u64 = 0x141b79fc0; const P2: u64 = 0x141bcd1d0; const P3: u64 = 0x141b79d50; const P4: u64 = 0x141b7a0e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = r8; rsi = rdx; rbx = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r14 = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); r9 = 0; fl_l = 0; fl_r = 0; r8 = rsp.wrapping_add(0x20); rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r14)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = rbp; r8 = rdi; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); r9 = 0x2; r8 = rsp.wrapping_add(0x20); rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rdi; rdx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7a7f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x50;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    r10 = rdx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rdx, rax); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = r10;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7afd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x38; const P5: u64 = 0x40; const P6: u64 = 0x41; const P7: u64 = 0x48; const P8: u64 = 0x50; const P9: u64 = 0x58; const P10: u64 = 0x14065f220;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm1 as u64)) as u64);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P7), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P8), rax);
    rax = ((rdx.wrapping_add(P9)) as u64) as u64;
    rcx = ((r8.wrapping_add(P9)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7c3b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141bcd1d0; const P1: u64 = 0x14241c190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = (((r9 as u8)) as u32) as u32 as u64; rbp = r8; rsi = rdx; rdi = rcx; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu8(rcx.wrapping_add(0x41u64), ((r15 as u8)) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = rsp.wrapping_add(0x20); rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = ((r15 as u32)) as u32 as u64; bb = 6; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); r9 = 0x3; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x20); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = 0x2; bb = 5; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x4u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); r9 = 0; fl_l = 0; fl_r = 0; r8 = rsp.wrapping_add(0x30); rdx = rbp; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = (((r14 as u8)) as u32) as u32 as u64; r8 = rbp; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rcx = rax.wrapping_add(0x10); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rcx.wrapping_sub(0x8u64), r15); crate::simd_ops::wu64(rcx, r15); rcx = rcx.wrapping_add(0x18); rdx = rcx.wrapping_sub(0x10); fl_l = rdx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 13; continue; }
            13 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7cd00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x38; const P5: u64 = 0x40; const P6: u64 = 0x41; const P7: u64 = 0x48; const P8: u64 = 0x50;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm1 as u64)) as u64);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P7), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P8), rax);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7cd90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143226978; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu16(rdx.wrapping_add(P8), ((rcx as u16)) as u16);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7cff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b7d150; const P1: u64 = 0x14241c190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rdx; rbp = rcx; xmm0 = 0u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm0) as u128); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r15); r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; rdx = rcx; rcx = rax.wrapping_sub(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), r15); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rax); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; r14 = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbp; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), r15); crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rbp; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7d0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140656690; const P1: u64 = 0xb8; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x14065ff50; const P5: u64 = 0x141b7d540;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r11 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(r11)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rdx = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(r11)) as u64; r9 = rdi; rdx = r10; rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7d150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1406565e0; const P1: u64 = 0x141ba33f0; const P2: u64 = 0x141b7d2a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = rdx; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r14 = r8; rsi = rcx; rdx = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rbx = rcx; rdi = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rax); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r14.wrapping_add(0x58u64), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; rbx = rcx; rdi = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rcx = rsi; rax = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rdx = rbp; crate::simd_ops::wu64(r14.wrapping_add(0x28u64), (0x0u64) as u64); rcx = r14; crate::simd_ops::wu64(r14.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(r14)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7d2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438ca230; const P1: u64 = 0x1438ca2e0; const P2: u64 = 0x14254768e; const P3: u64 = 0x141b7d150; const P4: u64 = 0x1438ca480; const P5: u64 = 0x1406565e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; r12 = rcx; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x30u64))) as u64; r15 = r8; rbp = rdx; rbx = rcx; rdi = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rcx = rsi; rax = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64; r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r15.wrapping_add(0x28u64), r13); rdx = rbp; crate::simd_ops::wu64(r15.wrapping_add(0x30u64), rax); rcx = r15; rax = (crate::simd_ops::ru64(r15)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r15)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r15; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x50u64))) as u64; r9 = P0; r8 = P1; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r13 as u32)) as u32); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = rax; rdx = rbp; rcx = r12; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x50u64))) as u64; r9 = P4; r8 = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r13 as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r12)) as u64; rdx = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 9; continue; }
            9 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 10; continue; }
            10 => { rdx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); rcx = r12; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; rbx = rcx; rdi = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rcx = rsi; rax = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rdx = rbp; crate::simd_ops::wu64(r14.wrapping_add(0x28u64), r13); rcx = r14; crate::simd_ops::wu64(r14.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(r14)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), r13); crate::simd_ops::wu64(r15.wrapping_add(0x18u64), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7d460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1406565e0; const P2: u64 = 0x30; const P3: u64 = 0x28; const P4: u64 = 0x38; const P5: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = r8; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbp = rdx; r9 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; rbx = rcx; rdi = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rcx = r14; rax = (crate::simd_ops::ru64(r14)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rdx = rbp; crate::simd_ops::wu64(rsi.wrapping_add(P3), (0x0u64) as u64); rcx = rsi; crate::simd_ops::wu64(rsi.wrapping_add(P2), rax); rax = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7d540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba2b40; const P1: u64 = 0x140660350; const P2: u64 = 0x14241c0a0; const P3: u64 = 0x1434268d0; const P4: u64 = 0x14241c190; const P5: u64 = 0x141b7d7c0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b7d540, funcinfo_rva: 0x3738ee4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xa8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rsi = (r9) as u64; r14 = (r8) as u64; r12 = (rdx) as u64; r15 = (rcx) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d574); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; r10 = (crate::simd_ops::ru64(rax)) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d583); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; r10 = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d590); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rax) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = ((rbx.wrapping_sub(0x1u64)) as u64) as u64; rcx = (((rcx) & (rdx))) as u64; rbx = ((rbx).wrapping_sub(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rbx }) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (rcx) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx).wrapping_add(rbx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 4; continue; }
            3 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (rcx) as u64; bb = 4; continue; }
            4 => { rax = ((rax).wrapping_sub(r8)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), (rax) as u64); rax = ((r8.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d5df); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x48u64), (r14) as u64); rcx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x68u64))) as u64; r8 = (rsi) as u64; rdx = (r15) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d5f9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((r9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (rbx) as u64); r8 = (rsi) as u64; rdx = (0x1u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d60b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); r9 = (r12) as u64; r8 = (r14) as u64; rdx = ((rbp.wrapping_add(0x40u64)) as u64) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d622); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; r13 = (crate::simd_ops::ru64(rax)) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((xmm0 as u32)) as u32); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rbx) as u64); rdx = ((rbx.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = ((rbx.wrapping_add(0x20u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d647); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rbx) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (0x7u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), (0x8u64) as u64); xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((xmm0 as u32)) as u32); rdx = ((rbx.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(0x78u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d68a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x8u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d6a8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rbx) as u64); rax = ((rbx.wrapping_add(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx, (rdi) as u64); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r13) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; r8 = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rdx = (r12) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d6f9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), (rax) as u64); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (0x1u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (rax) as u64); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d73a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (0x0u64) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d76a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b7d781); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7d7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba2b40; const P1: u64 = 0x140660160; const P2: u64 = 0x141bccbf0; const P3: u64 = 0x141b7d0d0; const P4: u64 = 0x140660540; const P5: u64 = 0x141b7db20;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r15 = rcx; rcx = (0x100000001b3u64) as u64; r10 = (((rdi as u8)) as u32) as u32 as u64; rax = (0xcbf29ce484222325u64) as u64; r12 = rdx; rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; r10 = (((r10) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xb9u64))) as u32) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; r10 = (((r10) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r13 = r8; r10 = (((r10) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xbau64))) as u32) as u32 as u64; r10 = (((r10) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r10 = (((r10) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xbbu64))) as u32) as u32 as u64; r10 = (((r10) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r10 = (((r10) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xbcu64))) as u32) as u32 as u64; r10 = (((r10) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r10 = (((r10) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xbdu64))) as u32) as u32 as u64; r10 = (((r10) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r10 = (((r10) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xbeu64))) as u32) as u32 as u64; r10 = (((r10) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r10 = (((r10) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xbfu64))) as u32) as u32 as u64; r10 = (((r10) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r10 = (((r10) ^ (rax))) as u64; r10 = (((r10) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x30u64))) as u64; rcx = (((rcx) & (r10))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(0x18u64)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 6; continue; }
            5 => { rax = rbp; bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rax; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rsi = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rcx = rbx.wrapping_sub(0x1); r8 = rax; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (((rcx) & (rdx))) as u64; rbx = ((rbx).wrapping_sub(rcx)) as u64; fl_l = rcx; fl_r = rcx; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rbx }) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = rcx; rax = ((rax).wrapping_sub(rbx)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdx).wrapping_add(rbx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 12; continue; }
            11 => { rdx = rbp; rax = rcx; bb = 12; continue; }
            12 => { rax = ((rax).wrapping_sub(r8)) as u64; rcx = rsi; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rax); rax = ((rdx.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = r14; rdx = r15; rbx = rax; rcx = rbx; crate::simd_ops::wu64(rax.wrapping_add(0x48u64), rdi); rax = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r8 = 0x8; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; rax = r10; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; r8 = ((r8).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rbx); fl_l = rax; fl_r = rax; rcx = ((r9.wrapping_mul(8)) as u64) as u64; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { r8 }) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = rdx; rax = ((rax).wrapping_sub(r8)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = ((rax).wrapping_sub(rcx)) as u64; rdx = ((r10.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rax); rax = ((rdx.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), r9); bb = 17; continue; }
            16 => { fl_l = rdx; fl_r = rcx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rdx); bb = 17; continue; }
            17 => { r8 = r14; rdx = 0x1; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rdx = rsp.wrapping_add(0xb8); rcx = r15; r8 = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rdx = rdi.wrapping_add(0x18); rax = ((rax).wrapping_sub(1)) as u64; rcx = rbp; fl_l = rax; fl_r = (0xfffffffffffffffdu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = r14; rcx = r15; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rdx = rdi.wrapping_add(0x20); rax = ((rax).wrapping_sub(1)) as u64; fl_l = rax; fl_r = (0xfffffffffffffffdu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { r8 = r14; rcx = r15; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rcx = rax; bb = 22; continue; }
            22 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r8 = rsp.wrapping_add(0xb8); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rsi); rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rdi); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rax, rcx); rcx = r13; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rsi); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 23; continue; }
            23 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r8 = r13; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rsi); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rbp.wrapping_mul(8)), rax); fl_l = ((rbp).wrapping_add(1)) as u64; rbp = ((fl_l as u64)) as u64; bb = 26; continue; }
            26 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rbp); rax = rsi; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), (0x1u64) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rcx); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7db20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba2b40; const P1: u64 = 0x140660160; const P2: u64 = 0x141b7d7c0; const P3: u64 = 0x141b7dce0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsi = rcx; rcx = r14; r12 = r9; r15 = r8; rbp = rdx; rax = (crate::simd_ops::ru64(r14)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; r13 = rax; r10 = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(r13)) as u64; rcx = r13; rbx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = rbx.wrapping_sub(0x1); r8 = rax; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (((rcx) & (rdx))) as u64; rbx = ((rbx).wrapping_sub(rcx)) as u64; fl_l = rcx; fl_r = rcx; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rbx }) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = rcx; rax = ((rax).wrapping_sub(rbx)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx).wrapping_add(rbx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 4; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rcx; bb = 4; continue; }
            4 => { rax = ((rax).wrapping_sub(r8)) as u64; rcx = r13; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rax); rax = ((r8.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(r13)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = rdi; rdx = rsi; rcx = rax; rbx = rax; r9 = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x48u64), r14); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbx); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdx = rsp.wrapping_add(0x98); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); r9 = rbp; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rcx); r8 = r14; rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (((crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as i32 as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = rax; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); r9 = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8); rdx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r8 = r15; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r12); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rax); rax = rbx; bb = 13; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); r8 = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); rcx = rsi; rdx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rax); bb = 11; continue; }
            11 => { rax = rbx; bb = 13; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7dce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba2b40; const P1: u64 = 0x140660160; const P2: u64 = 0x140660540;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r14 = rcx; rcx = (0x100000001b3u64) as u64; r9 = (((rdi as u8)) as u32) as u32 as u64; rax = (0xcbf29ce484222325u64) as u64; r12 = rdx; rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x99u64))) as u32) as u32 as u64; r15 = r8; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x9au64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x9bu64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x9cu64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x9du64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x9eu64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x9fu64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x30u64))) as u64; rcx = (((rcx) & (r9))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(0x18u64)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rax; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rsi = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = rbx.wrapping_sub(0x1); r8 = rax; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = (((rcx) & (rdx))) as u64; rbx = ((rbx).wrapping_sub(rcx)) as u64; fl_l = rcx; fl_r = rcx; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rbx }) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = rcx; rax = ((rax).wrapping_sub(rbx)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((rdx).wrapping_add(rbx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 13; continue; }
            12 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rcx; bb = 13; continue; }
            13 => { rax = ((rax).wrapping_sub(r8)) as u64; rcx = rsi; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); rax = ((r8.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = rbp; rdx = r14; rcx = rax; rbx = rax; r9 = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x48u64), rdi); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbx); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdx = rsp.wrapping_add(0x98); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); r9 = r12; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rcx); r8 = rdi; rcx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = rsp.wrapping_add(0x88); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); rdx = rsp.wrapping_add(0x30); rcx = r15; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7df10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0xa8; const P2: u64 = 0x8; const P3: u64 = 0x40; const P4: u64 = 0x10; const P5: u64 = 0x24; const P6: u64 = 0x20; const P7: u64 = 0x1403d79a0; const P8: u64 = 0x38; const P9: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rsi = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rdi = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P3))) as u32 as u64; fl_r = 0x4;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = ((rsi.wrapping_add(P4)) as u64) as u64; rbx = (crate::simd_ops::ru32(r8.wrapping_add(P5))) as u32 as u64; rcx = ((rcx.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rdx = ((r8.wrapping_add(P4)) as u64) as u64; xmm6 = ((crate::simd_ops::ru32(r8.wrapping_add(P6))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P5), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P6), ((xmm6 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P8), ((rax as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P9), ((rax as u32)) as u32); }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7e6d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x50; const P3: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if !((((fl_l as u32)) == ((fl_r as u32)))) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; } rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7f290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7f2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    loop { xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); rcx = ((xmm1 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(r8.wrapping_add(P0), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if !((((fl_l as u32)) != ((fl_r as u32)))) { break; } }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7f2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7f2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7f350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x60; const P2: u64 = 0x30; const P3: u64 = 0x8; const P4: u64 = 0x28; const P5: u64 = 0x140436250; const P6: u64 = 0x18; const P7: u64 = 0x141b880f0; const P8: u64 = 0x1438442e8; const P9: u64 = 0x143c8f63c; const P10: u64 = 0x580e0; const P11: u64 = 0x141b52d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = rdx; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r9 = rsp.wrapping_add(0x30); rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; r8 = ((rsi.wrapping_add(P2)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = P8; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (rax as u32 as i32 as i64) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r9 = ((r9.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 4; continue; }
            4 => { r8 = ((crate::simd_ops::ru16(P9)) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; xmm3 = ((crate::simd_ops::ru32(r9)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rcx = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32) as u128; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(r8.wrapping_mul(8)))) as u64; r8 = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b7fde0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_shl(4u32)) as u64;
    rcx = ((rcx).wrapping_add(rax)) as u64;
    fl_l = rax; fl_r = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = 0x1; return rax; }
    loop { rdx = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax.wrapping_add(P4), ((rdx as u32)) as u32); rdx = (tmp_xchg) as u32 as u64; rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = rcx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b805c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rax;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b80d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8fd28; const P1: u64 = 0x140436250; const P2: u64 = 0x141b880f0; const P3: u64 = 0x143426abc; const P4: u64 = 0x14211c5b0; const P5: u64 = 0x1434268d0; const P6: u64 = 0x1416c8880; const P7: u64 = 0x1433e1938; const P8: u64 = 0x1416c89a0; const P9: u64 = 0x1416c8b50; const P10: u64 = 0x1416e1530; const P11: u64 = 0x143932190; const P12: u64 = 0x141200340;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 53>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b80d00, funcinfo_rva: 0x3739270, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x1a8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x80u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x180u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); r12 = (rdx) as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; r13 = (crate::simd_ops::ru64(rcx.wrapping_add(r8.wrapping_mul(8)))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); r8 = ((r15.wrapping_add(0x20u64)) as u64) as u64; r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r12.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b80d6f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b80d7f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; r8 = ((r8).wrapping_add(0x8u64)) as u64; rcx = (crate::simd_ops::ru64(r13)) as u64; r9 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), (r9) as u64); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(r9.wrapping_mul(8)))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); xmm7 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm7 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(r9.wrapping_add(0x5cu64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b80dcb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 2; continue; }
            2 => { xmm6 = ((crate::simd_ops::ru32(P5)) as u32) as u128; bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x28u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { r14 = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; r14 = ((r14).wrapping_add(0x8u64)) as u64; rsi = (crate::simd_ops::ru64(r13)) as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b80e11); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm1 = crate::simd_ops::vshufps(xmm0, xmm0, 0xd8u8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (((xmm1 >> 64) as u32) as u32) as u32); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; xmm1 = crate::simd_ops::vshufps(xmm0, xmm0, 0xd8u8); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (((xmm1 >> 64) as u32) as u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x0u64) as u64); rax = (0xffffu32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x78u64), ((rax as u16)) as u16); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b80e78); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b80e9a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x30u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x34u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (r14) as u64); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b80eb3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P11)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b80edb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) as u32 as u64; fl_r = (0x8u32) as u32 as u64; rsi = (rsi & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b80eeb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { xmm2 = ((crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32) as u128; bb = 13; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); bb = 15; continue; }
            12 => { xmm2 = ((crate::simd_ops::ru32(r15.wrapping_add(0x14u64))) as u32) as u128; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x2cu64))) as u32 as u64; bb = 14; continue; }
            14 => { xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); rcx = ((xmm1 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x2cu64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 16; continue; }
            16 => { r11 = ((rsp.wrapping_add(0x180u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b81680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x140436250; const P3: u64 = 0xc; const P4: u64 = 0x140663880;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b81680, funcinfo_rva: 0x37392e4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (r8) as u64;
    r10 = (rdx) as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = ((r8).wrapping_add(0x10u64)) as u64;
    r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x141b816a1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rcx = ((rcx).wrapping_shl(4u32)) as u64;
    rax = ((rdx as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b816c5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; }
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64;
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu128(rax, (xmm0) as u128);
    crate::simd_ops::lk_add_u32(rbx.wrapping_add(P0), (1) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b816f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140436250; const P1: u64 = 0x14241c0a0; const P2: u64 = 0x143426fd0; const P3: u64 = 0x143426f84; const P4: u64 = 0x142646a90; const P5: u64 = 0x14241c190;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b816f0, funcinfo_rva: 0x3739328, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm9) as u128); r10 = (rdx) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), (rsi) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x60u64), (rsi) as u64); r8 = ((rdi.wrapping_add(0x10u64)) as u64) as u64; r9 = ((r11.wrapping_sub(0x58u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b81757); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (0x10u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b81767); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x28u64))) as u32 as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 3; continue; }
            3 => { xmm8 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm9 = ((crate::simd_ops::ru32(P3)) as u32) as u128; bb = 4; continue; }
            4 => { xmm4 = (xmm8) as u128; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x28u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { xmm5 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; bb = 7; continue; }
            7 => { xmm1 = crate::simd_ops::vsubps(xmm5, crate::simd_ops::ru128(rax)); xmm3 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8); xmm0 = 0u128; xmm4 = (xmm0 & !0xffff_ffffu128) | (xmm4 & 0xffff_ffffu128); xmm2 = (xmm0 & !0xffff_ffffu128) | (xmm3 & 0xffff_ffffu128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(xmm2 as u32) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x98u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x88u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x8cu64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); bb = 14; continue; }
            12 => { xmm6 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x94u64))) as u32) as u128; xmm7 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x90u64))) as u32) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b8183b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (((((rax as u32)) as i32) as f32).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x8u64)) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x2cu64))) as u32 as u64; bb = 13; continue; }
            13 => { xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); rcx = ((xmm1 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x2cu64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 14; continue; }
            14 => { rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 15; continue; }
            15 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 16; continue; }
            16 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b8188f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b82060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x88; const P2: u64 = 0x7c; const P3: u64 = 0x8; const P4: u64 = 0xac; const P5: u64 = 0xd8; const P6: u64 = 0xe8; const P7: u64 = 0x14062e8e0; const P8: u64 = 0x141b82110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = rdx; rdi = rcx; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(P1)))) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; r9 = rsp.wrapping_add(0x30); r8 = ((r8).wrapping_add(0x108u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; xmm1 = 0u128; xmm0 = (((rax as u32)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x1u8) as u8); bb = 6; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b82110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c9a628; const P1: u64 = 0x141b5bc40; const P2: u64 = 0x143c8d910; const P3: u64 = 0x14278ee90; const P4: u64 = 0x140663ea0; const P5: u64 = 0x14062ee30; const P6: u64 = 0x140265ba0; const P7: u64 = 0x143426d28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b82110, funcinfo_rva: 0x37393c8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; r9 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd8u64))) as u64; r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b82164); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(6u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x88u64)))) as u64; r14 = (((rax) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x60u64)))) as u64; rdx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xd0u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; r8 = ((rbp.wrapping_add(0x18u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; r9 = ((P3) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b821cb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (rax) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), (0x0u32) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(1)) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = ((rdi.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b8220a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x40u64)))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x48u64), (1) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x68u64), (0x0u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), (r12) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), (0x1u32) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rcx) as u64); rbx = (crate::simd_ops::ru64(r14)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x58u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x59u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x11u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x5au64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x12u64), ((rax as u8)) as u8); rdx = ((rbx.wrapping_add(0x60u64)) as u64) as u64; rcx = ((rdi.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b8226d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), (rax) as u64); xmm0 = ((crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(core::ptr::read_unaligned((P7) as *const u64))).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm0 as u64)))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x78u64), (0x0u32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x7cu64), (0x2u32) as u32); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b822d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0xc8; const P2: u64 = 0x88; const P3: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_shl(6u32)) as u64;
    fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P2)))) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b82300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1c; const P2: u64 = 0x18; const P3: u64 = 0x141b82110;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    r8 = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((r8 as u32)) as u32);
    return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b82ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x141b82ae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = rcx; rcx = rdx; rdi = rdx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b82b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x1438ca708; const P3: u64 = 0x1438ca748; const P4: u64 = 0x14254768e; const P5: u64 = 0xc0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    r9 = P2;
    r8 = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b82bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1406648a0; const P1: u64 = 0x140663fd0; const P2: u64 = 0x1406649f0; const P3: u64 = 0x140663ff0; const P4: u64 = 0x142645940; const P5: u64 = 0x1406643f0; const P6: u64 = 0x14241c190;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; r14 = rdx; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); r8 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x28u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = rbx; rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 6; continue; }
            6 => { rax = ((r8 as u32)) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rdi)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 7; continue; }
            7 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rbx; rcx = ((rcx).wrapping_sub(rdi)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); r9 = P1; rdx = rsp.wrapping_add(0x98); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            9 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rbx; rcx = ((rcx).wrapping_sub(rdi)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); r9 = P3; rdx = rsp.wrapping_add(0x28); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            11 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; rax = 0x3; let prod = (rax as u32 as u64).wrapping_mul(((r8 as u32)) as u32 as u64); rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rdx as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(30u32)) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul((0x80000001u32) as i32) as u32) as u32 as u64; fl_l = (((r8 as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; rax = 0x1; r8 = (if (((fl_l as u32)) == 0) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); r8 = rsp.wrapping_add(0x80); rdx = rsp.wrapping_add(0x38); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(r14, rax); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x10u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x28u64))) as u32) as u32 as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(r14.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x2cu64))) as u32 as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(r14.wrapping_add(0x2cu64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 15; continue; }
            14 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rax = (((rbx as u8)) as u32) as u32 as u64; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b82dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x20; const P2: u64 = 0x28; const P3: u64 = 0x1406645b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rbx = rdx; rdi = rcx; rax = r11.wrapping_sub(0x58); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r11.wrapping_sub(0x58); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); bb = 2; continue; }
            2 => { rax = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rcx = ((rdx.wrapping_add(r9)) as u64) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r9); r9 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x80); rcx = rsp.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x20); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rbx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b82ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    xmm1 = crate::simd_ops::vcmpps(xmm0, crate::simd_ops::ru128(rax), 0x0u8);
    rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64;
    rax = ((((rax as u32)) & (0x7u32))) as u32 as u64;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b82ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140664460;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rsi = rdx; rbx = rcx.wrapping_add(0x20); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; r9 = ((rcx.wrapping_add(r10)) as u64) as u64; rax = rcx; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { xmm2 = (crate::simd_ops::ru128(rdx)) as u128; bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; xmm1 = crate::simd_ops::vcmpps(xmm0, xmm2, 0x0u8); rdx = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64; rdx = ((((rdx as u32)) & (0x7u32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = r9; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r10) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm0) as u128); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x24u64), (0x3e8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x28u64), (0x1u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x2cu64), ((rax as u32)) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(0xffffffffffffffd0u64)) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), (0x1u32) as u32); crate::simd_ops::wu64(rax, rdi); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rax, rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b83010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x2c; const P2: u64 = 0x140664460;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = ((rdx).wrapping_add(rax)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    fl_l = rdx; fl_r = rax;
    if !((((fl_l as u64)) <= ((fl_r as u64)))) { rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b83050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140664460; const P1: u64 = 0x141b82ee0; const P2: u64 = 0x141b855c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rbp = rdx; r14 = rcx; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x28u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x60u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x40u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x2cu64))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(0x40u64))) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; fl_l = r8; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { xmm6 = (crate::simd_ops::ru128(rbx)) as u128; rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x28u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x2cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = r14.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x20u64)))) as u64; crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x24u64), (0x3e8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x28u64), (0x1u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x2cu64), ((r15 as u32)) as u32); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x28u64), (1) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x28u64))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x20u64)))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x20u64), (xmm6) as u128); bb = 10; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x20); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 11; continue; }
            11 => { rdx = rbp; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b831a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b831a0; const P1: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; rsi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbp = rcx; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), (0x0u64) as u64); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; r14 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            8 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = rsi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), (0x0u64) as u64); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); j_tgt = r8; bb = 0xffff_fffe; continue; }
            13 => { rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 14; continue; }
            14 => { return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x141b831a0u64) { 0x9 => { bb = 1; continue; } 0x39 => { bb = 2; continue; } 0x40 => { bb = 3; continue; } 0x54 => { bb = 4; continue; } 0x6d => { bb = 5; continue; } 0x70 => { bb = 6; continue; } 0x79 => { bb = 7; continue; } 0x85 => { bb = 8; continue; } 0x8a => { bb = 9; continue; } 0x9c => { bb = 10; continue; } 0xa5 => { bb = 11; continue; } 0xcc => { bb = 12; continue; } 0xd9 => { bb = 13; continue; } 0xee => { bb = 14; continue; } _ => { return crate::simd_ops::tail_stk(j_tgt, rax, rcx, rdx, r8, r9, rsp); } } }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b832a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14240ab10; const P1: u64 = 0x143228170; const P2: u64 = 0x1423fe030; const P3: u64 = 0x1424bff90; const P4: u64 = 0x14065da10; const P5: u64 = 0x1401df680; const P6: u64 = 0x143a34220; const P7: u64 = 0x1424b0060; const P8: u64 = 0x141b852f0; const P9: u64 = 0x143a33da0; const P10: u64 = 0x141b832a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 47>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x138u64)) as u64; rbp = r9; rsi = r8; r14 = rcx; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), (0x1u8) as u8); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r8)) as u64; rdx = rsp.wrapping_add(0x20); rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x20); rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rax.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 7; continue; }
            6 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 9; continue; }
            9 => { rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x138u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { r15 = 0; fl_l = 0; fl_r = 0; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rax)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; rdx = rax; rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; rdx = rax; rcx = rdi; rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); r13 = rax; bb = 19; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x20); rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; rdx = rax; rcx = rdi; rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { r13 = r15; bb = 18; continue; }
            17 => { r8 = P6; rdx = rdi; rcx = rax; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r13 = rax; bb = 18; continue; }
            18 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(r13.wrapping_add(0x30u64), rsi); r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; r9 = rbp; rcx = r13; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r12 = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x198u64), r15); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 42; continue; } bb = 21; continue; }
            21 => { bb = 22; continue; }
            22 => { r14 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = rsp.wrapping_add(0xa8); rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; rcx = rsp.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 25; continue; }
            25 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r12 = rax; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = r12.wrapping_sub(0x1); rcx = (((rcx) & (rdx))) as u64; r12 = ((r12).wrapping_sub(rcx)) as u64; fl_l = rcx; fl_r = rcx; r12 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { r12 }) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 48; continue; } bb = 27; continue; }
            27 => { rax = rcx; rax = ((rax).wrapping_sub(r12)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 48; continue; } bb = 28; continue; }
            28 => { rdx = ((rdx).wrapping_add(r12)) as u64; rax = ((rax).wrapping_sub(r8)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); rax = ((rdx.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = rax; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x198u64))) as u64; bb = 39; continue; }
            30 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = rsp.wrapping_add(0xa8); rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rax.wrapping_add(0x8); rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 33; continue; }
            32 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rsi = r15; bb = 38; continue; }
            37 => { r8 = P9; rdx = rsi; rcx = rax; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 38; continue; }
            38 => { rcx = rsp.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 39; continue; }
            39 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 40; continue; }
            40 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r14); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(r12.wrapping_mul(8)), rsi); r12 = ((r12).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x198u64), r12); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 41; continue; }
            41 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x190u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x180u64))) as u64; bb = 42; continue; }
            42 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 47; continue; } bb = 43; continue; }
            43 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x188u64))) as u64; bb = 44; continue; }
            44 => { r9 = rbp; r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rcx = r14; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(r15.wrapping_mul(8)), rax); fl_l = ((r15).wrapping_add(1)) as u64; r15 = ((fl_l as u64)) as u64; bb = 46; continue; }
            46 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 44; continue; } bb = 47; continue; }
            47 => { crate::simd_ops::wu64(r13.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(r13.wrapping_add(0x18u64), r15); rax = r13; bb = 11; continue; }
            48 => { fl_l = rcx; fl_r = r8; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rcx); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r8); bb = 10; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b83650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x141b82ae0; const P2: u64 = 0x5c;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b83680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432281b8; const P1: u64 = 0x18; const P2: u64 = 0x141b831a0; const P3: u64 = 0x20; const P4: u64 = 0x1401de830;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b836c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143427178; const P1: u64 = 0x10; const P2: u64 = 0x1432281b8; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x40; const P8: u64 = 0x143159a38; const P9: u64 = 0x44; const P10: u64 = 0x143159a40; const P11: u64 = 0x4c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8);
    rax = P2;
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P7), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(P8)) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P9), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(P10)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P11), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((xmm1 as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(0x54u64), ((xmm1 as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(0x58u64), ((xmm1 as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(0x5cu64), (0x1u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b83730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427178; const P1: u64 = 0x1432281b8; const P2: u64 = 0x143159a38; const P3: u64 = 0x143159a40; const P4: u64 = 0x140664010; const P5: u64 = 0x141b832a0; const P6: u64 = 0x141b82ae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rbx = rdx; rsi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = 0x10; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = r8; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rcx }) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = (0x60u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0xffffffffffffffa0u64)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rax = rcx.wrapping_add(0x60); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), ((rdi as u8)) as u8); rax = P1; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rcx, rax); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), rdi); crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rdi as u32)) as u32); rdi = rcx; xmm0 = ((crate::simd_ops::ru64(P2)) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x44u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x54u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x58u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x5cu64), (0x1u8) as u8); bb = 7; continue; }
            5 => { rax = rdx.wrapping_sub(0x60); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x60u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 7; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0xb8u64))) as u64; r9 = rbx; rdx = rsi; rcx = rdi; r8 = (crate::simd_ops::ru64(r8)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x5cu64), ((rax as u8)) as u8); rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b83850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x2c; const P3: u64 = 0x24; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x3e8u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x1u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b84090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x8; const P4: u64 = 0x1406663e0; const P5: u64 = 0x30; const P6: u64 = 0x40; const P7: u64 = 0x44; const P8: u64 = 0x54; const P9: u64 = 0x5c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); rax = ((rdx.wrapping_add(P2)) as u64) as u64; rcx = ((r8.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P5), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(P7))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P7), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P8), ((xmm1 as u64)) as u64); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P9))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P9), ((rax as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b84140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143427178; const P1: u64 = 0x10; const P2: u64 = 0x1432281b8; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x40; const P8: u64 = 0x143159a38; const P9: u64 = 0x44; const P10: u64 = 0x143159a40; const P11: u64 = 0x4c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    crate::simd_ops::wu8(rdx.wrapping_add(P1), (0x0u8) as u8);
    rax = P2;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    xmm0 = 0u128;
    crate::simd_ops::wu128(rdx.wrapping_add(P6), (xmm0) as u128);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(P8)) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P9), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(P10)) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P11), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(0x50u64), ((xmm1 as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x54u64), ((xmm1 as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x58u64), ((xmm1 as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(0x5cu64), (0x1u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b842e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x28; const P2: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_shl(4u32)) as u64;
    rcx = ((rcx).wrapping_add(rax)) as u64;
    fl_l = rax; fl_r = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    loop { rdx = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax.wrapping_add(P2), ((rdx as u32)) as u32); rdx = (tmp_xchg) as u32 as u64; rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = rcx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b84320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x140664460; const P4: u64 = 0x10; const P5: u64 = 0x24; const P6: u64 = 0x28; const P7: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x3e8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x1u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P7), ((rax as u32)) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P1), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(0xffffffffffffffd0u64)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rax.wrapping_add(P4), (xmm0) as u128); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b84690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x50; const P3: u64 = 0xc; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rcx = r8; rbx = r8; rdi = rdx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b84710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x8; const P2: u64 = 0x141b84690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rdi = rcx;
    rcx = rdx;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b85180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432283f0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu8(rdx.wrapping_add(P1), (0x0u8) as u8);
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b852f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = rdx; rdx = ((rdx.wrapping_mul(8)) as u64) as u64; rbp = r8; rdi = rcx; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { r11 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; r8 = 0x8; rax = r11; r10 = ((r8 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; r10 = ((r10).wrapping_sub(rax)) as u64; fl_l = rax; fl_r = rax; r10 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { r10 }) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = rax; rcx = ((rcx).wrapping_sub(r10)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r10 = ((r10).wrapping_add(r11)) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rcx); rax = ((r10.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), rax); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), r10); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), r14); rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; bb = 6; continue; }
            5 => { rcx = rax; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), rdx); fl_l = rcx; fl_r = rdx; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 6; continue; }
            6 => { r10 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rdx = ((rbp.wrapping_mul(8)) as u64) as u64; rax = r10; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; r8 = ((r8).wrapping_sub(rax)) as u64; fl_l = rax; fl_r = rax; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { r8 }) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rcx; rax = ((rax).wrapping_sub(r8)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_sub(rdx)) as u64; rcx = ((r10.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rax); rax = ((rcx.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbp); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { fl_l = rcx; fl_r = rdx; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rcx); bb = 11; continue; }
            11 => { crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rax = ((crate::exe::tls_pointer() as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); r12 = (0x7fffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r15); r15 = 0x6348; r15 = ((r15).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r15)) as u64; if (((fl_l as u64)) != 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            16 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 25; continue; }
            17 => { fl_l = rdx; fl_r = r12; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rbx = rsi; bb = 25; continue; }
            19 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 23; continue; } bb = 20; continue; }
            20 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 25; continue; }
            23 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rbx = rax; bb = 25; continue; }
            25 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rsi; bb = 27; continue; }
            27 => { r10 = (crate::simd_ops::ru64(r15)) as u64; rdx = ((rbp.wrapping_mul(8)) as u64) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; if (((fl_l as u64)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 32; continue; } bb = 29; continue; }
            29 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { r9 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 39; continue; }
            31 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rax); bb = 40; continue; }
            32 => { fl_l = rdx; fl_r = r12; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { r8 = rsi; bb = 40; continue; }
            34 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 38; continue; } bb = 35; continue; }
            35 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 36; continue; }
            36 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 40; continue; }
            38 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 39; continue; }
            39 => { r8 = rax; bb = 40; continue; }
            40 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 43; continue; }
            42 => { rsi = r8; bb = 43; continue; }
            43 => { crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b855c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x141b855c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_r = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rdx = rsi; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rax = 0x2; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b85650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x141b85650;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0x1; rsi = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_r = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rdx = rsi; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b856d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432283f0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8);
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b85760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a082f8; const P1: u64 = 0x142407c80; const P2: u64 = 0x142485aa0; const P3: u64 = 0x14340e320; const P4: u64 = 0x1432283c0; const P5: u64 = 0x142486460; const P6: u64 = 0x142645580; const P7: u64 = 0x142485780; const P8: u64 = 0x143228388; const P9: u64 = 0x140663fd0; const P10: u64 = 0x140666d00; const P11: u64 = 0x1406645b0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x10); rsp = ((rsp).wrapping_sub(0x110u64)) as u64; rsi = rdx; r13 = rcx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x28u64))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(r10)) as u64; fl_l = r10; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; r12 = (r12 & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); r15 = 0; fl_l = 0; fl_r = 0; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdx); rdi = (crate::simd_ops::ru64(P0)) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rcx = ((rcx).wrapping_add(0x40u64)) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = 0x1; rcx = ((r14 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r15); rax = P3; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), r15); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r15); rcx = 0x3; rax = (((r14 as u8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; let cx_src: u8 = (rcx as u8); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u8(rdx.wrapping_add(0x4u64), fl_l as u8, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = (rax & !0xffu64) | ((((fl_r as u8)) as u8 as u64)); } if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rax, (0xffffu32) as u32)) as u32 as u64; fl_r = fl_l; bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rsp.wrapping_add(0x2au64), ((r14 as u8)) as u8); r15 = r13.wrapping_add(0x28); rax = (crate::simd_ops::ru64(r15)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm0) as u128); crate::simd_ops::wu16(rbp.wrapping_sub(0x60u64), ((r14 as u16)) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rax); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x48u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r14); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rcx = ((rcx).wrapping_add(0x40u64)) as u64; rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((r14 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (0x0u64) as u64); rax = P4; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); let y_addr: u64 = rbp.wrapping_sub(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(0x18u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x50u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_add(0x38u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::lk_add_u32(rax, (1) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r14 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x1008u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rdx = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(P0)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; r15 = rcx.wrapping_add(0x28); rax = (crate::simd_ops::ru64(r15)) as u64; r14 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x50u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 19; continue; }
            19 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(r13.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; r15 = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 20; continue; }
            20 => { r13 = P8; bb = 21; continue; }
            21 => { rdi = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 30; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 23; continue; }
            23 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 29; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r13); rax = P9; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); rax = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rax); rax = rbp.wrapping_sub(0x30); crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (0x0u64) as u64); rdx = rbp.wrapping_sub(0x30); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); rax = rbp.wrapping_sub(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; r8 = ((r9.wrapping_add(rcx)) as u64) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r9); r9 = rbp.wrapping_sub(0x30); r8 = rdx; rdx = rbp.wrapping_add(0x68); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp.wrapping_sub(0x30); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x50); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { r14 = ((r14).wrapping_add(0x8u64)) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 31; continue; }
            31 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; rsp = ((rsp).wrapping_add(0x110u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b85b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x20; const P2: u64 = 0x24; const P3: u64 = 0x2c; const P4: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32) as u128; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(P2)))).to_bits() as u128); rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(P2)))).to_bits() as u128); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(P4)))).to_bits() as u128); bb = 3; continue; }
            3 => { xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); rcx = ((xmm1 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(r9.wrapping_add(P3), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 6; continue; }
            6 => { xmm0 = 0u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(r9.wrapping_add(P0), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b85b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8f63c; const P1: u64 = 0x580e0; const P2: u64 = 0x14214f5d0;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = ((rsp).wrapping_sub(0xc8u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x80u64), (0x21u64) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax); r8 = rdx; rdx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x60u64), ((rax as u16)) as u16); xmm0 = 0u128; crate::simd_ops::wu64(r11.wrapping_sub(0x60u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu32(r11.wrapping_sub(0x18u64), ((rax as u32)) as u32); xmm1 = 0u128; crate::simd_ops::wu128(r11.wrapping_sub(0x38u64), (xmm0) as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x1u8) as u8); crate::simd_ops::wu64(r11.wrapping_sub(0x70u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); crate::simd_ops::wu32(r11.wrapping_sub(0x40u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(r11.wrapping_sub(0x3cu64), ((xmm1 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(r8)) as u64; r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (((xmm0 >> 64) as u32) as u32) as u32); xmm0 = (crate::simd_ops::ru128(r9)) as u128; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; r9 = r11.wrapping_sub(0x80); rax = r11.wrapping_sub(0x68); crate::simd_ops::wu128(r11.wrapping_sub(0x28u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x3cu64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rdx = rsp.wrapping_add(0x3c); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (((xmm0 >> 64) as u32) as u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0xc8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b85c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140436250; const P1: u64 = 0x14241c0a0; const P2: u64 = 0x143ca77b8; const P3: u64 = 0x1434268d0; const P4: u64 = 0x14241c190; const P5: u64 = 0x143c8f63c; const P6: u64 = 0x14214f5d0; const P7: u64 = 0x141bd4a60; const P8: u64 = 0x143428120; const P9: u64 = 0x143427a70; const P10: u64 = 0x143428460; const P11: u64 = 0x143427730; const P12: u64 = 0x1434282a0; const P13: u64 = 0x143426820; const P14: u64 = 0x143426944; const P15: u64 = 0x143428240; const P16: u64 = 0x143428830;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 127>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b85c60, funcinfo_rva: 0x3739970, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x3f8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0x2f8u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x3b8u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x88u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x98u64), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xa8u64), (xmm11) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xb8u64), (xmm12) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xc8u64), (xmm13) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xd8u64), (xmm14) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xe8u64), (xmm15) as u128); r13 = (rdx) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x318u64), (r9) as u64); xmm6 = 0u128; xmm6 = crate::simd_ops::vinsertps(xmm6, ((crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u128), 0x10u8); xmm1 = 0u128; xmm1 = crate::simd_ops::vinsertps(xmm1, ((crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u128), 0x10u8); crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdi = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); rsi = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r8 = ((rbx.wrapping_add(0x30u64)) as u64) as u64; r9 = ((rbp.wrapping_add(0x200u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b85d34); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm6 = crate::simd_ops::vaddps(xmm6, crate::simd_ops::ru128(rbp.wrapping_add(0x200u64))); rdx = (0x10u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b85d4c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rsi as u32)) as u32); crate::simd_ops::wu128(rax, (xmm6) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rsi as u32)) as u32); bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x318u64))) as u64; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x10u64))) as u128; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x310u64), ((r8 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32) as u128; xmm12 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm12 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r10 = (r10 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r10 = (r10 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rbp.wrapping_add(0x300u64), ((r10 as u8)) as u8); rdx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r13)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; r11 = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (r11) as u64); rcx = (r14) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); rdx = (r14) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdx) as u64); rsi = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rsi) as u64); fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 20; continue; } bb = 9; continue; }
            9 => { rdi = (r14) as u64; xmm6 = ((crate::simd_ops::ru32(P3)) as u32) as u128; bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(rsi).wrapping_add(0x28u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(rsi).wrapping_add(0x10u64))) as u128; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x308u64), (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8); fl_l = (rcx) as u64 as u64; fl_r = (0x14u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 12; continue; }
            12 => { xmm1 = (((r14 as u32)) as u32) as u128; rax = (((r10 as u8)) as u32) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm4 = crate::simd_ops::vblendvps(xmm12, xmm0, xmm2); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 13; continue; }
            13 => { fl_l = ((xmm12 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 25; continue; } bb = 14; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(core::ptr::read_unaligned((rbx.wrapping_add(0x24u64)) as *const u32))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(core::ptr::read_unaligned((rbx.wrapping_add(0x24u64)) as *const u32))).to_bits() as u128); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(core::ptr::read_unaligned((rbx.wrapping_add(0x8u64)) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::ru32(rdi.wrapping_add(rsi).wrapping_add(0x2cu64))) as u32 as u64; bb = 15; continue; }
            15 => { xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); rcx = ((xmm1 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(rsi).wrapping_add(0x2cu64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 17; continue; }
            17 => { xmm6 = ((crate::simd_ops::ru32(P3)) as u32) as u128; bb = 18; continue; }
            18 => { rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdx) as u64); rdi = ((rdi).wrapping_add(0x30u64)) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rsi) as u64); fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x10u64))) as u128; r10 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x300u64))) as u32) as u32 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 19; continue; }
            19 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b85ecc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            22 => { r11 = ((rsp.wrapping_add(0x3b8u64)) as u64) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x18u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x28u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x38u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x48u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x58u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x68u64))) as u128; xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x78u64))) as u128; xmm13 = (crate::simd_ops::ru128(r11.wrapping_sub(0x88u64))) as u128; xmm14 = (crate::simd_ops::ru128(r11.wrapping_sub(0x98u64))) as u128; xmm15 = (crate::simd_ops::ru128(r11.wrapping_sub(0xa8u64))) as u128; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 24; continue; }
            24 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm12 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 25; continue; }
            25 => { rax = ((r14 as u32)) as u32 as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rdi.wrapping_add(rsi).wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); bb = 17; continue; }
            26 => { rcx = ((rcx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rsi))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 41; continue; } bb = 27; continue; }
            27 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r12 = ((r9 as u32)) as u32 as u64; r15 = ((r9 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = ((((rbx) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 28; continue; }
            28 => { xmm0 = crate::simd_ops::vaddps(xmm0, xmm1); crate::simd_ops::wu64(rbp.wrapping_sub(0x4cu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x44u64), (((xmm0 >> 64) as u32) as u32) as u32); bb = 29; continue; }
            29 => { xmm2 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), (0x21u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x28u64), (0x1u8) as u8); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (r9) as u64); rdx = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r13)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xa0u64), (r9) as u64); crate::simd_ops::wu16(rbp.wrapping_add(0x80u64), (0x0u16) as u16); crate::simd_ops::wu64(rbp.wrapping_add(0x88u64), (r9) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x90u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0xa8u64), ((xmm12 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xacu64), ((xmm12 as u32)) as u32); crate::simd_ops::wu128(rbp.wrapping_add(0xb0u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0xc0u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0xd0u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), ((xmm2 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x50u64), (((xmm2 >> 64) as u32) as u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rax = ((rbp.wrapping_add(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rbp.wrapping_add(0x20u64)) as u64) as u64; r8 = ((rbp.wrapping_sub(0x58u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x4cu64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b86045); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x80u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rdx = (rdx & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = ((r9 as u32)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); r12 = (((r12 as u32)).wrapping_add((rax as u32))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x310u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 32; continue; } bb = 30; continue; }
            30 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 31; continue; }
            31 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = (((rax as u8)) as u32) as u32 as u64; bb = 60; continue; }
            32 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 38; continue; } bb = 34; continue; }
            34 => { r15 = ((r15).wrapping_add(1)) as u64; r14 = ((r14).wrapping_add(0x10u64)) as u64; fl_l = (r15) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 29; continue; } bb = 35; continue; }
            35 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 36; continue; }
            36 => { xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | (((((r12 as u32)) as i32) as f32).to_bits() as u128); xmm0 = 0u128; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 39; continue; } bb = 37; continue; }
            37 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rbx) as i64) as f32).to_bits() as u128); xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 61; continue; }
            38 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rax = (((rax as u8)) as u32) as u32 as u64; bb = 60; continue; }
            39 => { rax = (rbx) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; rbx = ((((rbx as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rax) | (rbx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 61; continue; }
            40 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x308u64))) as u32) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; bb = 60; continue; }
            41 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x6cu64), (0x0u32) as u32); rax = (rcx) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rcx as u32)) as u32); r8 = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (r11) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b86122); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 59; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x70u64))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 59; continue; } bb = 43; continue; }
            43 => { r12 = ((r9 as u32)) as u32 as u64; r15 = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; r15 = ((r15).wrapping_shl(4u32)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r15 = ((((r15) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 44; continue; }
            44 => { xmm14 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; xmm15 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; rsi = ((r9.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 45; continue; }
            45 => { xmm13 = (crate::simd_ops::ru128(r14)) as u128; fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x6bu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 48; continue; } bb = 46; continue; }
            46 => { xmm7 = (crate::simd_ops::ru128(P8)) as u128; xmm4 = crate::simd_ops::vmulps(xmm14, crate::simd_ops::ru128(P9)); xmm0 = crate::simd_ops::vdpps(xmm4, xmm4, 0x7fu8); xmm2 = (crate::simd_ops::ru128(P10)) as u128; xmm3 = crate::simd_ops::vrsqrtps(xmm0); xmm0 = crate::simd_ops::vmulps(xmm3, xmm0); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm2 = crate::simd_ops::vsubps(xmm2, xmm1); xmm0 = crate::simd_ops::vmulps(xmm3, xmm7); xmm1 = crate::simd_ops::vmulps(xmm0, xmm2); xmm5 = crate::simd_ops::vmulps(xmm1, xmm4); xmm6 = (crate::simd_ops::ru128(P11)) as u128; xmm1 = crate::simd_ops::vpermilps(xmm6, 0xc9u8); xmm0 = crate::simd_ops::vpermilps(xmm5, 0xd2u8); xmm4 = crate::simd_ops::vmulps(xmm0, xmm1); xmm2 = crate::simd_ops::vpermilps(xmm6, 0xd2u8); xmm0 = crate::simd_ops::vpermilps(xmm5, 0xc9u8); xmm3 = crate::simd_ops::vmulps(xmm0, xmm2); xmm2 = 0u128; xmm2 = crate::simd_ops::vfmadd_ps(xmm7, xmm5, xmm2, false, false); xmm2 = crate::simd_ops::vfmadd_ps(xmm6, crate::simd_ops::ru128(P12), xmm2, false, false); xmm1 = 0u128; xmm0 = crate::simd_ops::vsubps(xmm3, xmm4); xmm2 = crate::simd_ops::vfmadd_ps(xmm1, xmm0, xmm2, false, false); xmm1 = crate::simd_ops::vaddps(xmm2, xmm15); xmm0 = crate::simd_ops::vmulps(xmm7, xmm14); xmm3 = crate::simd_ops::vsubps(xmm1, xmm0); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), (0x21u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x40u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (r9) as u64); rdx = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r13)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x100u64), (r9) as u64); crate::simd_ops::wu16(rbp.wrapping_add(0xe0u64), (0x0u16) as u16); crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (r9) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0xf0u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x108u64), ((xmm12 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x10cu64), ((xmm12 as u32)) as u32); crate::simd_ops::wu128(rbp.wrapping_add(0x110u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x120u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x130u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), ((xmm13 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (((xmm13 >> 64) as u32) as u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x34u64), ((xmm3 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x2cu64), (((xmm3 >> 64) as u32) as u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rax = ((rbp.wrapping_add(0xe0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rbp.wrapping_add(0x38u64)) as u64) as u64; r8 = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x34u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b862ba); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xe0u64))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 48; continue; } bb = 47; continue; }
            47 => { rbx = (((rsi as u8)) as u32) as u32 as u64; bb = 57; continue; }
            48 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 57; continue; } bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x6cu64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 53; continue; } bb = 50; continue; }
            50 => { xmm11 = (crate::simd_ops::ru128(P8)) as u128; xmm4 = crate::simd_ops::vmulps(xmm14, crate::simd_ops::ru128(P9)); xmm0 = crate::simd_ops::vdpps(xmm4, xmm4, 0x7fu8); xmm10 = (crate::simd_ops::ru128(P10)) as u128; xmm3 = crate::simd_ops::vrsqrtps(xmm0); xmm0 = crate::simd_ops::vmulps(xmm3, xmm0); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm2 = crate::simd_ops::vsubps(xmm10, xmm1); xmm0 = crate::simd_ops::vmulps(xmm11, xmm3); xmm1 = crate::simd_ops::vmulps(xmm0, xmm2); xmm3 = crate::simd_ops::vmulps(xmm4, xmm1); xmm7 = (crate::simd_ops::ru128(P11)) as u128; xmm9 = crate::simd_ops::vpermilps(xmm7, 0xc9u8); xmm0 = crate::simd_ops::vpermilps(xmm3, 0xd2u8); xmm6 = crate::simd_ops::vmulps(xmm9, xmm0); xmm8 = crate::simd_ops::vpermilps(xmm7, 0xd2u8); xmm0 = crate::simd_ops::vpermilps(xmm3, 0xc9u8); xmm4 = crate::simd_ops::vmulps(xmm8, xmm0); xmm5 = 0u128; xmm5 = crate::simd_ops::vfmadd_ps(xmm11, xmm3, xmm5, false, false); xmm1 = (((r9 as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x69u64))) as u32) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = ((crate::simd_ops::ru32(P13)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(P14)) as u32) as u128; xmm0 = crate::simd_ops::vblendvps(xmm1, xmm0, xmm2); xmm0 = crate::simd_ops::vbroadcastss((xmm0 as u32)); xmm5 = crate::simd_ops::vfmadd_ps(xmm0, xmm7, xmm5, false, false); xmm0 = crate::simd_ops::vsubps(xmm4, xmm6); xmm5 = crate::simd_ops::vfmadd_ps(xmm0, crate::simd_ops::ru128(P15), xmm5, false, false); xmm1 = crate::simd_ops::vaddps(xmm5, xmm15); xmm0 = crate::simd_ops::vmulps(xmm14, xmm11); xmm7 = crate::simd_ops::vsubps(xmm1, xmm0); xmm2 = crate::simd_ops::vshufps(xmm14, xmm14, 0xd2u8); xmm6 = crate::simd_ops::vmulps(xmm9, xmm2); xmm0 = crate::simd_ops::vshufps(xmm14, xmm14, 0xc9u8); xmm5 = crate::simd_ops::vmulps(xmm8, xmm0); xmm4 = crate::simd_ops::vsubps(xmm13, xmm7); xmm0 = crate::simd_ops::vdpps(xmm4, xmm4, 0x7fu8); xmm3 = crate::simd_ops::vrsqrtps(xmm0); xmm0 = crate::simd_ops::vmulps(xmm3, xmm0); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm2 = crate::simd_ops::vsubps(xmm10, xmm1); xmm0 = crate::simd_ops::vmulps(xmm11, xmm3); xmm1 = crate::simd_ops::vmulps(xmm0, xmm2); xmm2 = crate::simd_ops::vmulps(xmm4, xmm1); xmm0 = crate::simd_ops::vsubps(xmm5, xmm6); xmm3 = crate::simd_ops::vdpps(xmm0, xmm2, 0x7fu8); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm12 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 53; continue; } bb = 51; continue; }
            51 => { crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), (0x21u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x58u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), (r9) as u64); rdx = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r13)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x160u64), (r9) as u64); crate::simd_ops::wu16(rbp.wrapping_add(0x140u64), (0x0u16) as u16); crate::simd_ops::wu64(rbp.wrapping_add(0x148u64), (r9) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x150u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x168u64), ((xmm12 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x16cu64), ((xmm12 as u32)) as u32); crate::simd_ops::wu128(rbp.wrapping_add(0x170u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x180u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x190u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), ((xmm13 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), (((xmm13 >> 64) as u32) as u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x1cu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x14u64), (((xmm7 >> 64) as u32) as u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rbx as u8)) as u8); rax = ((rbp.wrapping_add(0x140u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rbp.wrapping_add(0x50u64)) as u64) as u64; r8 = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x1cu64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b86496); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x140u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 52; continue; }
            52 => { rbx = (((rsi as u8)) as u32) as u32 as u64; bb = 56; continue; }
            53 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x6au64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 56; continue; } bb = 54; continue; }
            54 => { xmm11 = (crate::simd_ops::ru128(P8)) as u128; xmm4 = crate::simd_ops::vmulps(xmm14, crate::simd_ops::ru128(P9)); xmm0 = crate::simd_ops::vdpps(xmm4, xmm4, 0x7fu8); xmm10 = (crate::simd_ops::ru128(P10)) as u128; xmm3 = crate::simd_ops::vrsqrtps(xmm0); xmm0 = crate::simd_ops::vmulps(xmm3, xmm0); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm2 = crate::simd_ops::vsubps(xmm10, xmm1); xmm0 = crate::simd_ops::vmulps(xmm11, xmm3); xmm1 = crate::simd_ops::vmulps(xmm0, xmm2); xmm3 = crate::simd_ops::vmulps(xmm4, xmm1); xmm7 = (crate::simd_ops::ru128(P11)) as u128; xmm9 = crate::simd_ops::vpermilps(xmm7, 0xc9u8); xmm0 = crate::simd_ops::vpermilps(xmm3, 0xd2u8); xmm6 = crate::simd_ops::vmulps(xmm9, xmm0); xmm8 = crate::simd_ops::vpermilps(xmm7, 0xd2u8); xmm0 = crate::simd_ops::vpermilps(xmm3, 0xc9u8); xmm4 = crate::simd_ops::vmulps(xmm8, xmm0); xmm5 = 0u128; xmm5 = crate::simd_ops::vfmadd_ps(xmm11, xmm3, xmm5, false, false); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm1 = (((r9 as u32)) as u32) as u128; rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x69u64))) as u32) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = ((crate::simd_ops::ru32(P13)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(P14)) as u32) as u128; xmm0 = crate::simd_ops::vblendvps(xmm1, xmm0, xmm2); xmm0 = crate::simd_ops::vbroadcastss((xmm0 as u32)); xmm5 = crate::simd_ops::vfmadd_ps(xmm0, xmm7, xmm5, false, false); xmm0 = crate::simd_ops::vsubps(xmm4, xmm6); xmm5 = crate::simd_ops::vfmadd_ps(xmm0, crate::simd_ops::ru128(P16), xmm5, false, false); xmm1 = crate::simd_ops::vaddps(xmm5, xmm15); xmm0 = crate::simd_ops::vmulps(xmm14, xmm11); xmm7 = crate::simd_ops::vsubps(xmm1, xmm0); xmm2 = crate::simd_ops::vshufps(xmm14, xmm14, 0xd2u8); xmm6 = crate::simd_ops::vmulps(xmm9, xmm2); xmm0 = crate::simd_ops::vshufps(xmm14, xmm14, 0xc9u8); xmm5 = crate::simd_ops::vmulps(xmm8, xmm0); xmm4 = crate::simd_ops::vsubps(xmm13, xmm7); xmm0 = crate::simd_ops::vdpps(xmm4, xmm4, 0x7fu8); xmm3 = crate::simd_ops::vrsqrtps(xmm0); xmm0 = crate::simd_ops::vmulps(xmm3, xmm0); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm2 = crate::simd_ops::vsubps(xmm10, xmm1); xmm0 = crate::simd_ops::vmulps(xmm11, xmm3); xmm1 = crate::simd_ops::vmulps(xmm0, xmm2); xmm2 = crate::simd_ops::vmulps(xmm4, xmm1); xmm0 = crate::simd_ops::vsubps(xmm5, xmm6); xmm3 = crate::simd_ops::vdpps(xmm0, xmm2, 0x7fu8); fl_l = ((xmm12 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 57; continue; } bb = 55; continue; }
            55 => { crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), (0x21u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x70u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), (r9) as u64); rdx = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r13)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1c0u64), (r9) as u64); crate::simd_ops::wu16(rbp.wrapping_add(0x1a0u64), ((r9 as u16)) as u16); crate::simd_ops::wu64(rbp.wrapping_add(0x1a8u64), (r9) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x1b0u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x1c8u64), ((xmm12 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1ccu64), ((xmm12 as u32)) as u32); crate::simd_ops::wu128(rbp.wrapping_add(0x1d0u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x1e0u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x1f0u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), ((xmm13 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), (((xmm13 >> 64) as u32) as u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x4u64), ((xmm7 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), (((xmm7 >> 64) as u32) as u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r9 as u8)) as u8); rax = ((rbp.wrapping_add(0x1a0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rbp.wrapping_add(0x68u64)) as u64) as u64; r8 = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x4u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b86669); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (((rbx as u8)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x1a0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rsi as u32) } else { (rbx as u32) }) as u32 as u64; bb = 56; continue; }
            56 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 57; continue; }
            57 => { r12 = ((r12).wrapping_add(1)) as u64; r14 = ((r14).wrapping_add(0x10u64)) as u64; fl_l = (r12) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 45; continue; } bb = 58; continue; }
            58 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 59; continue; }
            59 => { rax = (((rbx as u8)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x310u64))) as u32 as u64; bb = 60; continue; }
            60 => { xmm1 = (((r9 as u32)) as u32) as u128; xmm0 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm6 = (xmm0) as u128; xmm2 = crate::simd_ops::vblendvps(xmm0, xmm12, xmm2); bb = 61; continue; }
            61 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 66; continue; } bb = 62; continue; }
            62 => { fl_l = ((xmm12 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 68; continue; } bb = 63; continue; }
            63 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(core::ptr::read_unaligned((rbx.wrapping_add(0x24u64)) as *const u32))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(core::ptr::read_unaligned((rbx.wrapping_add(0x24u64)) as *const u32))).to_bits() as u128); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(core::ptr::read_unaligned((rbx.wrapping_add(0x8u64)) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::ru32(rdi.wrapping_add(rsi).wrapping_add(0x2cu64))) as u32 as u64; bb = 64; continue; }
            64 => { xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); rcx = ((xmm1 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(rsi).wrapping_add(0x2cu64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 64; continue; } bb = 65; continue; }
            65 => { bb = 69; continue; }
            66 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 67; continue; }
            67 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm12 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 63; continue; } bb = 68; continue; }
            68 => { rax = ((r9 as u32)) as u32 as u64; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rdi.wrapping_add(rsi).wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); bb = 69; continue; }
            69 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x318u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 18; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b86740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x140436250; const P3: u64 = 0xc; const P4: u64 = 0x140663880;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b86740, funcinfo_rva: 0x37399b0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rdi = (r9) as u64; rbx = (r8) as u64; r10 = (rdx) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = ((r8).wrapping_add(0x30u64)) as u64; r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b8676e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { xmm1 = (crate::simd_ops::ru128(rdi)) as u128; xmm6 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b8679c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::wu128(rax, (xmm6) as u128); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b86ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c9a628; const P1: u64 = 0x141b5c030; const P2: u64 = 0x1401de830;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = rdx; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(r8.wrapping_mul(8)))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r9); rbp = (crate::simd_ops::ru32(r15.wrapping_add(0x14u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx.wrapping_add(0x28); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbp as u32)) as u32); r9 = rsp.wrapping_add(0x30); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rbx = rax; bb = 11; continue; }
            8 => { rbx = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 11; continue; }
            11 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 12; continue; }
            12 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32) as u128; xmm4 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x28u64))) as u32 as u64; r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 14; continue; }
            14 => { rdx = ((r9 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(r8)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { xmm3 = (crate::simd_ops::ru128(rcx.wrapping_add(0x10u64))) as u128; rax = r8; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { xmm1 = crate::simd_ops::vsubps(xmm3, crate::simd_ops::ru128(rax)); xmm2 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 19; continue; }
            19 => { bb = 21; continue; }
            20 => { rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rcx.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 21; continue; }
            21 => { rcx = ((rcx).wrapping_add(0x30u64)) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 22; continue; }
            22 => { rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b87760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x140436250;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = rdx;
    r9 = r8;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((r8).wrapping_add(0x20u64)) as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b87790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140436250; const P1: u64 = 0x143ca77b8; const P2: u64 = 0x1434268d0; const P3: u64 = 0x143427080; const P4: u64 = 0x143426e70; const P5: u64 = 0x141bd4a60; const P6: u64 = 0x143427f30; const P7: u64 = 0x14183adac; const P8: u64 = 0x141b9c3b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r9 = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r8 = rbp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), r14); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rsi); rsi = rbx.wrapping_add(0x28); crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm10) as u128); xmm10 = ((crate::simd_ops::ru32(P4)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm6) as u128); xmm7 = 0u128; bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 5; continue; }
            5 => { rax = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0xe4u64), ((rcx as u32)) as u32); rax = ((rax).wrapping_shr(32u32)) as u64; r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0xe0u64), ((rax as u32)) as u32); rcx = r14; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), (0x0u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 17; continue; } bb = 7; continue; }
            7 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = crate::simd_ops::vsubps(xmm6, crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))); xmm2 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8); xmm0 = crate::simd_ops::vsqrtps(xmm2); xmm3 = crate::simd_ops::vdivps(xmm1, xmm0); xmm1 = (crate::simd_ops::ru128(P6)) as u128; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm2 = crate::simd_ops::vcmpps(xmm1, xmm2, 0x2u8); xmm4 = (xmm3) & (xmm2); xmm5 = crate::simd_ops::vdpps(xmm0, xmm4, 0x5fu8); xmm0 = crate::simd_ops::vdpps(xmm0, xmm0, 0x5fu8); xmm1 = crate::simd_ops::vdpps(xmm4, xmm4, 0x5fu8); xmm3 = crate::simd_ops::vsqrtps(xmm0); xmm2 = crate::simd_ops::vsqrtps(xmm1); xmm0 = crate::simd_ops::vmulps(xmm3, xmm2); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 10; continue; } bb = 8; continue; }
            8 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = (xmm7) as u128; bb = 11; continue; }
            10 => { xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 11; continue; }
            11 => { xmm1 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm1 = 0u128; xmm3 = (xmm1 & !0xffff_ffffu128) | (xmm9 & 0xffff_ffffu128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | (xmm8 & 0xffff_ffffu128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm3 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32) as u128; xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 18; continue; } bb = 12; continue; }
            12 => { rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x14u64))) as u32) as u128; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 14; continue; }
            14 => { xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x8u64)))).to_bits() as u128); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; bb = 15; continue; }
            15 => { xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); rcx = ((xmm1 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rsi.wrapping_add(0x4u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 16; continue; }
            16 => { bb = 18; continue; }
            17 => { rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rsi, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); bb = 18; continue; }
            18 => { rbx = ((rbx).wrapping_add(0x30u64)) as u64; rsi = ((rsi).wrapping_add(0x30u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 19; continue; }
            19 => { xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xb0u64))) as u128; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; bb = 20; continue; }
            20 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; bb = 21; continue; }
            21 => { rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b880f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x4c; const P2: u64 = 0x48; const P3: u64 = 0x20; const P4: u64 = 0x30; const P5: u64 = 0x143c8ff58; const P6: u64 = 0x180a8; const P7: u64 = 0x18090; const P8: u64 = 0x142489ee0; const P9: u64 = 0x180ac; const P10: u64 = 0x18098; const P11: u64 = 0x40; const P12: u64 = 0x1426467e8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } bb = 3; continue; }
            3 => { r8 = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = (((rbx as u16)) as u32) as u32 as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(rdi).wrapping_add(P6))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rdi).wrapping_add(P7))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(rdi).wrapping_add(P9))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rdi).wrapping_add(P10))) as u64; r11 = (((rbx as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = r11.wrapping_add(0xc03); rax = ((rax).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_shr(32u32)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdi).wrapping_add(P11))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64)))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (((rbx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rdx).wrapping_add(0x8u64))) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141b881ce, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b881e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x4; const P3: u64 = 0x141b654b0;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r10 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    r10 = ((r10).wrapping_shl(4u32)) as u64;
    r10 = ((r10).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64;
    fl_l = (crate::simd_ops::ru8(r10.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(r10)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; let y_addr: u64 = rcx; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b88250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x4; const P3: u64 = 0x8; const P4: u64 = 0x141b654b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; let y_addr: u64 = rcx; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rax = 0x3e8; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b88340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140436250; const P1: u64 = 0x14241c0a0; const P2: u64 = 0x143428720; const P3: u64 = 0x143426fd0; const P4: u64 = 0x1434268d0; const P5: u64 = 0x14241c190;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141b88340, funcinfo_rva: 0x3739bd0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm8) as u128); r10 = (rdx) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); r8 = ((rdi.wrapping_add(0x20u64)) as u64) as u64; r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b88397); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (0x10u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b883a7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 3; continue; }
            3 => { xmm8 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm5 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm7 = 0u128; xmm6 = ((crate::simd_ops::ru32(P4)) as u32) as u128; bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x28u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { xmm0 = (xmm5) as u128; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { xmm1 = crate::simd_ops::vpermilps(crate::simd_ops::ru128(rcx.wrapping_add(0x10u64)), 0x55u8); bb = 7; continue; }
            7 => { xmm0 = crate::simd_ops::vpermilps(crate::simd_ops::ru128(rdx), 0x55u8); xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); rdx = ((rdx.wrapping_add(0x10u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { xmm3 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32) as u128; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 23; continue; } bb = 12; continue; }
            12 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 23; continue; } bb = 13; continue; }
            13 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = 0u128; xmm4 = (xmm2 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm0 = 0u128; xmm3 = (xmm0 & !0xffff_ffffu128) | (xmm7 & 0xffff_ffffu128); xmm1 = (xmm0 & !0xffff_ffffu128) | (xmm6 & 0xffff_ffffu128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(xmm1 as u32) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1) & (xmm8); xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm7 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x1cu64)) as *const u32))).to_bits() as u128); xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x18u64)) as *const u32))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm4 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x8u64)) as *const u32))).to_bits() as u128); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x2cu64))) as u32 as u64; bb = 14; continue; }
            14 => { xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); rdx = ((xmm1 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rdx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rcx.wrapping_add(0x2cu64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rcx = ((rcx).wrapping_add(0x30u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 17; continue; }
            17 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141b8850d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            19 => { r11 = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; bb = 22; continue; }
            21 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 22; continue; }
            22 => { rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 23; continue; }
            23 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rcx.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); bb = 15; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b88c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d11c;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x18u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x28u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; r9 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp, (xmm6) as u128); xmm6 = 0u128; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; r11 = (crate::simd_ops::ru64(rdx.wrapping_add(r9.wrapping_mul(8)))) as u64; bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x28u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; fl_r = 0x2; xmm4 = (xmm6) as u128; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r11)) as u64; rdx = (crate::simd_ops::ru32(r11.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 5; continue; }
            5 => { xmm5 = (crate::simd_ops::ru128(rcx.wrapping_add(0x10u64))) as u128; bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x20u64))) as u128; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x30u64))) as u32) as u128; xmm1 = crate::simd_ops::vsubps(xmm0, xmm5); xmm3 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_add(0x40u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { bb = 10; continue; }
            9 => { xmm4 = ((crate::simd_ops::ru32(r10.wrapping_add(0x18u64))) as u32) as u128; bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(r10.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rcx.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); bb = 15; continue; }
            13 => { xmm2 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(r10.wrapping_add(0x8u64)))).to_bits() as u128); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x2cu64))) as u32 as u64; bb = 14; continue; }
            14 => { xmm0 = (((rax as u32)) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); rdx = ((xmm1 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rdx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rcx.wrapping_add(0x2cu64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rcx = ((rcx).wrapping_add(0x30u64)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 16; continue; }
            16 => { xmm6 = (crate::simd_ops::ru128(rsp)) as u128; bb = 17; continue; }
            17 => { rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141b89c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8fd28; const P1: u64 = 0x580e0; const P2: u64 = 0x10; const P3: u64 = 0x14211c310;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32) as u128;
    xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32) as u128;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(r9.wrapping_mul(8)))) as u64;
    r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    rdx = r8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm1 as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = ((r9).wrapping_add(0x8u64)) as u64; fl_r = fl_l;
    r9 = ((fl_l as u64)) as u64;
    xmm2 = (xmm3) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}
