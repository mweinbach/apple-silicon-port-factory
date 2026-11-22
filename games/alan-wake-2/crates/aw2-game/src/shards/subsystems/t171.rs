#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1411cff30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143881620; const P1: u64 = 0x14307aa58; const P2: u64 = 0x1413c3940; const P3: u64 = 0x14307a9d0; const P4: u64 = 0x14307a9e0; const P5: u64 = 0x1423fe030; const P6: u64 = 0x142481290; const P7: u64 = 0x1433d7560; const P8: u64 = 0x1; const P9: u64 = 0x1402c4e50; const P10: u64 = 0x4; const P11: u64 = 0x1433d7550; const P12: u64 = 0x1433dbf50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (crate::simd_ops::ru64(rdx)) as u64; rbx = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x1u32) as u32); rsi = rcx; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(P0)) as u64; r8 = rsp.wrapping_add(0x50); rdx = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx)) as u64; rax = P3; bb = 3; continue; }
            2 => { rdx = (crate::simd_ops::ru64(P0)) as u64; rcx = P4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { r8 = rsp.wrapping_add(0x60); rdx = rax; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; rax = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r9 = P7; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rdx = rsi; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rdi as u32)) as u32); r8 = ((rdi.wrapping_add(P8)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x60); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = ((rsi.wrapping_add(P10)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); r9 = P11; r8 = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x60); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rsi.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); r9 = P12; r8 = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d0280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142535070; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x14183acc2; const P7: u64 = 0x142428d70; const P8: u64 = 0xb8; const P9: u64 = 0x1433dba28; const P10: u64 = 0xbc; const P11: u64 = 0xc4;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0xd8; r8 = 0x10; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rdi; bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xd8; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P8), ((rdi as u8)) as u8); r10 = P9; crate::simd_ops::wu64(rbx, r10); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(P10), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P11), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xd0u64), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d0350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1; const P2: u64 = 0x142535360; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x8; const P6: u64 = 0xc; const P7: u64 = 0x1433dba28; const P8: u64 = 0xffffffffffffff3c; const P9: u64 = 0x14183acc2; const P10: u64 = 0x142428d70; const P11: u64 = 0xfffffffffffffff4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r9; rax = (((r9) as i64).wrapping_mul((0xd8u64) as i64) as u64) as u64; rdx = ((rax.wrapping_add(P0)) as u64) as u64; rcx = ((rdx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rcx }) as u64; rdx = 0x10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r14 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((r14 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rbp = ((rax.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rax, (0xd8u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(P5), ((rsi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P6), (0x8u32) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = rbp.wrapping_add(0xc4); r15 = P7; bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xd8; rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P8), r15); crate::simd_ops::wu8(rbx.wrapping_add(P11), ((r14 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), r14); crate::simd_ops::wu32(rbx, ((r14 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x4u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0xcu64), r14); rbx = rbx.wrapping_add(0xd8); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = rbp; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d1980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14247f340; const P1: u64 = 0x1411cf940; const P2: u64 = 0x14240ab10; const P3: u64 = 0x1425371a0; const P4: u64 = 0x142533200; const P5: u64 = 0x1430affa0; const P6: u64 = 0x1426e68d7; const P7: u64 = 0x1401f2430; const P8: u64 = 0x1401eb150; const P9: u64 = 0x14183acc2; const P10: u64 = 0x14309f824; const P11: u64 = 0x143881620; const P12: u64 = 0x1413aa3e0; const P13: u64 = 0x1401ee280; const P14: u64 = 0x142409b50; const P15: u64 = 0x1401f22d0; const P16: u64 = 0x14240a730; const P17: u64 = 0x142482340; const P18: u64 = 0x1413415d0; const P19: u64 = 0x1411d2014;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 121>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411d1980, funcinfo_rva: 0x360f484, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x3c8, conts: &[0x1411d1d0d] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x3b0u64)) as u64; rsi = (r9) as u64; rdi = (r8) as u64; rbx = (rcx) as u64; r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r12) as u64); rdx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d19b0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x1eu64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 40; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 40; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 59; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (r12) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((r12 as u8)) as u8); let y_addr: u64 = rsp.wrapping_add(0x90u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), (r12) as u64); rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), (r12) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((r12 as u32)) as u32); rbx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc4u64), ((rbx as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xcbu64), (0x10u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0xccu64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xd0u64), ((r12 as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x408u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (rax) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1aa7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d1aa7, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d1d0d => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d1980) } }; rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0xc8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1ab5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d1ab5, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d1d0d => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d1980) } }; rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1ae3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d1ae3, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d1d0d => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d1980) } }; rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 8; continue; }
            8 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1b66); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d1b66, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d1d0d => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d1980) } }; rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            13 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 15; continue; }
            14 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1b78); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d1b78, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d1d0d => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d1980) } }; rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), (r12) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1bb3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d1bb3, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d1d0d => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d1980) } }; rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 20; continue; }
            20 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 21; continue; }
            21 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1c36); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d1c36, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d1d0d => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d1980) } }; rax = c_rax; xmm0 = c_x0; bb = 28; continue; }
            25 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 27; continue; }
            26 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1c48); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d1c48, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d1d0d => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d1980) } }; rax = c_rax; xmm0 = c_x0; bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (r12) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1c78); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d1c78, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d1d0d => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d1980) } }; rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 30; continue; }
            30 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 31; continue; }
            31 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 32; continue; }
            32 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 39; continue; } bb = 35; continue; }
            35 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1cf5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d1cf5, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d1d0d => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d1980) } }; rax = c_rax; xmm0 = c_x0; bb = 39; continue; }
            36 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 38; continue; }
            37 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 38; continue; }
            38 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1d07); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d1d07, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d1d0d => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d1980) } }; rax = c_rax; xmm0 = c_x0; bb = 39; continue; }
            39 => { bb = 58; continue; }
            40 => { r9 = (rdi) as u64; r8 = ((P5) as u64) as u64; rdx = ((crate::simd_ops::ru8(P6)) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1d28); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x3f8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1d38); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1d42); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0x310u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1d54); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rdx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), (0x10u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x388u64), (0x6u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x390u64), (r12) as u64); rbx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x398u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x3a0u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x380u64), (r12) as u64); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x80u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm0) as u128); rax = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rax as u32)) as u32); r14 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(r14.wrapping_sub(0x30u64), (r12) as u64); rax = ((P10) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_sub(0x28u64), (rax) as u64); crate::simd_ops::wu64(r14.wrapping_sub(0x20u64), (r12) as u64); crate::simd_ops::wu64(r14.wrapping_sub(0x18u64), (r12) as u64); crate::simd_ops::wu32(r14.wrapping_sub(0x8u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_sub(0x10u64), (r12) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x390u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1e04); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_sub(0x18u64), (rax) as u64); bb = 43; continue; }
            43 => { crate::simd_ops::wu32(rsp.wrapping_add(0x3a8u64), ((r12 as u32)) as u32); r15 = (crate::simd_ops::ru64(P11)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x7cu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rax = ((r9.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1e55); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; bb = 45; continue; }
            45 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x70u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rax as u32)) as u32); r14 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x70u64)))) as u64; crate::simd_ops::wu64(r14.wrapping_sub(0x30u64), (rsi) as u64); crate::simd_ops::wu64(r14.wrapping_sub(0x28u64), (r15) as u64); crate::simd_ops::wu32(r14.wrapping_sub(0x20u64), (0x1u32) as u32); crate::simd_ops::wu32(r14.wrapping_sub(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu64(r14.wrapping_sub(0x18u64), (r12) as u64); crate::simd_ops::wu32(r14.wrapping_sub(0x8u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_sub(0x10u64), (r12) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x390u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1ed3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_sub(0x18u64), (rax) as u64); bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x3a8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { crate::simd_ops::wu64(rsi, (r12) as u64); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((r12 as u32)) as u32); bb = 50; continue; }
            50 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x398u64), ((rcx as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rdx as u32)) as u32); rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1f17); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), (0x0u8) as u8); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 53; continue; } bb = 51; continue; }
            51 => { crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), ((rdi as u8)) as u8); bb = 55; continue; }
            52 => { bb = 53; continue; }
            53 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 54; continue; }
            54 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rbx) as u64); r9 = (rbx) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1f63); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 55; continue; }
            55 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x400u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 57; continue; } bb = 56; continue; }
            56 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1fa2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 57; continue; }
            57 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1fce); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d1fdb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 58; continue; }
            58 => { r11 = ((rsp.wrapping_add(0x3b0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            59 => { r8 = (rdi) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2006); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d200f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P19, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d2020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14247f340; const P1: u64 = 0x1411cfa70; const P2: u64 = 0x14240ab10; const P3: u64 = 0x1425371a0; const P4: u64 = 0x142533200; const P5: u64 = 0x14183acc2; const P6: u64 = 0x14309f824; const P7: u64 = 0x143881620; const P8: u64 = 0x1413aa3e0; const P9: u64 = 0x1401ee280; const P10: u64 = 0x142409b50; const P11: u64 = 0x1401f22d0; const P12: u64 = 0x14240a730; const P13: u64 = 0x142482340; const P14: u64 = 0x1413415d0; const P15: u64 = 0x1411d267b;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 121>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411d2020, funcinfo_rva: 0x360f4d4, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x3c8, conts: &[0x1411d23ad] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x3b0u64)) as u64; rsi = (r9) as u64; rdi = (r8) as u64; rbx = (rcx) as u64; r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r12) as u64); rdx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2050); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x1eu64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 40; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 40; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xeu64))) as u16 as u64; fl_r = (0x3u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 58; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (r12) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((r12 as u8)) as u8); let y_addr: u64 = rsp.wrapping_add(0x90u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), (r12) as u64); rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), (r12) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((r12 as u32)) as u32); rbx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc4u64), ((rbx as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xcbu64), (0x10u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0xccu64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0xd0u64), ((r12 as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x408u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (rax) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2147); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d2147, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d23ad => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d2020) } }; rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0xc8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2155); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d2155, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d23ad => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d2020) } }; rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2183); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d2183, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d23ad => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d2020) } }; rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 8; continue; }
            8 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2206); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d2206, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d23ad => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d2020) } }; rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            13 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 15; continue; }
            14 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2218); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d2218, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d23ad => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d2020) } }; rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), (r12) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2253); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d2253, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d23ad => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d2020) } }; rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 20; continue; }
            20 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 21; continue; }
            21 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d22d6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d22d6, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d23ad => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d2020) } }; rax = c_rax; xmm0 = c_x0; bb = 28; continue; }
            25 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 27; continue; }
            26 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d22e8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d22e8, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d23ad => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d2020) } }; rax = c_rax; xmm0 = c_x0; bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (r12) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x108u64) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2318); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d2318, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d23ad => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d2020) } }; rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x108u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 30; continue; }
            30 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 31; continue; }
            31 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 32; continue; }
            32 => { rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(0x108u64)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 39; continue; } bb = 35; continue; }
            35 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2395); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d2395, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d23ad => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d2020) } }; rax = c_rax; xmm0 = c_x0; bb = 39; continue; }
            36 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 38; continue; }
            37 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 38; continue; }
            38 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d23a7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411d23a7, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x1411d23ad => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 39; continue; } _ => l.unknown(0x1411d2020) } }; rax = c_rax; xmm0 = c_x0; bb = 39; continue; }
            39 => { bb = 57; continue; }
            40 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0x310u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d23bf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rdx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), (0x10u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x388u64), (0x6u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x390u64), (r12) as u64); rbx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x398u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x3a0u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x380u64), (r12) as u64); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x80u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm0) as u128); rax = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rax as u32)) as u32); r14 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(r14.wrapping_sub(0x30u64), (r12) as u64); rax = ((P6) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_sub(0x28u64), (rax) as u64); crate::simd_ops::wu64(r14.wrapping_sub(0x20u64), (r12) as u64); crate::simd_ops::wu64(r14.wrapping_sub(0x18u64), (r12) as u64); crate::simd_ops::wu32(r14.wrapping_sub(0x8u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_sub(0x10u64), (r12) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x390u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d246f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_sub(0x18u64), (rax) as u64); bb = 43; continue; }
            43 => { crate::simd_ops::wu32(rsp.wrapping_add(0x3a8u64), ((r12 as u32)) as u32); r15 = (crate::simd_ops::ru64(P7)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x7cu64))) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rax = ((r9.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d24c0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; bb = 45; continue; }
            45 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x70u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rax as u32)) as u32); r14 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x70u64)))) as u64; crate::simd_ops::wu64(r14.wrapping_sub(0x30u64), (rsi) as u64); crate::simd_ops::wu64(r14.wrapping_sub(0x28u64), (r15) as u64); crate::simd_ops::wu32(r14.wrapping_sub(0x20u64), (0x1u32) as u32); crate::simd_ops::wu32(r14.wrapping_sub(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu64(r14.wrapping_sub(0x18u64), (r12) as u64); crate::simd_ops::wu32(r14.wrapping_sub(0x8u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_sub(0x10u64), (r12) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x390u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d253e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_sub(0x18u64), (rax) as u64); bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x3a8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { crate::simd_ops::wu64(rsi, (r12) as u64); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((r12 as u32)) as u32); bb = 50; continue; }
            50 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x398u64), ((rcx as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rdx as u32)) as u32); rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2582); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x10000010u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), (0x0u8) as u8); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 52; continue; } bb = 51; continue; }
            51 => { crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), ((rdi as u8)) as u8); bb = 54; continue; }
            52 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 52; continue; } bb = 53; continue; }
            53 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rbx) as u64); r9 = (rbx) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d25ca); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 54; continue; }
            54 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x400u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = (((r8) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 56; continue; } bb = 55; continue; }
            55 => { rax = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2609); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 56; continue; }
            56 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::lk_and_u32(rcx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x10u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2635); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2642); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 57; continue; }
            57 => { r11 = ((rsp.wrapping_add(0x3b0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            58 => { r8 = (rdi) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d266d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2676); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P15, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d2ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14209a1e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rsi = rcx;
    rdi = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rsi.wrapping_add(P0), (0x0u64) as u64);
    crate::simd_ops::wu32(rsi, ((rdi as u32)) as u32);
    rax = ((rsi.wrapping_add(P0)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d2b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14209a1e0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d2b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14209a1e0; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x108; const P4: u64 = 0x4; const P5: u64 = 0x100; const P6: u64 = 0xe; const P7: u64 = 0x20; const P8: u64 = 0x18; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rcx = ((r9.wrapping_add(P2)) as u64) as u64; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(P7), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P8), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d2c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1411d2b40;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rdx; rdi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
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
            15 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; rdx = rbx; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r14); rdx = rdx.wrapping_add(0x10); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d2d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x118; const P1: u64 = 0x142442de0; const P2: u64 = 0x8; const P3: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d2de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340ed48; const P1: u64 = 0x8; const P2: u64 = 0x1433dc1e0; const P3: u64 = 0x1433dc1b8; const P4: u64 = 0x142498f00;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = P2;
    crate::simd_ops::wu64(rcx, rax);
    r9 = ((r8 as u32)) as u32 as u64;
    r8 = ((rdx as u32)) as u32 as u64;
    rdx = P3;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d2e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    fl_l = r8; fl_r = r9;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = rcx; return rax; }
    loop { rax = ((((rax as u32)) | (crate::simd_ops::ru32(r8)))) as u32 as u64; r8 = ((r8).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); fl_l = r8; fl_r = r9; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d2ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1411d3100; const P3: u64 = 0x14183acc2; const P4: u64 = 0x1411d3270;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rcx.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r8 = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d2fb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411d2e20; const P1: u64 = 0x142619be0; const P2: u64 = 0x141834e48; const P3: u64 = 0x1401e9170; const P4: u64 = 0x1437fd9e8; const P5: u64 = 0x14183acce; const P6: u64 = 0x1411d301d;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411d2fb0, funcinfo_rva: 0x367e3d4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rcx) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rcx = (0x62e0u32) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rbx))) as u32 as u64; rsi = (0x62e8u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rbx), ((rax as u32)) as u32); rcx = ((rbx.wrapping_add(rsi)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2fe9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((P1) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d2ff5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(rsi)) as u64) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d3002); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rcx) as u64); rdx = ((P4) as u64) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d3018); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d3270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14206b020; const P1: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411d3270, funcinfo_rva: 0x359dc18, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (r8) as u64; rbx = (rdx) as u64; bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d329b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d32a3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d32d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142075af0; const P1: u64 = 0x140fff1e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = r8; rbp = rdx; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbx); rcx = ((rcx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rcx, rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rbx); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x88u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x98u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), rbx); rsi = rdi.wrapping_add(0xc0); xmm0 = 0u128; crate::simd_ops::wu128(rsi, (xmm0) as u128); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xd8u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xe0u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xe8u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x2u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x1u32) as u32); r9 = ((rbx as u32)) as u32 as u64; rax = rsp.wrapping_add(0x40); bb = 1; continue; }
            1 => { r9 = ((((r9 as u32)) | (crate::simd_ops::ru32(rax)))) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; r8 = rsp.wrapping_add(0x4c); fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xe0u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0xd8u64), rbx); rdx = r14; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = r9.wrapping_add(0x3f); rdx = (((rdx) & (0xffffffffffffffc0u64))) as u64; rdx = ((rdx).wrapping_shr(6u32)) as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { r10 = (0xffffffffffffffffu64) as u64; bb = 4; continue; }
            4 => { rax = rdx.wrapping_sub(0x1); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = r9.wrapping_sub(0x1); rax = (!(rax)) as u64; fl_l = ((((rax as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rcx = ((r10).wrapping_shr((((rax) as u32) & 63))) as u64; bb = 7; continue; }
            6 => { rcx = r10; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(rbx.wrapping_mul(8)))) as u64; rax = (((rax) & (rcx))) as u64; rax = (!(rax)) as u64; crate::simd_ops::wu64(r8.wrapping_add(rbx.wrapping_mul(8)), rax); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 8; continue; }
            8 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d3470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140fff1e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdi = rcx.wrapping_add(0xc0); crate::simd_ops::wu64(rcx, rbx); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbx); rsi = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0xc8u64), rbx); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0xd8u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0xe0u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0xe8u64), rbx); let y_addr: u64 = rdx.wrapping_add(0x10u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x10u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdx.wrapping_add(0x30u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rcx.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x50u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rcx.wrapping_add(0x50u64), (xmm0) as u128); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0xd8u64), rdx); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rcx); rcx = rdi; crate::simd_ops::wu64(rsi.wrapping_add(0xe0u64), r9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = r9.wrapping_add(0x3f); rdx = ((rdx).wrapping_shr(6u32)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r10 = rbx.wrapping_sub(0x1); bb = 2; continue; }
            2 => { rax = rdx.wrapping_sub(0x1); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = r9.wrapping_sub(0x1); rax = (!(rax)) as u64; fl_l = ((((rax as u32)) & (0x3fu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; rcx = ((r10).wrapping_shr((((rax) as u32) & 63))) as u64; bb = 5; continue; }
            4 => { rcx = r10; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(rbx.wrapping_mul(8)))) as u64; rax = (((rax) & (rcx))) as u64; rax = (!(rax)) as u64; crate::simd_ops::wu64(r8.wrapping_add(rbx.wrapping_mul(8)), rax); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d35d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x1411d35d0; const P2: u64 = 0x14243b450; const P3: u64 = 0x1411d3470; const P4: u64 = 0x1411d32d0; const P5: u64 = 0x1411d3b20; const P6: u64 = 0x142645598;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r14 = rdx; rdi = rcx; rbx = rcx.wrapping_add(0x30); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = rdi.wrapping_add(0x38); rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x28u64), (1) as u64); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; bb = 8; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; bb = 8; continue; }
            7 => { rcx = rsi; bb = 8; continue; }
            8 => { r8 = rdi.wrapping_add(0x40); rax = (crate::simd_ops::ru64(r8)) as u64; rdx = rax.wrapping_sub(0xb0); fl_l = rax; fl_r = rax; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsi } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx).wrapping_add(0xb0u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 11; continue; }
            10 => { rdx = rsi; bb = 11; continue; }
            11 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((rcx).wrapping_add(0xb0u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 14; continue; }
            13 => { rcx = rsi; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rdx, rcx); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x48u64), (1) as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 16; continue; }
            15 => { r8 = rdi.wrapping_add(0x40); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(r8)) as u64; rbp = rax.wrapping_sub(0xb0); fl_l = rax; fl_r = rax; rbp = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsi } else { rbp }) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64; rax = rcx.wrapping_add(0x3f); rax = (((rax) & (0xffffffffffffffc0u64))) as u64; r11 = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; rax = ((rax).wrapping_shr(6u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r11); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x48); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = r8; rax = ((rax).wrapping_shr(6u32)) as u64; rdx = ((r11.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = r8; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rcx = 0x1; rcx = ((rcx).wrapping_shl((((rax) as u32) & 63))) as u64; rcx = (!(rcx)) as u64; crate::simd_ops::lk_and_u64(rdx, (rcx) as u64); r11 = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64; r9 = r11.wrapping_add(0x3f); r9 = (((r9) & (0xffffffffffffffc0u64))) as u64; r9 = ((r9).wrapping_shr(6u32)) as u64; r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 17; continue; }
            17 => { rcx = rsi; rdx = r9.wrapping_sub(0x1); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rax = r10; bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 20; continue; }
            20 => { rcx = ((rcx).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 21; continue; }
            21 => { rax = r11.wrapping_sub(0x1); rax = (!(rax)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rcx = (0xffffffffffffffffu64) as u64; rax = ((rcx).wrapping_shr((((rax) as u32) & 63))) as u64; fl_l = (crate::simd_ops::ru64(r10.wrapping_add(r9.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = rbp.wrapping_add(0xb0); rcx = rax.wrapping_sub(0xb0); fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsi } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rsi = rcx.wrapping_add(0xb0); bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x48u64), (1) as u32)) as u32 as u64; bb = 26; continue; }
            26 => { crate::simd_ops::wu64(r14, rbp); r8 = (((r8) as i64).wrapping_mul((crate::simd_ops::ru64(rdi)) as i64) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r8); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = r14; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d3820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x1411d3ac0; const P2: u64 = 0x142645598; const P3: u64 = 0x1411d3820; const P4: u64 = 0x142075a20; const P5: u64 = 0x1401de830; const P6: u64 = 0x14243b3e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = rcx; rdx = 0; fl_l = 0; fl_r = 0; rax = r8; let d = (crate::simd_ops::ru64(rcx)) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } r14 = rax; rcx = ((rcx).wrapping_add(0x30u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0xc0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r10 = rsi.wrapping_add(0x38); rdx = r10.wrapping_sub(0xb0); fl_l = r10; fl_r = r10; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx).wrapping_add(0xb0u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 4; continue; }
            3 => { rdx = r8; bb = 4; continue; }
            4 => { r9 = rdi.wrapping_add(0xb0); rcx = r9; fl_l = rdi; fl_r = rdi; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rcx }) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rdx, rcx); fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 6; continue; }
            5 => { r9 = rdi.wrapping_add(0xb0); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; rax = r14; rax = ((rax).wrapping_shr(6u32)) as u64; r14 = ((((r14 as u32)) & (0x3fu32))) as u32 as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = 0x1; rax = ((rax).wrapping_shl((((r14) as u32) & 63))) as u64; rax = (!(rax)) as u64; rax = (((rax) & (crate::simd_ops::ru64(rdx)))) as u64; fl_l = (((rax) >> (((r14) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((r14) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx, rax); r11 = (crate::simd_ops::ru64(rdi.wrapping_add(0xd0u64))) as u64; r10 = r11.wrapping_add(0x3f); r10 = (((r10) & (0xffffffffffffffc0u64))) as u64; r10 = ((r10).wrapping_shr(6u32)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { r14 = (0xffffffffffffffffu64) as u64; rcx = r14; r9 = r10.wrapping_sub(0x1); rax = r8; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = r9; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = r9; fl_l = (((rcx) & (0xfffffffffffffff8u64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; let (y_lo, y_hi): (u128, u128) = (crate::simd_ops::vpcmpeqd(xmm1, xmm1), crate::simd_ops::vpcmpeqd(ymm1_hi, ymm1_hi)); xmm1 = y_lo; ymm1_hi = y_hi; xmm2 = xmm1; ymm2_hi = ymm1_hi; bb = 10; continue; }
            10 => { let y_addr: u64 = rdx.wrapping_add(rax.wrapping_mul(8)); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let (y_lo, y_hi): (u128, u128) = ((xmm0) & (xmm1), (ymm0_hi) & (ymm1_hi)); xmm1 = y_lo; ymm1_hi = y_hi; let y_addr: u64 = rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let (y_lo, y_hi): (u128, u128) = ((xmm0) & (xmm2), (ymm0_hi) & (ymm2_hi)); xmm2 = y_lo; ymm2_hi = y_hi; rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { let (y_lo, y_hi): (u128, u128) = ((xmm1) & (xmm2), (ymm1_hi) & (ymm2_hi)); xmm2 = y_lo; ymm2_hi = y_hi; xmm1 = (ymm2_hi) as u128; ymm1_hi = 0; xmm0 = crate::simd_ops::vpsrldq(xmm1, 0x8u8); ymm0_hi = 0; xmm3 = (xmm1) & (xmm0); xmm0 = crate::simd_ops::vpsrldq(xmm2, 0x8u8); ymm0_hi = 0; xmm0 = (xmm2) & (xmm0); ymm0_hi = 0; xmm1 = (xmm0) & (xmm3); ymm1_hi = 0; rcx = ((xmm1 as u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rcx = (((rcx) & (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))))) as u64; rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { rax = r11.wrapping_sub(0x1); rax = (!(rax)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; r9 = ((r14).wrapping_shr((((rax) as u32) & 63))) as u64; fl_l = rcx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(r10.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; rax = (((rax) & (r9))) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 16; continue; }
            16 => { r9 = rdi.wrapping_add(0xb0); bb = 17; continue; }
            17 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { rax = r9.wrapping_sub(0xb0); fl_l = r9; fl_r = r9; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { r8 = rax.wrapping_add(0xb0); bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); rcx = (crate::simd_ops::ru64(r8)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x48u64), (1) as u32); rcx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u64(rsi.wrapping_add(0x28u64), (1) as u64); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx.wrapping_add(0x8u64)))) as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0xe0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = rdi.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { rcx = rdi.wrapping_add(0xc0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            27 => { rcx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d3ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x3f; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0xfffffffffffffff8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((r10.wrapping_add(P1)) as u64) as u64; r8 = ((r8).wrapping_shr(6u32)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(P2)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = r9; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = ((r10.wrapping_add(P2)) as u64) as u64; rcx = (0xffffffffffffffffu64) as u64; rax = (!(rax)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rax = ((rcx).wrapping_shr((((rax) as u32) & 63))) as u64; fl_l = (crate::simd_ops::ru64(r9.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P3))) as u64; fl_r = rax; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d3c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffc0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x48; const P8: u64 = 0x8; const P9: u64 = 0x38; const P10: u64 = 0x20; const P11: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rsi = 0; fl_l = 0; fl_r = 0; rdi = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x50; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rsi; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rax = 0x10000; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } crate::simd_ops::wu32(r9.wrapping_add(P7), ((rsi as u32)) as u32); crate::simd_ops::wu64(r9.wrapping_add(P8), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P2), rax); rax = ((r9.wrapping_add(P9)) as u64) as u64; crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(P8), rax); rax = rbx; crate::simd_ops::wu64(r9, rcx); crate::simd_ops::wu64(r9.wrapping_add(P1), (0x10000u64) as u64); crate::simd_ops::wu64(r9.wrapping_add(P10), r8); crate::simd_ops::wu64(r9.wrapping_add(P11), rsi); crate::simd_ops::wu64(r9.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rbx, r9); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d3d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffc0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x48; const P8: u64 = 0x8; const P9: u64 = 0x38; const P10: u64 = 0x20; const P11: u64 = 0x28;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rbp = 0; fl_l = 0; fl_r = 0; rdi = r9; rsi = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x50; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); r10 = rbp; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; rax = r8; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } crate::simd_ops::wu32(r10.wrapping_add(P7), ((rbp as u32)) as u32); crate::simd_ops::wu64(r10.wrapping_add(P8), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(r10.wrapping_add(P2), rax); rax = ((r10.wrapping_add(P9)) as u64) as u64; crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(P8), rax); rax = rbx; crate::simd_ops::wu64(r10, rcx); crate::simd_ops::wu64(r10.wrapping_add(P1), r8); crate::simd_ops::wu64(r10.wrapping_add(P10), r9); crate::simd_ops::wu64(r10.wrapping_add(P11), rbp); crate::simd_ops::wu64(r10.wrapping_add(0x30u64), rbp); crate::simd_ops::wu64(rbx, r10); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d3de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142442de0; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x108; const P4: u64 = 0x4; const P5: u64 = 0x100; const P6: u64 = 0xe; const P7: u64 = 0x20; const P8: u64 = 0x18; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rcx = ((r9.wrapping_add(P2)) as u64) as u64; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(P7), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P8), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x190; const P1: u64 = 0x8; const P2: u64 = 0x19; const P3: u64 = 0x10; const P4: u64 = 0x140246bb0; const P5: u64 = 0x1401e66f0; const P6: u64 = 0x108; const P7: u64 = 0x14240ab10; const P8: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = ((rsi.wrapping_add(P0)) as u64) as u64; rcx = ((rsi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d40c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x19; const P1: u64 = 0x10; const P2: u64 = 0x1411d40c0; const P3: u64 = 0x128; const P4: u64 = 0x1411d4020; const P5: u64 = 0x20; const P6: u64 = 0x14240ab10; const P7: u64 = 0x1401e66f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = r8; rbp = rdx; r14 = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = rbp; rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rsi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x142442de0; const P2: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            4 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d41b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x142409b50; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x4u8) as u8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); rdx = ((rbx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu8(rdx, ((rax as u8)) as u8); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rdx, (0x0u8) as u8); bb = 7; continue; }
            4 => { r9 = (0xffffffffffffffffu64) as u64; bb = 5; continue; }
            5 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::wu8(rbx.wrapping_add(P4), (0x0u8) as u8); rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x142537280; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x18; const P7: u64 = 0x142061420;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rsi = r8; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(P2)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { rbx = rdx; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rbx); xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r9 = (crate::simd_ops::ru32(r9)) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rax); rax = rdi; r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffd8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x142062c90;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = rcx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x38; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rcx = r9; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rdx = 0x2000; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 1; continue; } bb = 5; continue; }
            5 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x10; const P2: u64 = 0x40; const P3: u64 = 0x142537280; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x18; const P7: u64 = 0x142063050;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = r8; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((rcx.wrapping_add(P2)) as u32) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; rcx = rax; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P6), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { rdx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d44d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1418349c0; const P1: u64 = 0xb10; const P2: u64 = 0xa80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rbx = rcx;
    rdi = (crate::simd_ops::ru64(rax)) as u64;
    rax = 0x70;
    fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rax = 0x467e0;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdi).wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rbx;
    if (((fl_l as u64)) != ((fl_r as u64))) { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { crate::simd_ops::wu64(rax.wrapping_add(rdi).wrapping_add(P1), rcx); crate::simd_ops::wu8(rax.wrapping_add(rdi).wrapping_add(P2), ((rcx as u8)) as u8); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64;
    r8 = rdx;
    rcx = ((rax as u32)) as u32 as u64;
    rdx = (0xe9721725f2cc49cu64) as u64;
    rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64;
    rdx = (0x3c5a37a36834ced9u64) as u64;
    r9 = (0x9ddfea08eb382d69u64) as u64;
    rcx = ((rcx).wrapping_sub(rdx)) as u64;
    rdx = (0x7c4ba56abfd08f4fu64) as u64;
    rax = (((rax) as i64).wrapping_mul((rdx) as i64) as u64) as u64;
    rcx = (((rcx) ^ (rax))) as u64;
    rax = rcx;
    rax = ((rax).wrapping_shr(47u32)) as u64;
    rax = (((rax) ^ (rcx))) as u64;
    rcx = (0x9ae16a3b2f90404fu64) as u64;
    rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64;
    rcx = (0x651e95c4d06fbfb1u64) as u64;
    rcx = ((rcx).wrapping_add(rax)) as u64;
    rcx = (((rcx) ^ (r8))) as u64;
    rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rdx = rcx;
    rdx = ((rdx).wrapping_shr(47u32)) as u64;
    rdx = (((rdx) ^ (rcx))) as u64;
    rdx = (((rdx) ^ (r8))) as u64;
    rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    rax = rdx;
    rax = ((rax).wrapping_shr(47u32)) as u64;
    rax = (((rax) ^ (rdx))) as u64;
    rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d45c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411d4900; const P1: u64 = 0x1411d45c0; const P2: u64 = 0x1411d4be0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r13 = (crate::simd_ops::ru64(rdx)) as u64; rbp = r9; r14 = (crate::simd_ops::ru64(rcx)) as u64; rax = r13; rax = ((rax).wrapping_sub(r14)) as u64; rsi = r8; rax = (((rax) & (0xfffffffffffffff8u64))) as u64; r15 = rdx; r12 = rcx; fl_l = rax; fl_r = (0x100u64) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rsi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { r9 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rdx = rsp.wrapping_add(0x38); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(r15)) as u64; rcx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = r8; rdx = (crate::simd_ops::ru64(r12)) as u64; r9 = rbp; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; rsi = ((((rsi) as i64).wrapping_shr(1u32) as u64)) as u64; rax = (((rax) & (0xfffffffffffffff8u64))) as u64; rsi = ((rsi).wrapping_add(rcx)) as u64; rcx = rbx; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = (((rcx) & (0xfffffffffffffff8u64))) as u64; fl_l = rcx; fl_r = rax; rcx = rsp.wrapping_add(0x30); if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); r8 = rsi; rdx = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r12, rdi); bb = 5; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r8); rdx = rsp.wrapping_add(0x38); r8 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r15, rbx); bb = 5; continue; }
            5 => { r13 = (crate::simd_ops::ru64(r15)) as u64; r14 = (crate::simd_ops::ru64(r12)) as u64; rax = r13; rax = ((rax).wrapping_sub(r14)) as u64; rax = (((rax) & (0xfffffffffffffff8u64))) as u64; fl_l = rax; fl_r = (0x100u64) as u64; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 1; continue; } bb = 6; continue; }
            6 => { fl_l = r14; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 7; continue; }
            7 => { rdi = r14.wrapping_add(0x8); fl_l = rdi; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { r15 = (crate::simd_ops::ru64(rdi)) as u64; rsi = rdi; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = r15; rax = crate::simd_ops::call_stk(rbp, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 10; continue; }
            10 => { fl_l = r14; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = rdi; rdx = rdi.wrapping_add(0x8); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rdx = rdx.wrapping_sub(0x8); rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; crate::simd_ops::wu64(rdx, rcx); fl_l = rax; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(r14, r15); bb = 38; continue; }
            14 => { r13 = ((r13).wrapping_sub(r14)) as u64; r13 = ((((r13) as i64).wrapping_shr(3u32) as u64)) as u64; rsi = r13; rsi = ((((rsi) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = rsi; fl_r = rsi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 28; continue; } bb = 15; continue; }
            15 => { rdx = r13.wrapping_sub(0x1); r12 = rdx; fl_l = ((((r12) as i64).wrapping_shr(1u32) as u64)) as u64; r12 = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { r15 = (crate::simd_ops::ru64(r14.wrapping_add(rsi.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; rsi = ((rsi).wrapping_sub(1)) as u64; rdi = rsi; rcx = rsi; fl_l = rsi; fl_r = r12; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 20; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rbx = ((rcx.wrapping_add(rcx)) as u64) as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rax = crate::simd_ops::call_stk(rbp, rax, rcx, rdx, r8, r9, rsp); fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rcx = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(r14.wrapping_add(rdi.wrapping_mul(8)), rax); rdi = rcx; fl_l = rcx; fl_r = r12; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 18; continue; } bb = 19; continue; }
            19 => { rdx = r13.wrapping_sub(0x1); bb = 20; continue; }
            20 => { fl_l = rdi; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = ((r13 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(r13.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(rdi.wrapping_mul(8)), rax); rdi = rdx; bb = 23; continue; }
            23 => { rbx = rdi.wrapping_sub(0x1); rbx = ((((rbx) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = rsi; fl_r = rdi; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 26; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(rbx.wrapping_mul(8)))) as u64; rdx = r15; rax = crate::simd_ops::call_stk(rbp, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(rbx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(r14.wrapping_add(rdi.wrapping_mul(8)), rax); rdi = rbx; rbx = ((rbx).wrapping_sub(1)) as u64; rbx = ((((rbx) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = rsi; fl_r = rdi; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 24; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu64(r14.wrapping_add(rdi.wrapping_mul(8)), r15); rdx = r13.wrapping_sub(0x1); fl_l = rsi; fl_r = rsi; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 16; continue; } bb = 27; continue; }
            27 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(r15)) as u64; rdi = (crate::simd_ops::ru64(r12)) as u64; rax = rcx; rax = ((rax).wrapping_sub(rdi)) as u64; rax = (((rax) & (0xfffffffffffffff8u64))) as u64; fl_l = rax; fl_r = (0x10u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 39; continue; } bb = 29; continue; }
            29 => { bb = 30; continue; }
            30 => { rbx = rcx.wrapping_sub(0x8); rcx = ((rcx).wrapping_sub(rdi)) as u64; rcx = (((rcx) & (0xfffffffffffffff8u64))) as u64; fl_l = rcx; fl_r = (0x10u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = rsp.wrapping_add(0x98); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); rcx = rsp.wrapping_add(0x90); rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = rbx; r8 = ((r8).wrapping_sub(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); r8 = ((((r8) as i64).wrapping_shr(3u32) as u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { rax = rbx; rcx = rbx; rax = ((rax).wrapping_sub(rdi)) as u64; rax = (((rax) & (0xfffffffffffffff8u64))) as u64; fl_l = rax; fl_r = (0x10u64) as u64; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 30; continue; } bb = 33; continue; }
            33 => { bb = 39; continue; }
            34 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_sub(0x8u64))) as u64; rbx = rdi.wrapping_sub(0x8); rcx = r15; rax = crate::simd_ops::call_stk(rbp, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 35; continue; }
            35 => { bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = r15; crate::simd_ops::wu64(rsi, rax); rsi = rbx; rdx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rbx = ((rbx).wrapping_sub(0x8u64)) as u64; rax = crate::simd_ops::call_stk(rbp, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 36; continue; } bb = 37; continue; }
            37 => { crate::simd_ops::wu64(rsi, r15); bb = 38; continue; }
            38 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 39; continue; }
            39 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411d4cd0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r15 = rdx; rdi = (crate::simd_ops::ru64(r8)) as u64; r13 = r9; r10 = rdi; r12 = r8; rdi = ((rdi).wrapping_sub(0x8u64)) as u64; r10 = ((r10).wrapping_sub(rax)) as u64; r10 = ((((r10) as i64).wrapping_shr(4u32) as u64)) as u64; rdx = rdi; rdx = ((rdx).wrapping_sub(rax)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rbx = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = (0x28u64) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = rbp.wrapping_sub(0x20); r15 = rdx; r15 = ((r15).wrapping_shl(4u32)) as u64; r14 = ((rdx.wrapping_mul(8)) as u64) as u64; r12 = ((r14.wrapping_add(rax)) as u64) as u64; rcx = ((r15.wrapping_add(rax)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rcx); rdx = rbp.wrapping_sub(0x18); rcx = rbp.wrapping_sub(0x10); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((r14.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); r8 = rbp.wrapping_sub(0x10); rax = rbx; rdx = rbp.wrapping_sub(0x18); rax = ((rax).wrapping_sub(r14)) as u64; rcx = rbp.wrapping_sub(0x20); r9 = r13; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rdi; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rdi); rsi = ((rsi).wrapping_sub(r14)) as u64; r8 = rbp.wrapping_sub(0x10); rdi = ((rdi).wrapping_sub(r15)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rsi); r9 = r13; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rdi); rdx = rbp.wrapping_sub(0x18); rcx = rbp.wrapping_sub(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = r13; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rsi); r8 = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rbx); rdx = rbp.wrapping_sub(0x18); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), r12); rcx = rbp.wrapping_sub(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; bb = 3; continue; }
            2 => { r8 = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rdi); rdx = rbp.wrapping_sub(0x18); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rbx); rcx = rbp.wrapping_sub(0x20); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru64(r15)) as u64; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rdi = rbx.wrapping_sub(0x8); rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(r13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(r13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rbx = rdi; fl_l = (crate::simd_ops::ru64(r15)) as u64; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { fl_l = rsi; fl_r = (crate::simd_ops::ru64(r12)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(r13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(r13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(r12)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { rdi = rsi; r14 = rbx; bb = 14; continue; }
            14 => { fl_l = rdi; fl_r = (crate::simd_ops::ru64(r12)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(r13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(r13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rdi, rcx); bb = 19; continue; }
            19 => { fl_l = ((rsi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(r12)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru64(r15)) as u64; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 22; continue; }
            22 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; r15 = r14.wrapping_sub(0x8); bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(r15)) as u64; rax = crate::simd_ops::call_stk(r13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(r13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rbx = ((rbx).wrapping_sub(0x8u64)) as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(r15, rcx); bb = 27; continue; }
            27 => { r14 = ((r14).wrapping_sub(0x8u64)) as u64; r15 = ((r15).wrapping_sub(0x8u64)) as u64; fl_l = (crate::simd_ops::ru64(r12)) as u64; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 28; continue; }
            28 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(r12)) as u64; fl_l = r14; fl_r = (crate::simd_ops::ru64(r15)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 30; continue; }
            30 => { fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 31; continue; }
            31 => { fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rsi, rcx); bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rsi = ((rsi).wrapping_add(0x8u64)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, rax); rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rdi, rcx); fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 14; continue; }
            34 => { r14 = ((r14).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 35; continue; }
            35 => { rbx = ((rbx).wrapping_sub(0x8u64)) as u64; fl_l = r14; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu64(rbx, rcx); bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rsi.wrapping_sub(0x8u64))) as u64; fl_l = rsi; fl_r = (0x8u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rsi, rcx); bb = 14; continue; }
            38 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rdi, rax); fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r14, rcx); bb = 14; continue; }
            39 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rax, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rsi); rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r13 = r8; r8 = ((r8).wrapping_sub(1)) as u64; r12 = r9; rdi = r8; rsi = rdx; rdi = ((((rdi) as i64).wrapping_shr(1u32) as u64)) as u64; r14 = rcx; r15 = rdx; fl_l = rdx; fl_r = rdi; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rbx = ((rdx.wrapping_add(rdx)) as u64) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rax = crate::simd_ops::call_stk(rbp, rax, rcx, rdx, r8, r9, rsp); fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rcx = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; rdx = rcx.wrapping_add(0x2); rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rsi.wrapping_mul(8)), rax); rsi = rdx; fl_l = rdx; fl_r = rdi; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 3; continue; }
            3 => { r8 = r13.wrapping_sub(0x1); bb = 4; continue; }
            4 => { if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((r13 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r13.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rsi.wrapping_mul(8)), rax); rsi = r8; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(r14)) as u64; rdi = rsi.wrapping_sub(0x1); rdi = ((((rdi) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r15; fl_r = rsi; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(r12)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(rdi.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(rbp, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(rdi.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(rsi.wrapping_mul(8)), rax); rsi = rdi; rdi = ((rdi).wrapping_sub(1)) as u64; rdi = ((((rdi) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r15; fl_r = rsi; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(r12)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(rsi.wrapping_mul(8)), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rdi = rcx; rsi = r9; r14 = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(rsi, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(r8, rcx); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(rsi, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r14)) as u64; r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(r8, rdx); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(rsi, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(r8, rdx); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x142537280; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x18; const P7: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rbx as u32)) as u32 as u64; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P6), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rax.wrapping_add(P1), rbx); crate::simd_ops::wu64(rax.wrapping_add(P6), rbx); crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(rax.wrapping_add(P7), rbx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142063360; const P4: u64 = 0xfffffffffffffff8; const P5: u64 = 0x142535070; const P6: u64 = 0x1426456c8; const P7: u64 = 0x1426453a0;
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
            5 => { rdx = 0x18; r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d4eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1402174f0; const P1: u64 = 0x14021b730; const P2: u64 = 0x14240aa00; const P3: u64 = 0x140208500; const P4: u64 = 0x140200130;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = r8; r14 = rdx; r15 = rcx; rbp = (crate::simd_ops::ru64(rcx)) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r12 as u32)) as u32); rdi = rbp; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = rbx.wrapping_add(0x1c); rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r12 as u32)) as u32); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; bb = 5; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rdi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = rdi.wrapping_add(0x1c); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (0x186186186186186u64) as u64; fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rbx = rax.wrapping_add(0x1c); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); crate::simd_ops::lk_and_u32(rbx, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rbx, (0x10u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x88u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbp); crate::simd_ops::wu16(rdi.wrapping_add(0x18u64), (0x0u16) as u16); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = rdi; rdx = rsp.wrapping_add(0x20); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu8(r14.wrapping_add(0x8u64), ((rax as u8)) as u8); rax = r14; r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1411d4ffc, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d52a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433dc308; const P1: u64 = 0x142645790; const P2: u64 = 0x143857a18; const P3: u64 = 0x143857a10; const P4: u64 = 0x1426455a8; const P5: u64 = 0x143a63208; const P6: u64 = 0x143a63210; const P7: u64 = 0x143857a1c; const P8: u64 = 0x14183acc8; const P9: u64 = 0x142645598; const P10: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbp = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rcx, rax); rsi = rcx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(P2)) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P3; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P2, ((rbx as u32)) as u32); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(P5)) as u64; r8 = (crate::simd_ops::ru32(P6)) as u32 as u64; rbx = rdx; crate::simd_ops::lk_add_u32(P7, (1) as u32); rcx = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = rdx; bb = 4; continue; }
            4 => { rbx = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = rax.wrapping_add(0x8); rax = rbx; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = rbx.wrapping_add(0x8); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx, rdx); fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru32(P6)) as u32 as u64; rdx = (crate::simd_ops::ru64(P5)) as u64; bb = 12; continue; }
            12 => { rax = ((r8 as u32)) as u32 as u64; r8 = ((rax.wrapping_mul(8)) as u64) as u64; rdi = ((r8.wrapping_add(rdx)) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(P6, (0x0u32) as u32); bb = 16; continue; }
            15 => { r8 = ((r8).wrapping_sub(rdi)) as u64; rcx = rbx; r8 = ((r8).wrapping_add(rdx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_sub(rbx)) as u64; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(P6, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(P7, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = 0xffffffff; rcx = P3; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P2, ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = 0x8; rcx = rsi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d5690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a631f4; const P1: u64 = 0x143a631f0; const P2: u64 = 0x142537280; const P3: u64 = 0x142535070; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x143a631e8; const P7: u64 = 0x14183acc8; const P8: u64 = 0x1401de830;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rsi = (crate::simd_ops::ru32(P1)) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; fl_l = rdi; fl_r = rdx; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rdi }) as u64; rbx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rax); bb = 13; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = rbx; bb = 13; continue; }
            7 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            11 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { r8 = rax; bb = 13; continue; }
            13 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            15 => { rbx = r8; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rcx = rbx; rdx = (crate::simd_ops::ru64(P6)) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = P6; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P6, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(P1, ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(P0, ((rdi as u32)) as u32); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d5bf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l;
    if !((((fl_l as u32)) != 0)) { rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d5c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x143c48ba0; const P2: u64 = 0x143a359f8; const P3: u64 = 0x143c48e58; const P4: u64 = 0x18; const P5: u64 = 0x1433dc360; const P6: u64 = 0x40; const P7: u64 = 0x30; const P8: u64 = 0x12; const P9: u64 = 0x15;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r9; r14 = r8; rdi = rdx; rbx = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(r9, ((rcx as u8)) as u8); bb = 9; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; r8 = rdi; crate::simd_ops::wu64(r14, rax); rdx = r14; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as i8 as i32) as u32) as u32 as u64; rcx = P5; rax = ((crate::simd_ops::ru8(rax.wrapping_add(rcx))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi, ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rbx.wrapping_add(P7)) as u64) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; rax = 0x20; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x8; rdx = 0x10; rax = (rax & !0xffffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { (rdx as u16) } else { (rax as u16) }) as u16 as u64)); rdx = 0x6; crate::simd_ops::wu16(rdi.wrapping_add(P8), ((rax as u16)) as u16); rax = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x8; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(P9), ((rax as u8)) as u8); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d5cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x141834d30; const P3: u64 = 0x8; const P4: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d5d4a, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d5d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140202a90; const P2: u64 = 0x10; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x141834d30; const P5: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; rax = ((r8.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { r8 = rcx; bb = 6; continue; }
            6 => { rcx = r8; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); crate::simd_ops::wu64(rsi.wrapping_add(P2), rax); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d5e01, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d5e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x19; const P2: u64 = 0x10; const P3: u64 = 0x1411d61c0; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    rsi = rcx;
    rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) != ((fl_r as u8))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), rdi); crate::simd_ops::wu64(rdi, rdi); crate::simd_ops::wu64(rdi.wrapping_add(P2), rdi); crate::simd_ops::wu64(rsi.wrapping_add(P0), (0x0u64) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = rsi; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rdx = 0x30; rbx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) == ((fl_r as u8)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P0), rdi);
    crate::simd_ops::wu64(rdi, rdi);
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rdi);
    crate::simd_ops::wu64(rsi.wrapping_add(P0), (0x0u64) as u64);
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d5e80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x19; const P2: u64 = 0x10; const P3: u64 = 0x1411d61c0; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    rdi = rcx;
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) != ((fl_r as u8))) { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0x30; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    loop { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = rdi; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rdx = 0x30; rbx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) == ((fl_r as u8)))) { break; } }
    rcx = (crate::simd_ops::ru64(rdi)) as u64;
    rdx = 0x30;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d5ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x141834d30; const P3: u64 = 0x8; const P4: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff0u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d5f3b, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d5f40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a35a98; const P1: u64 = 0xd0; const P2: u64 = 0x140202a90; const P3: u64 = 0xb8; const P4: u64 = 0xa0; const P5: u64 = 0x88; const P6: u64 = 0x70; const P7: u64 = 0x58; const P8: u64 = 0x40; const P9: u64 = 0x1411d5cd0; const P10: u64 = 0x28; const P11: u64 = 0x1411d5d50; const P12: u64 = 0x1411d5e80; const P13: u64 = 0x1411d5ee0; const P14: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(P0)) as u64;
    crate::simd_ops::wu64(P0, rdx);
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d5fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x141834d64; const P4: u64 = 0x1426cdcb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411d5fd0, funcinfo_rva: 0x367e710, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r9) as u64; r14 = (r8) as u64; r15 = (rcx) as u64; r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x158u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = (rbx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (rax) as u64); bb = 5; continue; }
            3 => { rdx = (0x150u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d6039); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d6046); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d6054); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (r12) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (rbx) as u64); rsi = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu64(rbx, (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (r12) as u64); rdi = ((rbx.wrapping_add(0x18u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (rdi) as u64); crate::simd_ops::wu64(rdi, (r12) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (r12) as u64); rcx = (0x30u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d6093); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1411d6093, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1411d5fd0) }; crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x101u16) as u16); crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xb8u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xd0u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xd8u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xe0u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xf8u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x100u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), (r12) as u64); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x118u64), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x120u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x128u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x130u64), (r12) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x138u64), (r12) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x140u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x144u64), (r12) as u64); crate::simd_ops::wu8(rbx.wrapping_add(0x14cu64), (0x0u8) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x14du64), ((rsi as u8)) as u8); crate::simd_ops::wu64(r15, (rbx) as u64); rax = (r15) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d61c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x19; const P1: u64 = 0x10; const P2: u64 = 0x1411d61c0; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = r8;
    rdi = rdx;
    rsi = rcx;
    if (((fl_l as u8)) != ((fl_r as u8))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rdx = 0x30; rbx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) == ((fl_r as u8)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d6220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x14183acc8; const P2: u64 = 0x141834d30; const P3: u64 = 0x1426467f0; const P4: u64 = 0x1402032e0; const P5: u64 = 0x1401e6f50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = rdx; rdx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rbp; rsi = ((rsi).wrapping_sub(rdx)) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; r12 = r8; rsi = ((((rsi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = (0x1fffffffffffffffu64) as u64; rbx = rcx; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; r15 = rax.wrapping_add(0x1); rdx = rcx; rax = r8; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = r8; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r14 = ((rcx.wrapping_mul(8)) as u64) as u64; rcx = r14; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r12)) as u64; rdi = rax; crate::simd_ops::wu64(rax.wrapping_add(rsi.wrapping_mul(8)), rcx); rcx = rax; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rsi = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; fl_l = rbp; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = r8; fl_r = rdx; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 8; continue; }
            5 => { rax = ((rdx.wrapping_add(rcx)) as u64) as u64; rcx = r15; fl_l = rax; fl_r = r15; rcx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 6; continue; }
            6 => { bb = 3; continue; }
            7 => { r8 = rbp; r8 = ((r8).wrapping_sub(rdx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rsi.wrapping_add(0x8); fl_l = r8; fl_r = rbp; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rdx = rbp; bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = r8; bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rcx = ((rdi.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbx, rdi); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = rsi; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); rcx = ((r14.wrapping_add(rdi)) as u64) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d636f, rcx, rsp); return rax; }
            15 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d6375, rcx, rsp); return rax; }
            16 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d637b, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d6380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x14183acc2; const P2: u64 = 0x14183acc8; const P3: u64 = 0x141834d30; const P4: u64 = 0x1426467f0; const P5: u64 = 0x1402032e0; const P6: u64 = 0x1401e6f50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rsi = rdx; r8 = (0x1fffffffffffffffu64) as u64; rbx = rcx; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 19; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rax = r8; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = ((rbp).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rdx = rcx; rbp = ((((rbp) as i64).wrapping_shr(3u32) as u64)) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = r8; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r15 = ((rax.wrapping_mul(8)) as u64) as u64; rcx = r15; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; r14 = rax; rdx = ((rdx).wrapping_sub(rbp)) as u64; fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = (0x0u64) as u64; rcx = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { r8 = ((rdx.wrapping_mul(8)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            5 => { rax = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = rsi; bb = 3; continue; }
            7 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 20; continue; } bb = 8; continue; }
            8 => { bb = 3; continue; }
            9 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rcx, rax); rcx = rcx.wrapping_add(0x8); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = r14; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { rcx = r8; bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((r14.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbx, r14); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = ((r15.wrapping_add(r14)) as u64) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d64bb, rcx, rsp); return rax; }
            19 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d64c1, rcx, rsp); return rax; }
            20 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d64c7, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d64d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x14183acc2; const P2: u64 = 0x14183acc8; const P3: u64 = 0x141834d30; const P4: u64 = 0x1426467f0; const P5: u64 = 0x1402032e0; const P6: u64 = 0x1401e6f50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = (0xaaaaaaaaaaaaaaau64) as u64; rdi = rdx; rbx = rcx; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rbp = (0x2aaaaaaaaaaaaaabu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = rbp; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64; r14 = rdx; r14 = ((((r14) as i64).wrapping_shr(2u32) as u64)) as u64; rax = r14; rax = ((rax).wrapping_shr(63u32)) as u64; r14 = ((r14).wrapping_add(rax)) as u64; rax = rbp; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = r9; rcx = rdx; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rax = r9; bb = 3; continue; }
            3 => { rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = ((rax.wrapping_mul(8)) as u64) as u64; rcx = r15; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rax = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; rcx = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = rdi; fl_l = rax; fl_r = r14; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsi; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = rbp; rcx = ((rcx).wrapping_sub(r8)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r8.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; rax = r8.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 8; continue; }
            8 => { r8 = rcx; bb = 9; continue; }
            9 => { rcx = r8; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((r15.wrapping_add(rsi)) as u64) as u64; crate::simd_ops::wu64(rbx, rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rax = ((rcx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = rdi; bb = 3; continue; }
            13 => { fl_l = rax; fl_r = r9; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { bb = 3; continue; }
            15 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d664e, rcx, rsp); return rax; }
            16 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d6654, rcx, rsp); return rax; }
            17 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d665a, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d6660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(3u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rbx.wrapping_add(rdi)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d66a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x1411d6a10; const P2: u64 = 0x140202a90; const P3: u64 = 0x141834d30; const P4: u64 = 0x1426467f0; const P5: u64 = 0x1402032e0; const P6: u64 = 0x1401e6f50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = rdx; rbx = rdx; r13 = (0x2aaaaaaaaaaaaaabu64) as u64; rdi = rcx; rbp = r8; rax = r13; r8 = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_sub(r8)) as u64; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; r9 = (0xaaaaaaaaaaaaaaau64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = r13; rcx = ((rcx).wrapping_sub(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); r15 = rdx.wrapping_add(0x1); let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = r9; rcx = rdx; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rcx = r9; bb = 3; continue; }
            3 => { rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); r12 = ((rax.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); rcx = r12; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; r14 = rax; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rbp, rax); rax = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; rbp = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r8); r8 = r14; crate::simd_ops::wu64(rbp, rcx); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rdx); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rbp.wrapping_add(0x18); rcx = rbx; bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; rax = r13; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(r8.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; rax = r8.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 11; continue; }
            11 => { r8 = rcx; bb = 12; continue; }
            12 => { rcx = r8; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rcx = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rdi, r14); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = ((r14.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = ((r12.wrapping_add(r14)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = rbp; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rcx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rax = ((rcx.wrapping_add(rdx)) as u64) as u64; rcx = r15; fl_l = rax; fl_r = r15; rcx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { bb = 3; continue; }
            16 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d6882, rcx, rsp); return rax; }
            17 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d6888, rcx, rsp); return rax; }
            18 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d688e, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d6890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e9110; const P1: u64 = 0x14183acc2; const P2: u64 = 0x14183acc8; const P3: u64 = 0x141834d30; const P4: u64 = 0x1426467f0; const P5: u64 = 0x1402032e0; const P6: u64 = 0x1401e6f50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (0xfffffffffffffffu64) as u64; rbx = rdx; rdi = rcx; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rax = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r14 = ((r14).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi))) as u64; rcx = ((((rcx) as i64).wrapping_shr(4u32) as u64)) as u64; rdx = rcx; r14 = ((((r14) as i64).wrapping_shr(4u32) as u64)) as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rsi = r8; bb = 3; continue; }
            3 => { rsi = ((rsi).wrapping_shl(4u32)) as u64; rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = r14; r8 = rbx; rcx = ((rcx).wrapping_shl(4u32)) as u64; rbp = rax; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = r8; fl_r = r14; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((r8).wrapping_shl(4u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rbp; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff0u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = rcx.wrapping_sub(0x8); fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 8; continue; }
            8 => { rcx = r8; bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rsi.wrapping_add(rbp)) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi, rbp); rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(rbp)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rsi = ((rdx.wrapping_add(rcx)) as u64) as u64; fl_l = rsi; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rsi = rbx; bb = 3; continue; }
            13 => { fl_l = rsi; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { bb = 3; continue; }
            15 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d69b8, rcx, rsp); return rax; }
            16 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d69be, rcx, rsp); return rax; }
            17 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1411d69c4, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d69d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rbx = ((rbx).wrapping_shl(4u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rbx.wrapping_add(rdi)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d6a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x8u64)) as u64;
    rax = r8;
    r11 = rdx;
    fl_l = rcx; fl_r = rdx;
    if (((fl_l as u64)) == ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
    r10 = rax;
    crate::simd_ops::wu64(rsp, rbx);
    r10 = ((r10).wrapping_sub(rcx)) as u64;
    r8 = ((rcx.wrapping_add(P0)) as u64) as u64;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    loop { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; r9 = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P0), rbx); crate::simd_ops::wu64(r8, rbx); crate::simd_ops::wu64(r8.wrapping_add(P1), rbx); crate::simd_ops::wu64(rax, rcx); rax = ((rax).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(r10.wrapping_add(r8), rdx); crate::simd_ops::wu64(r10.wrapping_add(r8).wrapping_add(P0), r9); r8 = ((r8.wrapping_add(P2)) as u64) as u64; rcx = ((r8.wrapping_add(P1)) as u64) as u64; fl_l = rcx; fl_r = r11; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp)) as u64;
    rsp = ((rsp).wrapping_add(0x8u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d6a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x30; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d6a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14206b020; const P2: u64 = 0x142442de0; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x108; const P5: u64 = 0x4; const P6: u64 = 0x100; const P7: u64 = 0xe; const P8: u64 = 0x20; const P9: u64 = 0x18; const P10: u64 = 0x1425371a0; const P11: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411d6a90, funcinfo_rva: 0x367e9ec, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rcx) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbp = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d6acb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d6ad3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rcx = ((r9.wrapping_add(P3)) as u64) as u64; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rcx.wrapping_add(P4)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P8), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P9), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d6b43); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 13; continue; }
            10 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            11 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d6b52); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 13; continue; }
            13 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, (rax) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), (rax) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d6b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a63250; const P1: u64 = 0x143a63248; const P2: u64 = 0x143a63254; const P3: u64 = 0x1411d6cb0; const P4: u64 = 0x1411d6e60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r9; rdi = rdx; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx.wrapping_mul(8)) as u64) as u64; r8 = (crate::simd_ops::ru64(P1)) as u64; r9 = ((rcx.wrapping_add(r8)) as u64) as u64; rax = (0x6666666666666667u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rdi; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x10u64)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (0x100u64) as u64); crate::simd_ops::wu64(rcx.wrapping_sub(0x10u64), rax); crate::simd_ops::wu64(rcx.wrapping_sub(0x8u64), rax); crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rcx = rcx.wrapping_add(0x28); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { bb = 12; continue; }
            5 => { fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_add(0x10u64)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (0x100u64) as u64); crate::simd_ops::wu64(rcx.wrapping_sub(0x10u64), rax); crate::simd_ops::wu64(rcx.wrapping_sub(0x8u64), rax); crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rcx = rcx.wrapping_add(0x28); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { bb = 12; continue; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; rdx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = r9; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu32(P0, ((rdi as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d6cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a63250; const P1: u64 = 0x142537280; const P2: u64 = 0x142535070; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x143a63248; const P6: u64 = 0x1411d6a90; const P7: u64 = 0x143a63254;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rdx; rsi = (crate::simd_ops::ru32(P0)) as u32 as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 13; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = rbp; bb = 13; continue; }
            7 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            11 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = rax; bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rbp; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(P5)) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rdx = rbx.wrapping_add(0x10); rcx = rax.wrapping_add(0x10); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x10u64), rbp); crate::simd_ops::wu64(rdx.wrapping_sub(0x10u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x8u64), rax); rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(0x10u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x11u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(0x11u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((rax as u32)) as u32); rdx = rdx.wrapping_add(0x28); rcx = rcx.wrapping_add(0x28); rax = rcx.wrapping_sub(0x10); fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = P5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P5, rbx); crate::simd_ops::wu32(P0, ((rsi as u32)) as u32); crate::simd_ops::wu32(P7, ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d6e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14206b020; const P1: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411d6e60, funcinfo_rva: 0x359dc18, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (r8) as u64; rbx = (rdx) as u64; bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d6e8b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411d6e93); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d6ed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x1411d7a20; const P2: u64 = 0x88; const P3: u64 = 0x8; const P4: u64 = 0x1401de400; const P5: u64 = 0xfffffffffffffff8; const P6: u64 = 0x141834ee8; const P7: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; r9 = P4; rdx = 0x10; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d6f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x1411d7a20; const P2: u64 = 0x88; const P3: u64 = 0x8; const P4: u64 = 0x1401de400; const P5: u64 = 0xfffffffffffffff8; const P6: u64 = 0x141834ee8; const P7: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; r9 = P4; rdx = 0x10; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d6f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x1411d7a20; const P2: u64 = 0x88; const P3: u64 = 0x8; const P4: u64 = 0x1401de400; const P5: u64 = 0xfffffffffffffff8; const P6: u64 = 0x141834ee8; const P7: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; r9 = P4; rdx = 0x10; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d7000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411d7af0; const P1: u64 = 0x14183acc2;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = (0x6666666666666667u64) as u64; rax = r10; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(6u32) as u64)) as u64; r8 = rdx; r8 = ((r8).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(1)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = r10; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rdx; rcx = ((((rcx) as i64).wrapping_shr(6u32) as u64)) as u64; rax = rcx; rax = ((rax).wrapping_shr(63u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rcx; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = r8; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rdx }) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xa0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x19u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x21u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x29u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x31u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x39u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x41u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x49u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x51u64), (0x0u64) as u64); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rbx.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu16(rbx.wrapping_add(0x80u64), (0x0u16) as u16); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x82u64), ((rax as u32)) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0x86u64), ((rax as u16)) as u16); crate::simd_ops::wu8(rbx.wrapping_add(0x88u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), rax); crate::simd_ops::wu32(rbx.wrapping_add(0x98u64), ((rax as u32)) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0x9cu64), ((rax as u16)) as u16); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(0xffffffffffffff60u64)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d7440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffb0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x50; const P8: u64 = 0x60; const P9: u64 = 0x48; const P10: u64 = 0x20; const P11: u64 = 0x1433dc878;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = rcx; rcx = 0x6348; rsi = 0; fl_l = 0; fl_r = 0; rbx = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x60; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsi; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rsi as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = P11; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu64(rdx, rax); rax = rdx.wrapping_add(0x38); crate::simd_ops::wu128(rdx.wrapping_add(0x8u64), (xmm0) as u128); crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x28u64), rax); rax = rdi; crate::simd_ops::wu32(rdx.wrapping_add(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x34u64), (0x2u32) as u32); crate::simd_ops::wu8(rdx.wrapping_add(0x58u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rdi, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d7530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x50; const P4: u64 = 0x60; const P5: u64 = 0x48; const P6: u64 = 0x1433dc898; const P7: u64 = 0x8; const P8: u64 = 0x20; const P9: u64 = 0x28; const P10: u64 = 0xffffffffffffffe0; const P11: u64 = 0x142535070; const P12: u64 = 0x1426456c8; const P13: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rsi = rcx; rcx = 0x6348; rbx = 0; fl_l = 0; fl_r = 0; rbp = r9; rdi = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rdx = ((crate::simd_ops::ru8(rbp)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = P6; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu64(rbx, rax); rax = rsi; crate::simd_ops::wu128(rbx.wrapping_add(P7), (xmm0) as u128); crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x1u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P8), rcx); crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = 0x30; r8 = ((rdx.wrapping_add(P10)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d7610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffe0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x50; const P8: u64 = 0x60; const P9: u64 = 0x48; const P10: u64 = 0x1433dc8b8; const P11: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = rcx; rcx = 0x6348; rbx = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x30; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = P10; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu64(rdx, rax); rax = rdi; crate::simd_ops::wu128(rdx.wrapping_add(P11), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), (0x2u32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rcx); crate::simd_ops::wu64(rdx.wrapping_add(0x28u64), (0x0u64) as u64); crate::simd_ops::wu64(rdi, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d76d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x50; const P4: u64 = 0x60; const P5: u64 = 0x48; const P6: u64 = 0x1433dc8a8; const P7: u64 = 0x8; const P8: u64 = 0x20; const P9: u64 = 0xffffffffffffffe8; const P10: u64 = 0x142535070; const P11: u64 = 0x1426456c8; const P12: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rsi = rcx; rcx = 0x6348; rbx = 0; fl_l = 0; fl_r = 0; rdi = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = P6; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu64(rbx, rax); rax = rsi; crate::simd_ops::wu128(rbx.wrapping_add(P7), (xmm0) as u128); crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x3u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P8), rcx); crate::simd_ops::wu64(rsi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = 0x28; r8 = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d77a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x50; const P4: u64 = 0x60; const P5: u64 = 0x48; const P6: u64 = 0x1433dc888; const P7: u64 = 0x8; const P8: u64 = 0x20; const P9: u64 = 0xffffffffffffffe8; const P10: u64 = 0x142535070; const P11: u64 = 0x1426456c8; const P12: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rsi = rcx; rcx = 0x6348; rbx = 0; fl_l = 0; fl_r = 0; rdi = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = P6; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu64(rbx, rax); rax = rsi; crate::simd_ops::wu128(rbx.wrapping_add(P7), (xmm0) as u128); crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x4u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P8), rcx); crate::simd_ops::wu64(rsi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = 0x28; r8 = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d7870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x50; const P4: u64 = 0x60; const P5: u64 = 0x48; const P6: u64 = 0x1433dc8c8; const P7: u64 = 0x8; const P8: u64 = 0x20; const P9: u64 = 0xffffffffffffffe8; const P10: u64 = 0x142535070; const P11: u64 = 0x1426456c8; const P12: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rsi = rcx; rcx = 0x6348; rbx = 0; fl_l = 0; fl_r = 0; rdi = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = P6; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu64(rbx, rax); rax = rsi; crate::simd_ops::wu128(rbx.wrapping_add(P7), (xmm0) as u128); crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x5u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P8), rcx); crate::simd_ops::wu64(rsi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = 0x28; r8 = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d7a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x1401de830; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x108; const P5: u64 = 0x4; const P6: u64 = 0x100; const P7: u64 = 0xe; const P8: u64 = 0x20; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rsi = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rcx = ((r9.wrapping_add(P3)) as u64) as u64; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rcx.wrapping_add(P4)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(P8), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P1), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 10; continue; } bb = 6; continue; }
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
pub unsafe extern "win64" fn fn_1411d7af0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1411d7950;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rsi = rdx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; r10 = 0x6348; r14 = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdi = rcx; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 13; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = r14; bb = 13; continue; }
            7 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            11 => { r8 = 0x10; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rbx = rax; bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = r14; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rdx = rbx.wrapping_add(0x10); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdx.wrapping_add(0xa0); crate::simd_ops::wu64(rcx, r14); crate::simd_ops::wu64(rdx.wrapping_sub(0xb0u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r14); crate::simd_ops::wu64(rdx.wrapping_sub(0xa8u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r14); crate::simd_ops::wu64(rdx.wrapping_sub(0xa0u64), rax); let y_addr: u64 = rcx.wrapping_add(0x18u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_sub(0x98u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x38u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_sub(0x78u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = rcx.wrapping_add(0x58u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdx.wrapping_sub(0x58u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rdx.wrapping_sub(0x38u64), ((xmm1 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdx.wrapping_sub(0x30u64), ((xmm0 as u64)) as u64); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x88u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_sub(0x28u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x18u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x9cu64))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0xa0u64)) as u64; crate::simd_ops::wu16(rdx.wrapping_sub(0x14u64), ((rax as u16)) as u16); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d7cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x98; const P2: u64 = 0x1411d7a20; const P3: u64 = 0x88; const P4: u64 = 0x8; const P5: u64 = 0x1401de400; const P6: u64 = 0xfffffffffffffff8; const P7: u64 = 0x141834ee8; const P8: u64 = 0x142442de0; const P9: u64 = 0x48; const P10: u64 = 0x14240ab10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; r9 = P5; rdx = 0x10; r8 = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d7da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x142537280; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x8; const P6: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P0), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx); bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rax.wrapping_add(P5), (0xffffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P6), (0x0u32) as u32); crate::simd_ops::wu64(rax, (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d7e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffe0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x8; const P8: u64 = 0xc; const P9: u64 = 0x20; const P10: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x30; rcx = rax; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rcx.wrapping_add(P7), (0xffffffffu32) as u32); rax = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P8), ((rbx as u32)) as u32); crate::simd_ops::wu64(rcx, rbx); crate::simd_ops::wu64(rcx.wrapping_add(P1), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P2), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P9), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P10), rbx); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d82e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fc10; const P1: u64 = 0x142645790; const P2: u64 = 0x1426455a8; const P3: u64 = 0x1433dc7e8; const P4: u64 = 0x1411d86e0; const P5: u64 = 0x14397fc80; const P6: u64 = 0x14205d410; const P7: u64 = 0x1403497a0; const P8: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x27); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rdi = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), (1) as u32); r14 = 0x80; r12 = ((r14.wrapping_sub(0x7fu64)) as u32) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; r13 = P3; bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (0x80000u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x35u64), (0x10001u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x2du64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x25u64), r12); let y_addr: u64 = rbp.wrapping_sub(0x51u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x19u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x31u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (0x2u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x61u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5du64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((r12 as u32)) as u32); rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp.wrapping_add(0x77); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xac3u32) as u32); r9 = rbp.wrapping_sub(0x19); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_sub(0x69); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rsi, rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = rsi.wrapping_add(0x10); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x1cu64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((r15 as u32)) as u32); bb = 13; continue; }
            13 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x20u64), (1) as u32); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x28u64), (1) as u64); fl_l = r14; fl_r = r12; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; rsp = ((rsp).wrapping_add(0xd0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d85c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411d86e0; const P1: u64 = 0x14397fc80; const P2: u64 = 0x1433dc7e8; const P3: u64 = 0x14205d410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rbp.wrapping_add(0x67); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); r9 = rbp.wrapping_add(0x17); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x5u64), (0x10001u64) as u64); rbx = rax; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), r8); rax = 0x1; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (0x80000u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xbu64), rax); rdx = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); rdx = rbp.wrapping_sub(0x39); crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), ((rax as u32)) as u32); let y_addr: u64 = rbp.wrapping_sub(0x21u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x17u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x3u64), rax); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x1u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8); crate::simd_ops::wu64(rbp.wrapping_add(0x47u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x2u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xac3u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(rbx, rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx)) as u64; r9 = rbx.wrapping_add(0x10); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rsp = ((rsp).wrapping_add(0xd0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d86e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x18; rcx = rax; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rcx, (0x0u64) as u64); rax = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P1), (0x0u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P7), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d8770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fc18; const P1: u64 = 0x142645790; const P2: u64 = 0x1426455a8; const P3: u64 = 0x1411d8e70; const P4: u64 = 0x1403497a0; const P5: u64 = 0x142645598; const P6: u64 = 0x141173780;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbp = rcx; r14 = r8.wrapping_add(0x3f); r14 = (((r14) & (0xffffffffffffffc0u64))) as u64; fl_l = r14; fl_r = (0x400000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; ymm0_hi = 0; rax = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rdx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rax); bb = 20; continue; }
            2 => { r15 = rcx.wrapping_add(0x10); rdx = (crate::simd_ops::ru64(r15)) as u64; r8 = rcx.wrapping_add(0x20); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r13 = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r8)) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; fl_l = rcx; fl_r = (0x400000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r8, r13); rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 6; continue; }
            6 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 7; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = rax; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), r12); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x1cu64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r13 as u32)) as u32); bb = 11; continue; }
            11 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x20u64), (1) as u32); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x28u64), (1) as u64)) as u64; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((r13 as u32)) as u32); bb = 15; continue; }
            15 => { rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rax as u32)) as u32); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); rdx = rsp.wrapping_add(0x70); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rbp.wrapping_add(0x10); r8 = rbp.wrapping_add(0x20); bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r15)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(rsi, r13); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), r13); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r13); xmm0 = (crate::simd_ops::ru128(rax)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsi, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r8))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rax); rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x24u64), ((r14 as u32)) as u32); crate::simd_ops::lk_add_u64(r8, (r14) as u64); fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x28u64), (1) as u64)) as u64; bb = 20; continue; }
            20 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d8b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fc18; const P1: u64 = 0x142645790; const P2: u64 = 0x1426455a8; const P3: u64 = 0x1433dc7e8; const P4: u64 = 0x1411d86e0; const P5: u64 = 0x14397fc80; const P6: u64 = 0x14205d410; const P7: u64 = 0x1403497a0; const P8: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x27); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rdi = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), (1) as u32); r14 = 0x10; r12 = ((r14.wrapping_sub(0xfu64)) as u32) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; r13 = P3; bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (0x400000u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x35u64), (0x10001u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x2du64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x25u64), r12); let y_addr: u64 = rbp.wrapping_sub(0x51u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x19u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x31u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (0x2u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x61u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5du64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((r12 as u32)) as u32); rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp.wrapping_add(0x77); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xac3u32) as u32); r9 = rbp.wrapping_sub(0x19); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_sub(0x69); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rsi, rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = rsi.wrapping_add(0x10); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x1cu64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((r15 as u32)) as u32); bb = 13; continue; }
            13 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x20u64), (1) as u32); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x28u64), (1) as u64); fl_l = r14; fl_r = r12; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; rsp = ((rsp).wrapping_add(0xd0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d8e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411d86e0; const P1: u64 = 0x14397fc80; const P2: u64 = 0x1433dc7e8; const P3: u64 = 0x14205d410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rbp.wrapping_add(0x67); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); r9 = rbp.wrapping_add(0x17); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x5u64), (0x10001u64) as u64); rbx = rax; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), r8); rax = 0x1; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (0x400000u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xbu64), rax); rdx = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); rdx = rbp.wrapping_sub(0x39); crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), ((rax as u32)) as u32); let y_addr: u64 = rbp.wrapping_sub(0x21u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x17u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x3u64), rax); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x1u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8); crate::simd_ops::wu64(rbp.wrapping_add(0x47u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x2u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xac3u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(rbx, rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx)) as u64; r9 = rbx.wrapping_add(0x10); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rsp = ((rsp).wrapping_add(0xd0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d93b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fc20; const P1: u64 = 0x142645790; const P2: u64 = 0x1426455a8; const P3: u64 = 0x1433dc7e8; const P4: u64 = 0x1411d86e0; const P5: u64 = 0x14397fc80; const P6: u64 = 0x14205d410; const P7: u64 = 0x1403497a0; const P8: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x27); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rdi = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), (1) as u32); r14 = 0x8; r12 = ((r14.wrapping_sub(0x7u64)) as u32) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; r13 = P3; bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (0x80000u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x35u64), (0x10001u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x2du64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x25u64), r12); let y_addr: u64 = rbp.wrapping_sub(0x51u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x19u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x31u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (0x2u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x61u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5du64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((r12 as u32)) as u32); rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp.wrapping_add(0x77); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xac3u32) as u32); r9 = rbp.wrapping_sub(0x19); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_sub(0x69); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rsi, rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = rsi.wrapping_add(0x10); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x1cu64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((r15 as u32)) as u32); bb = 13; continue; }
            13 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x20u64), (1) as u32); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x28u64), (1) as u64); fl_l = r14; fl_r = r12; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; rsp = ((rsp).wrapping_add(0xd0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d9ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fc28; const P1: u64 = 0x142645790; const P2: u64 = 0x1426455a8; const P3: u64 = 0x1433dc7e8; const P4: u64 = 0x1411d86e0; const P5: u64 = 0x14397fc80; const P6: u64 = 0x14205d410; const P7: u64 = 0x1403497a0; const P8: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x27); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rdi = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), (1) as u32); r14 = 0x80; r12 = ((r14.wrapping_sub(0x7fu64)) as u32) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; r13 = P3; bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (0x10000u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x35u64), (0x10001u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x2du64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x25u64), r12); let y_addr: u64 = rbp.wrapping_sub(0x51u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x19u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x31u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (0x2u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x61u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5du64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((r12 as u32)) as u32); rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp.wrapping_add(0x77); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xac3u32) as u32); r9 = rbp.wrapping_sub(0x19); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_sub(0x69); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rsi, rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = rsi.wrapping_add(0x10); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x1cu64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((r15 as u32)) as u32); bb = 13; continue; }
            13 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x20u64), (1) as u32); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x28u64), (1) as u64); fl_l = r14; fl_r = r12; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; rsp = ((rsp).wrapping_add(0xd0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411d9d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411d86e0; const P1: u64 = 0x14397fc80; const P2: u64 = 0x1433dc7e8; const P3: u64 = 0x14205d410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rbp.wrapping_add(0x67); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); r9 = rbp.wrapping_add(0x17); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x5u64), (0x10001u64) as u64); rbx = rax; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), r8); rax = 0x1; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (0x10000u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xbu64), rax); rdx = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); rdx = rbp.wrapping_sub(0x39); crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), ((rax as u32)) as u32); let y_addr: u64 = rbp.wrapping_sub(0x21u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x17u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x3u64), rax); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x1u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8); crate::simd_ops::wu64(rbp.wrapping_add(0x47u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x2u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xac3u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(rbx, rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx)) as u64; r9 = rbx.wrapping_add(0x10); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rsp = ((rsp).wrapping_add(0xd0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411da080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fc98; const P1: u64 = 0x142645790; const P2: u64 = 0x1426455a8; const P3: u64 = 0x1411da620; const P4: u64 = 0x1411da6a0; const P5: u64 = 0x1403497a0; const P6: u64 = 0x142645598; const P7: u64 = 0x1401dec00; const P8: u64 = 0x14183acc8; const P9: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbp = rcx; r15 = r8.wrapping_add(0x3f); r15 = (((r15) & (0xffffffffffffffc0u64))) as u64; fl_l = r15; fl_r = (0x100000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rdx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rdx.wrapping_add(0x20u64), (xmm0) as u128); bb = 22; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdx = rcx.wrapping_add(0x20); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r13 = 0; fl_l = 0; fl_r = 0; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; fl_l = rcx; fl_r = (0x100000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 21; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdx, r13); rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 6; continue; }
            6 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 7; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = rax; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), r12); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x1cu64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r13 as u32)) as u32); bb = 11; continue; }
            11 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x20u64), (1) as u32); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x28u64), (1) as u64)) as u64; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((r13 as u32)) as u32); bb = 15; continue; }
            15 => { rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rax as u32)) as u32); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { r13 = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; rdi = ((r13.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x1cu64))) as u32 as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rax = ((rcx.wrapping_mul(8)) as u64) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = r13.wrapping_add(0x1); rbx = ((rcx as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rax; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbx }) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = rax; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rbx as u32)) as u32); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(r12), rax); crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x18u64), (1) as u32); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rdx = rbp.wrapping_add(0x20); r13 = 0; fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(rsi, r13); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), r13); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r13); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), r13); let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdx))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x2cu64), ((r15 as u32)) as u32); crate::simd_ops::lk_add_u64(rdx, (r15) as u64); fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x28u64), (1) as u64)) as u64; bb = 22; continue; }
            22 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411da620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x142537280; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x18; const P7: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rbx as u32)) as u32 as u64; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P6), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbx); crate::simd_ops::wu64(rax.wrapping_add(P7), rbx); crate::simd_ops::wu64(rax.wrapping_add(P6), rbx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411da6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fc80; const P1: u64 = 0x1433dc7e8; const P2: u64 = 0x14205d410; const P3: u64 = 0x14397fcb8; const P4: u64 = 0x142645790; const P5: u64 = 0x1426455a8; const P6: u64 = 0x142093aa0; const P7: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x37); rsp = ((rsp).wrapping_sub(0xf0u64)) as u64; rsi = rcx; rbx = 0x1; crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rbx as u32)) as u32); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (0x100000u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x1du64), (0x10001u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x15u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0xdu64), rbx); let y_addr: u64 = rbp.wrapping_sub(0x39u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x1u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x19u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x1fu64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), (0x2u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x71u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x6du64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x69u64), ((rbx as u32)) as u32); rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp.wrapping_add(0x67); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); rdx = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xac3u32) as u32); r9 = rbp.wrapping_sub(0x1); r8 = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_sub(0x79); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(rsi, rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = rsi.wrapping_add(0x10); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), (0x1688u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rbx); crate::simd_ops::wu32(rbp.wrapping_sub(0x61u64), (0x27u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5du64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), (0x40000u64) as u64); r14 = (crate::simd_ops::ru64(P3)) as u64; rbx = r14.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 8; continue; }
            8 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); rdi = r14.wrapping_add(0xc0); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x44u64))) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((r15 as u32)) as u32); bb = 13; continue; }
            13 => { rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0xcu64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = rbx; r8 = rbp.wrapping_sub(0x61); rdx = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x90u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rsp = ((rsp).wrapping_add(0xf0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411da8a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fc38; const P1: u64 = 0x142645790; const P2: u64 = 0x8; const P3: u64 = 0x1426455a8; const P4: u64 = 0xc; const P5: u64 = 0x20; const P6: u64 = 0x24; const P7: u64 = 0x10; const P8: u64 = 0x1411daad0; const P9: u64 = 0x1c; const P10: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rdi as u32)) as u32); bb = 3; continue; }
            3 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P4), (1) as u32); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P9), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rbx.wrapping_add(P9), (0x0u32) as u32); bb = 7; continue; }
            7 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (1) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P4), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 10; continue; }
            10 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411da950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fc38; const P1: u64 = 0x142645790; const P2: u64 = 0x1426455a8; const P3: u64 = 0x1411daa40; const P4: u64 = 0x1411daad0; const P5: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 2; continue; }
            2 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0xcu64), (1) as u32); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rsi); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x1cu64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x0u32) as u32); bb = 7; continue; }
            7 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x20u64), (1) as u32); fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x28u64), (1) as u64)) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (r9) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdi = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), (0x0u32) as u32); bb = 11; continue; }
            11 => { rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rax as u32)) as u32); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = 0xffffffff; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(0x8u64), ((rdx as u32)) as u32); rdx = (tmp_xchg) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411daa40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1d8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142535070; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x142097120;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = rcx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x1d0; r8 = 0x10; rcx = r9; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dacd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433dc878; const P1: u64 = 0x28; const P2: u64 = 0x30; const P3: u64 = 0x8; const P4: u64 = 0x142442de0; const P5: u64 = 0x140ec18c0; const P6: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = ((rdx as u32)) as u32 as u64; r14 = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rcx = ((r14.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = 0x60; rcx = r14; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dade0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x142535070; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0;
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
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx, rax);
    rax = rdi;
    crate::simd_ops::wu64(rdi, rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dae50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 4>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); r10 = 0; fl_l = 0; fl_r = 0; r14 = r8; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r10); r11 = rdx; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r10); rbx = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), r10); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r10); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x24u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rdi = 0xd; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = 0x6; rdi = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rdi as u32) }) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rsi).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 4; continue; }
            3 => { rsi = r10; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r12 = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbp = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = ((rbp).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 7; continue; }
            6 => { rbp = r10; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; r15 = 0x39; rcx = (crate::simd_ops::ru32(r11.wrapping_add(0x1cu64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; crate::simd_ops::wu64(r8, r10); r8 = 0x2a; rax = ((r8 as u32)) as u32 as u64; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (r15 as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rdi as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0x14u64), ((r13 as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(r14.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(r14.wrapping_add(0x28u64), r12); rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x50u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r14); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(r11.wrapping_add(0x60u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x58u64))) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x68u64))) as u64; r8 = (if (((fl_l as u32)) == ((fl_r as u32))) { (r15 as u32) } else { (r8 as u32) }) as u32 as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdx))) as u64; rdx = (crate::simd_ops::ru32(r11.wrapping_add(0x64u64))) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { r10 = (crate::simd_ops::ru64(r11.wrapping_add(0x70u64))) as u64; fl_l = ((r10).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 11; continue; }
            10 => { rdx = ((r10 as u32)) as u32 as u64; r8 = ((r10 as u32)) as u32 as u64; rcx = r10; rax = r10; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(r9, rax); rax = rbx; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), rcx); crate::simd_ops::wu32(r9.wrapping_add(0x10u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x14u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(r9.wrapping_add(0x18u64), r10); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r9); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x24u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rax = 0x6; rdi = 0xd; crate::simd_ops::wu32(rcx, ((r10 as u32)) as u32); rdi = (if (((fl_l as u8)) == ((fl_r as u8))) { (rax as u32) } else { (rdi as u32) }) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 16; continue; }
            15 => { rcx = r10; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; r9 = (crate::simd_ops::ru32(r11.wrapping_add(0x20u64))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { r10 = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; fl_l = ((r10).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(r11.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x2; r8 = 0x2a; rax = (crate::simd_ops::ru32(r11.wrapping_add(0x18u64))) as u32 as u64; r15 = 0x39; r8 = (if (((fl_l as u32)) == ((fl_r as u32))) { (r15 as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = rbx; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), r10); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rdx); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            19 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rcx, (0x1u32) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { r10 = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = ((r10).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r10); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rcx); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411db050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1418349c0; const P1: u64 = 0x143857f30; const P2: u64 = 0x142645790; const P3: u64 = 0x8; const P4: u64 = 0x1426455a8; const P5: u64 = 0xc; const P6: u64 = 0x14397fc68; const P7: u64 = 0x18; const P8: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = rcx; r14 = 0x70; rsi = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { r15 = 0x6300; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); fl_l = (crate::simd_ops::ru64(r15.wrapping_add(rsi))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = 0x6318; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rsi))) as u64; crate::simd_ops::wu64(rdi, rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { rbx = P1; crate::simd_ops::wu64(rdi, rbx); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rbp as u32)) as u32); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (1) as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(rsi))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(rsi))) as u64; bb = 18; continue; }
            17 => { rax = (crate::simd_ops::ru64(P6)) as u64; bb = 18; continue; }
            18 => { crate::simd_ops::lk_add_u64(rax.wrapping_add(P7), (1) as u64); rax = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P3), rax); rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411db190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14209d670; const P1: u64 = 0x58; const P2: u64 = 0x1411e7fc0; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411db1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14209d670; const P1: u64 = 0x58; const P2: u64 = 0x1411e7fc0; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411db210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffb0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x8; const P8: u64 = 0x20; const P9: u64 = 0x28; const P10: u64 = 0x30; const P11: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x60; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rdi; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdx, rdi); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx.wrapping_add(P7), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P1), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P2), rax); crate::simd_ops::wu64(rdx.wrapping_add(P8), rax); crate::simd_ops::wu64(rdx.wrapping_add(P9), rax); crate::simd_ops::wu64(rdx.wrapping_add(P10), rax); crate::simd_ops::wu64(rdx.wrapping_add(P11), rax); rax = rbx; crate::simd_ops::wu64(rdx.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rdx.wrapping_add(0x48u64), rdi); crate::simd_ops::wu32(rdx.wrapping_add(0x50u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411db2d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1411db640; const P3: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = (0x4924924924924925u64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; r8 = (((rdx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = rbx; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; r8 = (((rdx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411db600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14209d670; const P1: u64 = 0x58; const P2: u64 = 0x1411e7fc0; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411db640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14183acc8; const P5: u64 = 0x1401de830;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rsi = rdx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; rdx = (((rdx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; r10 = 0x6348; rbx = rcx; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
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
            16 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = rdi; rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbx, rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411db890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1425371a0; const P1: u64 = 0x142533200; const P2: u64 = 0x1430782a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 1; continue; }
            1 => { r15 = 0; fl_l = 0; fl_r = 0; rdi = ((r15 as u32)) as u32 as u64; r14 = ((r15 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbp = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(r14).wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(r14).wrapping_add(0x10u64), r15); bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(1)) as u64; r14 = ((r14).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rcx = r9.wrapping_sub(0x1); rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rcx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rbx.wrapping_add(0x8); bb = 15; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rsi = rbx.wrapping_add(0x8); bb = 15; continue; }
            14 => { r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rax = P2; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rsi, r15); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r15); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411db9b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x40; const P5: u64 = 0x60; const P6: u64 = 0x80; const P7: u64 = 0xffffffffffffff80; const P8: u64 = 0x142535070; const P9: u64 = 0x1426456c8; const P10: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rsi = rcx; rcx = 0x6348; rdi = r8; rbx = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { let y_addr: u64 = rdi; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdi.wrapping_add(P3); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(P3); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = rdi.wrapping_add(P4); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(P4); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdi.wrapping_add(P5); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(P5); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(P6))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbx.wrapping_add(P6), (xmm0) as u128); crate::simd_ops::wu64(rsi, rbx); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = 0x90; r8 = ((rdx.wrapping_add(P7)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dba80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14183acc8; const P5: u64 = 0x1401de830;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; fl_l = rsi; fl_r = rdx; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rsi }) as u64; rdi = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
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
            16 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = rdi; rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbx, rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dbbc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14183acc8; const P5: u64 = 0x1401de830;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; fl_l = rsi; fl_r = rdx; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rsi }) as u64; rdi = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rsi.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
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
            16 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = rdi; rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbx, rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dbff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428770; const P1: u64 = 0x1434286b0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = rdx; rax = rdx.wrapping_add(0x1); rax = ((rax).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm1 = (crate::simd_ops::ru128(P0)) as u128; xmm2 = (crate::simd_ops::ru128(P1)) as u128; bb = 2; continue; }
            2 => { xmm0 = crate::simd_ops::vpshufb(xmm2, crate::simd_ops::ru128(rcx)); xmm0 = (xmm0) | (xmm1); crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu8(r8.wrapping_add(rdx), (0xffu8) as u8); r12 = 0; fl_l = 0; fl_r = 0; rsi = ((r12 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 4; continue; }
            4 => { r14 = ((r12 as u32)) as u32 as u64; rbp = r12.wrapping_sub(0x10); rdi = (0xde5fb9d2630458e9u64) as u64; r15 = (0xcbf29ce484222325u64) as u64; r9 = (0x100000001b3u64) as u64; bb = 5; continue; }
            5 => { r10 = (crate::simd_ops::ru64(rbx)) as u64; r11 = ((r10.wrapping_add(rsi)) as u64) as u64; fl_l = (crate::simd_ops::ru8(r11)) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rdx = (((rdx) ^ (r15))) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u32) as u32 as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u32) as u32 as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x5u64))) as u32) as u32 as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x6u64))) as u32) as u32 as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x7u64))) as u32) as u32 as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = rdi; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; rcx = r9; rcx = ((rcx).wrapping_shr(7u32)) as u64; rdx = r12; rcx = (((rcx) & (r8))) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rcx.wrapping_add(r10))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r8))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rcx.wrapping_add(r10))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((r12 as u32)) as u32); let bit_in: u32 = (rax as u32); if bit_in != 0 { rdi = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; rdi = (((rdi) & (r8))) as u64; rax = r9; rax = ((rax).wrapping_shr(7u32)) as u64; rdx = r8; rdx = (((rdx) & (rax))) as u64; r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & (0x7fu8))) as u8 as u64)); rcx = rsi; rcx = ((rcx).wrapping_sub(rdx)) as u64; rax = rdi; rax = ((rax).wrapping_sub(rdx)) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) & (r8))) as u64; fl_l = rcx; fl_r = (0xfffffffffffffff0u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu8(r11, ((r9 as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = rbp; rcx = (((rcx) & (rax))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r9 as u8)) as u8); bb = 19; continue; }
            11 => { rax = ((rdi.wrapping_add(r10)) as u64) as u64; rcx = rdi.wrapping_sub(0x10); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; crate::simd_ops::wu8(rax, ((r9 as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rcx = (((rcx) & (rax))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r9 as u8)) as u8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r15 = ((r14.wrapping_add(rcx)) as u64) as u64; rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rdx, rax); rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(0x8u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r15.wrapping_add(0x10u64), r12); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rsi.wrapping_add(rax), (0x80u8) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = rbp; rcx = (((rcx) & (rax))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), (0x80u8) as u8); bb = 18; continue; }
            15 => { rcx = (((rcx) & (rax))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r9 as u8)) as u8); r15 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r15 = ((r15).wrapping_add(r14)) as u64; r12 = (crate::simd_ops::ru64(r15)) as u64; r13 = ((crate::simd_ops::ru8(r15.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), rdx); rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r15 = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = ((r15.wrapping_add(rax)) as u64) as u64; rcx = ((r14.wrapping_add(rax)) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rcx, rax); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rdx); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(rax), r12); crate::simd_ops::wu8(r15.wrapping_add(rax).wrapping_add(0x8u64), ((r13 as u8)) as u8); r12 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(rax).wrapping_add(0x10u64), rcx); rsi = ((rsi).wrapping_sub(1)) as u64; rbp = ((rbp).wrapping_sub(1)) as u64; fl_l = r14; fl_r = (0x18u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 18; continue; }
            18 => { r15 = (0xcbf29ce484222325u64) as u64; bb = 19; continue; }
            19 => { rdi = (0xde5fb9d2630458e9u64) as u64; r9 = (0x100000001b3u64) as u64; bb = 20; continue; }
            20 => { rsi = ((rsi).wrapping_add(1)) as u64; rbp = ((rbp).wrapping_add(1)) as u64; r14 = ((r14).wrapping_add(0x18u64)) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rsi; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 21; continue; }
            21 => { rax = r8; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r8); r11 = rsp.wrapping_add(0x40); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dc340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14183acc2; const P5: u64 = 0x1425371a0; const P6: u64 = 0x142533200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rdx; rsi = rcx; r14 = (crate::simd_ops::ru64(rcx)) as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rdi = rdx.wrapping_add(0x18); rdi = (((rdi) & (0xfffffffffffffff8u64))) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rax.wrapping_mul(8).wrapping_add(0x7u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; r12 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            4 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = r12; bb = 12; continue; }
            6 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rax = rcx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 12; continue; }
            10 => { r8 = 0x10; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = rax; bb = 12; continue; }
            12 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r12; bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsi, rcx); rax = ((rdi.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); r8 = rbx.wrapping_add(0x10); rdx = 0xffffff80; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(rax), (0xffu8) as u8); rax = rbx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = rbx; rcx = ((rcx).wrapping_sub(rax)) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbx); rdi = r12; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 15; continue; }
            15 => { rbx = r15.wrapping_add(0x10); rax = (0xcbf29ce484222325u64) as u64; r15 = (0x100000001b3u64) as u64; r13 = (0xde5fb9d2630458e9u64) as u64; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 23; continue; } bb = 17; continue; }
            17 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_sub(0x10u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_sub(0xfu64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_sub(0xeu64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_sub(0xdu64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_sub(0xcu64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_sub(0xbu64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_sub(0xau64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_sub(0x9u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rax = r13; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r11 = ((rdx.wrapping_add(rax)) as u64) as u64; r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rcx = r11; rcx = ((rcx).wrapping_shr(7u32)) as u64; rdx = r12; rcx = (((rcx) & (r9))) as u64; r10 = (crate::simd_ops::ru64(rsi)) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rcx.wrapping_add(r10))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r9))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rcx.wrapping_add(r10))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { r8 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; r8 = ((r8).wrapping_add(rcx)) as u64; r8 = (((r8) & (r9))) as u64; r11 = (r11 & !0xffu64) | ((((((r11 as u8)) & (0x7fu8))) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(r10), ((r11 as u8)) as u8); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rcx = r8.wrapping_sub(0x10); rcx = (((rcx) & (rax))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r11 as u8)) as u8); rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rdx, rax); rax = ((crate::simd_ops::ru8(rbx.wrapping_sub(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(0x8u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, r12); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rbx, r12); rax = (0xcbf29ce484222325u64) as u64; bb = 23; continue; }
            23 => { rdi = ((rdi).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 24; continue; }
            24 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 25; continue; }
            25 => { rcx = r14.wrapping_sub(0x1); rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r14; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rcx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r14, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r14); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { r9 = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dc6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142068530; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411dc6b0, funcinfo_rva: 0x367f010, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (r9) as u64; r14 = (r8) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0xb0u64))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(0x10u64), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r10 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x1u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r10 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r8 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dc77e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411dc77e, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1411dc6b0) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = (0xa8u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dc7ab); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dc7bc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dc7ca); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dc7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142068530; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411dc7e0, funcinfo_rva: 0x367f010, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (r9) as u64; r14 = (r8) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0xb0u64))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(0x10u64), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r10 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x2u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r10 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r8 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dc8ae); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411dc8ae, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1411dc7e0) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = (0xa8u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dc8db); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dc8ec); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dc8fa); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dc910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142068530; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411dc910, funcinfo_rva: 0x367f010, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (r9) as u64; r14 = (r8) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0xb0u64))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(0x10u64), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r10 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r9 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x3u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r10 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r8 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dc9de); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411dc9de, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1411dc910) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = (0xa8u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dca0b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dca1c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dca2a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dca40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142068530; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411dca40, funcinfo_rva: 0x367f04c, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (r8) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r9 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x4u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r9 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r8 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); r9 = (crate::simd_ops::ru64(rax)) as u64; r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (rsi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dcb00); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411dcb00, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1411dca40) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = (0xa8u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dcb26); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dcb37); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dcb45); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dcb50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1433dc9a0; const P4: u64 = 0x142068530; const P5: u64 = 0x142535070; const P6: u64 = 0x1426456c8; const P7: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411dcb50, funcinfo_rva: 0x367f088, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (r8) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), (rax) as u64); bb = 3; continue; }
            3 => { rbx = (rdx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r9 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r8 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x4u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r9 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r8 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); r9 = ((P3) as u64) as u64; r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rdx = (rsi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dcc0c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1411dcc0c, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1411dcb50) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi, (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rdx = (0xa8u32) as u32 as u64; r8 = (0x10u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dcc32); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dcc43); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411dcc51); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dce60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142417470; const P1: u64 = 0x142060610; const P2: u64 = 0x143a49130; const P3: u64 = 0x14205f4e0; const P4: u64 = 0x1411dad70; const P5: u64 = 0x1411de8a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = r8; rbp = rdx; rsi = rcx; rbx = rcx.wrapping_add(0x58); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x64u64) as u64); bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rbx, ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((tmp_xchg) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 9; continue; }
            9 => { r9 = 0x10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rbx = P2; rcx = ((rcx).wrapping_add(rbx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rbx = ((crate::simd_ops::ru8(rcx.wrapping_add(rbx).wrapping_add(0x40u64))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r14 = rax; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rax, rdi); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbp); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64), r14); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x30u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x28u64)))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu8(rcx, (0x0u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 12; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dcfb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14209da50; const P1: u64 = 0x14209da80; const P2: u64 = 0x141834fc0; const P3: u64 = 0x14397fc80; const P4: u64 = 0x1433dcab8; const P5: u64 = 0x14205d410; const P6: u64 = 0x1411dd200; const P7: u64 = 0x1425371a0; const P8: u64 = 0x142533200; const P9: u64 = 0x1433dca60; const P10: u64 = 0x1433dca28;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P1; rdx = 0x60; r8 = ((rdx.wrapping_sub(0x5eu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); r9 = P4; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdi as u32)) as u32); r9 = rsp.wrapping_add(0x50); r8 = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), rax); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { r10 = r9.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            8 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            9 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); r9 = P4; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdi as u32)) as u32); r9 = rsp.wrapping_add(0x50); r8 = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rax); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { r10 = r9.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            19 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            20 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dd200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x142537280; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = r9; rdx = 0x6348; rdi = r8; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P0), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rdi)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(P5), rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dd280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142076db0; const P1: u64 = 0xc8; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x70; const P5: u64 = 0x7785fffb; const P6: u64 = 0x14205d410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = rdx; rcx = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rax.wrapping_add(P5)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dd2e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140208fd0; const P1: u64 = 0x141834ee8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = P0;
    rdx = 0x120;
    r8 = 0xc;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dd370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x142056c10; const P4: u64 = 0x1433dc950; const P5: u64 = 0x1403496f0; const P6: u64 = 0x1411db850; const P7: u64 = 0x141834fc0; const P8: u64 = 0x14397fc58; const P9: u64 = 0x1411db770;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(0xc0u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0xb8; r8 = 0x10; rcx = r9; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = P4; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), (0x0u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); rcx = rbx.wrapping_add(0x18); rsi = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P6; rdx = 0x18; r8 = ((rdx.wrapping_sub(0x16u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P6; rdx = 0x18; r8 = ((rdx.wrapping_sub(0x16u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r9 = P6; rdx = 0x18; r8 = ((rdx.wrapping_sub(0x16u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P8)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x0u32) as u32); r9 = rsp.wrapping_add(0x30); r8 = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x1u32) as u32); r9 = rsp.wrapping_add(0x30); r8 = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), rax); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dd4f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142535070; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x1430782a0; const P7: u64 = 0x8; const P8: u64 = 0x20; const P9: u64 = 0x28; const P10: u64 = 0x30; const P11: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0xb0; r8 = 0x10; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rcx, rbx); rax = P6; crate::simd_ops::wu64(rcx.wrapping_add(P7), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P1), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P2), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P8), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P9), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P10), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P11), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), rax); rax = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), rbx); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dd5d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142535070; const P4: u64 = 0x1426456c8; const P5: u64 = 0x1426453a0; const P6: u64 = 0x1411dcfb0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = rcx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0xd0; r8 = 0x10; rcx = r9; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dd660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x142072070;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    r9 = 0x6348;
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rdx = 0x3be8;
    r8 = 0x10;
    rcx = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dd6c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de830; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x1411defd0; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64);
    rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dd730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdx = 0x6348; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x18; rcx = rax; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rcx, rbx); rax = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P1), rbx); crate::simd_ops::wu64(rcx.wrapping_add(P7), rbx); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dd7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffff80; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x142072760;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = r9; rdi = r8; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = rcx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x90; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rcx = r10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r9 = (crate::simd_ops::ru32(r9)) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dd860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411ddc40; const P1: u64 = 0x14205dec0; const P2: u64 = 0x14183acbc; const P3: u64 = 0x14205de40; const P4: u64 = 0x142075bc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x98u64)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbp = r8; r12 = rdx; rbx = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rsi); rsi = rcx.wrapping_add(0x8); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdi); rdi = 0x100000; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); r15 = rcx.wrapping_add(0x10); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x74u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r15 = rbx.wrapping_add(0x10); rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; rsi = rbx.wrapping_add(0x8); fl_l = rdx; fl_r = rdi; rcx = rsp.wrapping_add(0x30); r8 = 0x1; rdx = (if (((fl_l as u64)) > ((fl_r as u64))) { rdi } else { rdx }) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(0x40u64); xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(0x18u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbx.wrapping_add(0x38u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = rbx.wrapping_add(0x58u64); crate::simd_ops::wu128(y_addr, xmm2); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm2_hi); crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), (0x0u64) as u64); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x74u64))) as u32 as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdi = ((rdx as u32)) as u32 as u64; rax = rbp; rdi = ((rdi).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_sub(r14)) as u64; fl_l = rax; fl_r = rdi; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x68u64)))) as u64; rdx = ((r14.wrapping_add(r12)) as u64) as u64; r8 = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x74u64))) as u32 as u64; bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x78u64), (rdi) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_l = ((r14).wrapping_add(rdi)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rax = ((rdx as u32)) as u32 as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(r15)) as u64; rdi = ((rdi).wrapping_sub(crate::simd_ops::ru64(rsi))) as u64; fl_l = rcx; fl_r = rdi; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rdi }) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x70u64))) as u32 as u64; r8 = rbx.wrapping_add(0x18); rdx = (crate::simd_ops::ru64(rsi)) as u64; rax = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x74u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u64(rsi, (rdi) as u64)) as u64; fl_r = fl_l; bb = 13; continue; }
            12 => { r15 = rbx.wrapping_add(0x10); rsi = rbx.wrapping_add(0x8); bb = 13; continue; }
            13 => { rdi = 0x100000; fl_l = r14; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 14; continue; }
            14 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 15; continue; }
            15 => { rsp = ((rsp).wrapping_add(0x98u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dd9e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x1411ddc40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) <= ((fl_r as u64)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx, (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dda10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0xc8; const P4: u64 = 0x142442de0; const P5: u64 = 0x14209da50; const P6: u64 = 0xffffffffffffffa2; const P7: u64 = 0x141834ee8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { r9 = P5; rdx = 0x60; r8 = ((rdx.wrapping_add(P6)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dda90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x14243b540; const P2: u64 = 0x88; const P3: u64 = 0x1401de830; const P4: u64 = 0x78; const P5: u64 = 0x1411d2b40; const P6: u64 = 0x68; const P7: u64 = 0x48; const P8: u64 = 0x1402c30b0; const P9: u64 = 0x38; const P10: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ddb00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x14243b540; const P2: u64 = 0x88; const P3: u64 = 0x1401de830; const P4: u64 = 0x78; const P5: u64 = 0x1411d2b40; const P6: u64 = 0x68; const P7: u64 = 0x48; const P8: u64 = 0x1402c30b0; const P9: u64 = 0x38; const P10: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ddc40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x78; const P3: u64 = 0x14205de40; const P4: u64 = 0x70; const P5: u64 = 0x18; const P6: u64 = 0x74; const P7: u64 = 0x142075bc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rcx;
    rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_r = rbx;
    rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { crate::simd_ops::ru64(rcx.wrapping_add(P2)) } else { rbx }) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    r9 = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64;
    r8 = ((rdi.wrapping_add(P5)) as u64) as u64;
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(rdi)) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx);
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_add_u64(rdi.wrapping_add(P1), (rbx) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ddca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x142535070; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x142056ed0; const P6: u64 = 0x1433dd578; const P7: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rsi = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rbx.wrapping_add(P0)) as u32) as u32 as u64; r8 = ((rbx.wrapping_add(P1)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rsi))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            2 => { rbx = rax; bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = P6; rcx = 0x63d0; rcx = ((rcx).wrapping_add(rsi)) as u64; crate::simd_ops::wu64(rbx, rax); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x40000; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = rdi; crate::simd_ops::wu64(rdi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dddd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x142085dc0; const P4: u64 = 0x142085c40; const P5: u64 = 0x1411dfda0; const P6: u64 = 0x14397fc80; const P7: u64 = 0x14205d410; const P8: u64 = 0x1433dc7e8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x110u64)) as u64; rdi = rcx; r15 = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0x28; r8 = ((rdx.wrapping_sub(0x18u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = r15; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x140u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x138u64), rbx); crate::simd_ops::wu64(rbx, (0x40000000u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x6400000u64) as u64); rax = 0xc0000000; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rax = 0x100; crate::simd_ops::wu16(rsp.wrapping_add(0x60u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x400u64) as u64); r8 = rsp.wrapping_add(0x40); rcx = rbx.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu16(rbx.wrapping_add(0x20u64), (0x0u16) as u16); rcx = (crate::simd_ops::ru64(P6)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = 0x3c; r8 = rsp.wrapping_add(0xd0); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_add(0x7785fffbu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xdcu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0x1u8) as u8); bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu8(rbx.wrapping_add(0x21u64), (0x1u8) as u8); bb = 12; continue; }
            12 => { rax = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x84u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x1000000u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x5cu64), (0x10001u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x64u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x6cu64), rax); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x98u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xb8u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), ((xmm0 as u64)) as u64); r10 = (crate::simd_ops::ru64(P6)) as u64; rax = (crate::simd_ops::ru64(r10)) as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(0xd8u64))) as u64; rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rsi = ((rsi).wrapping_add(0x1a8u64)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(P6)) as u64; bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsi, r15); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rax = P8; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x400u32) as u32); r9 = rsp.wrapping_add(0x98); r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x78); rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (((rax as u32)).wrapping_add(0x7785fffbu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rdi, rbx); rax = rdi; r11 = rsp.wrapping_add(0x110); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411de040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf0; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1420528f0; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = r9; r14 = r8; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r11 = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r9 = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64; r8 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rdx = (((crate::simd_ops::ru32(r14)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r10 = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r10 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0xe8; r8 = 0x10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411de130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x110; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142051f70; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = r9; r14 = r8; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; rax = rdx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { rbx = rdx; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r9 = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64; r8 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; rdx = (((crate::simd_ops::ru32(r14)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r10 = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r10 as u32)) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x108; r8 = 0x10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411de210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x14204e160; const P4: u64 = 0x14204e060; const P5: u64 = 0x14209e4d0; const P6: u64 = 0x14243b5f0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = rcx; rsi = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0xe0u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0xd8; r8 = 0x10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rsi; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rbx); crate::simd_ops::wu64(rbx, rsi); rcx = rbx.wrapping_add(0x8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rax = 0x200; crate::simd_ops::wu16(rsp.wrapping_add(0x68u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x100u64) as u64); rdx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), rsi); rcx = rbx.wrapping_add(0x98); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x8u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x80u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x10u64) as u64); r9 = 0x400; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbx); rax = rdi; r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411de360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141052dc0; const P1: u64 = 0x1411de5d0; const P2: u64 = 0x14307aa70; const P3: u64 = 0x1401df630; const P4: u64 = 0x1437fcf88; const P5: u64 = 0x14183acce;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = rdx; r8 = rdx; rdx = rsp.wrapping_add(0x30); rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { r8 = rsp.wrapping_add(0x20); rcx = rbx; rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { r10 = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rcx = (0xfffffffffffffffu64) as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rax = r10; rax = ((rax).wrapping_sub(r9)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdx = rbx.wrapping_add(0x58); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = r8; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = r9.wrapping_sub(0x1); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rax); bb = 9; continue; }
            8 => { rax = r10.wrapping_add(0x1); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; r8 = ((r8).wrapping_shl(4u32)) as u64; crate::simd_ops::wu128(rax.wrapping_add(r8), (xmm0) as u128); rax = (crate::simd_ops::ru64(rdx)) as u64; rax = ((rax).wrapping_add(r8)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { r10 = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rcx = (0xfffffffffffffffu64) as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rax = r10; rax = ((rax).wrapping_sub(r9)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdx = rbx.wrapping_add(0x58); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = r8; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = r9.wrapping_sub(0x1); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rax); bb = 15; continue; }
            14 => { rax = r10.wrapping_add(0x1); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; crate::simd_ops::wu128(r8.wrapping_add(rax), (xmm0) as u128); rax = (crate::simd_ops::ru64(rdx)) as u64; rax = ((rax).wrapping_add(r8)) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { rdx = P2; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P4; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1411de4e7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { rdx = P2; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P4; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1411de50a, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411de510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x138; const P1: u64 = 0x142075a20; const P2: u64 = 0xf0; const P3: u64 = 0x14243b3e0; const P4: u64 = 0x70; const P5: u64 = 0xa8; const P6: u64 = 0x14243b2e0; const P7: u64 = 0x142099c20; const P8: u64 = 0x14243b540; const P9: u64 = 0xb0; const P10: u64 = 0x30; const P11: u64 = 0x20; const P12: u64 = 0x1411d3de0; const P13: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = 0x1; rcx = (((rdx as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411de810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x14241c0a0; const P3: u64 = 0x14183acc8; const P4: u64 = 0x14241c190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = rdx; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rdi }) as u64; rdx = 0x8; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rsi = rax; rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = rax; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx, rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dea10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    r9 = rdx;
    rdx = (0x9ae16a3b2f904057u64) as u64;
    rax = ((r8.wrapping_mul(8).wrapping_add(P0)) as u64) as u64;
    rax = (((rax) ^ (r8))) as u64;
    rax = (((rax) as i64).wrapping_mul((rdx) as i64) as u64) as u64;
    rcx = rax;
    rcx = ((rcx).wrapping_shr(47u32)) as u64;
    rcx = (((rcx) ^ (rax))) as u64;
    rcx = (((rcx) ^ (r8))) as u64;
    r8 = (0x9ddfea08eb382d69u64) as u64;
    rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64;
    rax = rcx;
    rax = ((rax).wrapping_shr(47u32)) as u64;
    rax = (((rax) ^ (rcx))) as u64;
    rcx = (0x651e95c4d06fbfb1u64) as u64;
    rax = (((rax) as i64).wrapping_mul((rdx) as i64) as u64) as u64;
    rcx = ((rcx).wrapping_add(rax)) as u64;
    rcx = (((rcx) ^ (r9))) as u64;
    rcx = (((rcx) as i64).wrapping_mul((r8) as i64) as u64) as u64;
    rdx = rcx;
    rdx = ((rdx).wrapping_shr(47u32)) as u64;
    rdx = (((rdx) ^ (rcx))) as u64;
    rdx = (((rdx) ^ (r9))) as u64;
    rdx = (((rdx) as i64).wrapping_mul((r8) as i64) as u64) as u64;
    rax = rdx;
    rax = ((rax).wrapping_shr(47u32)) as u64;
    rax = (((rax) ^ (rdx))) as u64;
    rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dea90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x142537280; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x18; const P6: u64 = 0x8; const P7: u64 = 0x1c;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rbp = 0; fl_l = 0; fl_r = 0; rdi = r9; rsi = r8; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_add(0x20u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((rbp as u32)) as u32 as u64; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(r10.wrapping_add(P5), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r10.wrapping_add(P1), rax); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu64(rdx, rbp); crate::simd_ops::wu64(rdx.wrapping_add(P6), rbp); crate::simd_ops::wu64(rdx.wrapping_add(P1), rbp); crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rax as u32)) as u32); rax = rbx; crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411deb40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rdi = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P2), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rdi);
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx, rdi);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411deb90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397fc80; const P1: u64 = 0x1433dccb8; const P2: u64 = 0x14205d410; const P3: u64 = 0x1433dcab8; const P4: u64 = 0x1433dcca8; const P5: u64 = 0x1433dcc88; const P6: u64 = 0x1433dcc78; const P7: u64 = 0x1433dcc68;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rbx = rcx; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rbp); crate::simd_ops::wu8(rcx.wrapping_add(0x20u64), ((rbp as u8)) as u8); r10 = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(r10)) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P0)) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx, rbp); r9 = rbx; r8 = P1; rdx = ((rdi as u32)) as u32 as u64; rcx = r10; rax = crate::simd_ops::call_stk(rsi, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); r9 = P3; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x120u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rsi = rbx.wrapping_add(0x28); crate::simd_ops::wu64(rsi, rbp); rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsi, rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rdx = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); r9 = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((rdi as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x2; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 14; continue; }
            14 => { rdx = P5; bb = 17; continue; }
            15 => { rdx = P6; bb = 17; continue; }
            16 => { rdx = P7; bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), (0x1u8) as u8); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411ded20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x14205d410; const P2: u64 = 0x1411df270; const P3: u64 = 0x1411df170; const P4: u64 = 0x142645598;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = 0x887a0005; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { fl_l = rbp; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { bb = 9; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x8u64), (0x1u8) as u8); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rcx = (((crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rcx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rdx }) as u64; rcx = rdi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rsi = ((r15 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { r8 = rdi.wrapping_add(0x1c); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rdi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x10u64), (1) as u32); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64), (0x1u8) as u8); bb = 17; continue; }
            17 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = ((rax.wrapping_add(0x7785fffbu64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x8u64), (1) as u64); crate::simd_ops::wu8(rsi.wrapping_add(0x20u64), (0x0u8) as u8); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), r15); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411deee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rdi = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P2), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rdi);
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx, rdi);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411def40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x10; const P2: u64 = 0x30; const P3: u64 = 0x28; const P4: u64 = 0x8; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P2), rdi); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P1), rdi); rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx, rdi); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411df170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1411deb90; const P4: u64 = 0x1433dccc8; const P5: u64 = 0x1433dcc98;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = r8; rdi = rcx; r12 = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(0x40u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((r12 as u32)) as u32 as u64; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rdx = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r8 = rbx.wrapping_add(0x30); crate::simd_ops::wu64(r8, r12); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), r12); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r8, r12); rdx = P4; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; r15 = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), r12); r8 = rbx.wrapping_add(0x38); rdx = P5; rcx = r14; rax = crate::simd_ops::call_stk(r15, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbx); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411df270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x1411defd0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rsi = rdx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; r10 = 0x6348; rdi = rcx; rdx = ((rdx).wrapping_shl(4u32)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(r8))) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
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
            15 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdx.wrapping_add(0x10); crate::simd_ops::wu64(rcx, r14); crate::simd_ops::wu64(rdx.wrapping_sub(0x10u64), rax); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_sub(0x8u64), ((rax as u8)) as u8); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411df3d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411df430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x14206b020; const P4: u64 = 0x1425371a0; const P5: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411df430, funcinfo_rva: 0x367f744, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (r8) as u64; rdx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbx.wrapping_add(0x40u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411df474); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411df481); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411df48f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rbp as u32)) as u32 as u64; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (rax) as u64); bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rbx, (rbp) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (rbp) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), (rbp) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x3cu64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (rbp) as u64); rdi = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, (rsi) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411df4d1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = ((rdi.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rdi) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rdi) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411df531); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rdx = (rcx) as u64; r9 = (rdi) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411df549); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x14u64), ((rcx as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x1cu64), (0x1u8) as u8); rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411df590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411df630; const P1: u64 = 0xfffffffffffffffb; const P2: u64 = 0x141834ee8; const P3: u64 = 0x1411d2db0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    r9 = P0;
    rdx = 0x8;
    r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    r9 = P3;
    rdx = 0x8;
    r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411df5e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1401de830; const P2: u64 = 0x20; const P3: u64 = 0x18; const P4: u64 = 0x14209a1e0; const P5: u64 = 0x8; const P6: u64 = 0x10; const P7: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rbx.wrapping_add(P0)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P5), (0x0u64) as u64);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411df660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffc8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x1c; const P8: u64 = 0x40; const P9: u64 = 0x8; const P10: u64 = 0x20; const P11: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x48; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rdi; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdx.wrapping_add(P7), rdi); rax = rbx; crate::simd_ops::wu64(rdx.wrapping_add(P8), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P9), rdi); crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rdi as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(P10), rdi); crate::simd_ops::wu64(rdx, rdi); crate::simd_ops::wu64(rdx.wrapping_add(P1), rdi); crate::simd_ops::wu64(rdx.wrapping_add(P11), rdi); crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rdx.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411df740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14206b020; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x100; const P3: u64 = 0x108; const P4: u64 = 0x4; const P5: u64 = 0xe; const P6: u64 = 0x20; const P7: u64 = 0x18; const P8: u64 = 0x1425371a0; const P9: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rcx); crate::simd_ops::wu64(rdi, rax); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r8, rcx); rbx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rdx, rax); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r10 = ((rbx.wrapping_add(P1)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rbx; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(P3)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P6), rbx); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P7), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            5 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            6 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rdx = rcx; r9 = rbx; rcx = r10; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411df800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x100; const P2: u64 = 0x108; const P3: u64 = 0x4; const P4: u64 = 0xe; const P5: u64 = 0x20; const P6: u64 = 0x18; const P7: u64 = 0x1425371a0; const P8: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rcx); crate::simd_ops::wu64(rbx, rax); fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r8, rcx); r9 = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rdx, rax); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r9.wrapping_add(P0)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(P2)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P6), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411df8b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14206b020; const P1: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1411df8b0, funcinfo_rva: 0x3674058, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (r8) as u64; rbp = (rdx) as u64; rbx = (rcx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx, (r14) as u64); rsi = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411df8f4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1411df8fc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411df930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = r9; r10 = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(5u32)) as u64; rbx = r9; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
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
            14 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); r8 = rdi; bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = r8.wrapping_add(0x1a); bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rcx.wrapping_sub(0x1au64), rdi); crate::simd_ops::wu64(rcx.wrapping_sub(0x12u64), rdi); crate::simd_ops::wu64(rcx.wrapping_sub(0xau64), rdi); crate::simd_ops::wu64(rcx.wrapping_sub(0x2u64), rdi); rcx = rcx.wrapping_add(0x20); fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = r8; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dfa50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142055df0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dfa70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x8; const P4: u64 = 0x14397fc80; const P5: u64 = 0x1433dcab8; const P6: u64 = 0x120; const P7: u64 = 0x14205d410; const P8: u64 = 0x142645740; const P9: u64 = 0xfffffffffffffff8; const P10: u64 = 0x142535070; const P11: u64 = 0x1426456c8; const P12: u64 = 0x1426453a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rdx = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rbx, rdx); crate::simd_ops::wu64(rbx.wrapping_add(P3), rdx); crate::simd_ops::wu64(rbx.wrapping_add(P1), rdx); rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbx, rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r9 = P5; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8007000e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (((rax as u32)).wrapping_add(0x7785fffbu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rdx = 0x18; r8 = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; rcx = r9; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rbx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdx); bb = 4; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dfb80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c0; const P1: u64 = 0x1401de830; const P2: u64 = 0x1b0; const P3: u64 = 0x140f366d0; const P4: u64 = 0x1a8; const P5: u64 = 0x10; const P6: u64 = 0x198; const P7: u64 = 0x1411e05c0; const P8: u64 = 0x188; const P9: u64 = 0x160; const P10: u64 = 0x1402c30b0; const P11: u64 = 0x150; const P12: u64 = 0x14208a870; const P13: u64 = 0x14208b3d0; const P14: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P4), (0x0u64) as u64); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dfc30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1411e1210; const P3: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbx as u32)) as u32); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dfce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14040f460; const P3: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rbx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; fl_l = r8; fl_r = rsi; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
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
pub unsafe extern "win64" fn fn_1411dfda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142535070; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x14208aa90; const P4: u64 = 0x14209e4d0; const P5: u64 = 0x1401df790;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = r8; rdi = rcx; r14 = 0; fl_l = 0; fl_r = 0; rcx = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x250u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); bb = 5; continue; }
            3 => { rdx = 0x248; r8 = 0x10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = r14; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu64(rbx, r14); rdx = rbx.wrapping_add(0x8); crate::simd_ops::wu64(rdx, r14); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r14); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), r14); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), r14); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r14); rcx = rbx.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x120); rdx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu16(rbx.wrapping_add(0x180u64), ((r14 as u16)) as u16); crate::simd_ops::wu64(rbx.wrapping_add(0x188u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x190u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x198u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x1a0u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x1a8u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x1b0u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x1b8u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x1c0u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x1c8u64), r14); crate::simd_ops::wu8(rbx.wrapping_add(0x1d0u64), (0x0u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x1d4u64), (0x40000000u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x1d8u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x1e0u64), (0xffffffffffffffffu64) as u64); rcx = rbx.wrapping_add(0x1e8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x210u64), r14); crate::simd_ops::wu16(rbx.wrapping_add(0x218u64), (0x0u16) as u16); crate::simd_ops::wu64(rbx.wrapping_add(0x220u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x228u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x230u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x238u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x240u64), r14); crate::simd_ops::wu64(rdi, rbx); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1411dff20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142083150; const P1: u64 = 0x142084000; const P2: u64 = 0x1420850b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x60u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax); rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x28u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x28u64), (0x10u16) as u16); crate::simd_ops::wu16(rsp.wrapping_add(0x48u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rax); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x38u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x44u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x64u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x48u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0x2u8) as u8); rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xc8u64))) as u64; rax = rcx.wrapping_add(0x1); crate::simd_ops::wu64(rdx.wrapping_add(0xc8u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rcx); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x1cu64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x20u64), (1) as u32); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
