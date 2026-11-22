#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_14209ff70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1408746b0; const P1: u64 = 0x14183acc2; const P2: u64 = 0x1406dcc50; const P3: u64 = 0x14209f640; const P4: u64 = 0x14209f200; const P5: u64 = 0x142075970; const P6: u64 = 0x142075080; const P7: u64 = 0x14209d5c0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14209ff70, funcinfo_rva: 0x379bdf0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = (r9) as u64; rsi = (r8) as u64; rbp = (rcx) as u64; r10 = (crate::simd_ops::ru32(r9)) as u32 as u64; r10 = (((r10 as u32)).wrapping_add(0x1fu32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(5u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x250u64), ((r10 as u32)) as u32); r11 = (crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32 as u64; r11 = (((r11 as u32)).wrapping_add(0x1fu32)) as u32 as u64; r11 = (((r11 as u32)).wrapping_shr(5u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x254u64), ((r11 as u32)) as u32); r9 = ((r11 as u32)) as u32 as u64; r9 = ((((r9 as u32)) as i32).wrapping_mul(((r10 as u32)) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x258u64), ((r9 as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(2u32)) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x25eu64), ((rcx as u8)) as u8); rbx = ((r9 as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_shl(5u32)) as u32 as u64; r14 = ((r9.wrapping_mul(8)) as u32) as u32 as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x260u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), ((r15 as u32)) as u32); r8 = (crate::simd_ops::ru32(r8.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x264u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x27cu64), ((r8 as u32)) as u32); r8 = (((r8 as u32)).wrapping_add(crate::simd_ops::ru32(rsi.wrapping_add(0xa4u64)))) as u32 as u64; rax = ((rbx.wrapping_add(r14)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x280u64), ((r8 as u32)) as u32); rdx = ((r14.wrapping_add(0x1fu64)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; rdx = ((((rdx as u32)) & (0xffffffe0u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(crate::simd_ops::ru32(rsi.wrapping_add(0xa8u64)))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x26cu64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x284u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x270u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x288u64), ((r8 as u32)) as u32); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (((rdx as u32)).wrapping_add(0x1fu32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rbx as u32))) as u32 as u64; rdx = ((((rdx as u32)) & (0xffffffe0u32))) as u32 as u64; fl_l = (((r8 as u32)).wrapping_add(crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64)))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rbp.wrapping_add(0x274u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x28cu64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x290u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xa4u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x294u64), ((rax as u32)) as u32); r9 = (((r9 as u32)).wrapping_shl(4u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x298u64), ((r9 as u32)) as u32); rax = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0xbu32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x29cu64), ((rax as u32)) as u32); rax = ((r10.wrapping_mul(4).wrapping_sub(0x3u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2a0u64), ((rax as u32)) as u32); rcx = ((r11 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(4u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(0x10u32)) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((r10 as u32)) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2a4u64), ((rcx as u32)) as u32); rdx = ((r11.wrapping_mul(4).wrapping_sub(0x4u64)) as u32) as u32 as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul(((r10 as u32)) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2a8u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xb8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1fu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) >= ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((((rcx as u32)) | (0x2u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            4 => { rdx = ((((rdx as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2a8u64), ((rdx as u32)) as u32); fl_l = ((((rcx as u32)) | (0xau32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rbp.wrapping_add(0x2a4u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x1u32) as u32); rax = ((rdi.wrapping_add(0x134u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rcx = (if (((fl_l as u32)) >= ((fl_r as u32))) { rax } else { rcx }) as u64; r13 = (crate::simd_ops::ru32(rcx)) as u32 as u64; r12 = ((rsi.wrapping_add(0x58u64)) as u64) as u64; rbx = ((rbp.wrapping_add(0x2b8u64)) as u64) as u64; bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rcx = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_l = (r13) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r13) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { r13 } else { rdx }) as u64; rcx = ((rbx.wrapping_sub(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a0145); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r8 = (r13) as u64; fl_l = (r8) as u64 as u64; fl_r = (rsi) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 8; continue; }
            8 => { r8 = ((r8.wrapping_mul(8)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a0166); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 13; continue; }
            9 => { fl_l = (r13) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = (r13) as u64; fl_l = (r8) as u64 as u64; fl_r = (rsi) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { r8 = ((r8.wrapping_mul(8)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a0184); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 13; continue; }
            12 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rbx, ((r13 as u32)) as u32); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(r12, (rax) as u64); r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; r12 = ((r12).wrapping_add(0x8u64)) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x254u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x1fu32)) as u32 as u64; rdx = ((rdx).wrapping_shr(5u32)) as u64; rcx = ((rbp.wrapping_add(0x300u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a01c1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x300u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x88u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x250u64))) as u32 as u64; rdx = ((rax.wrapping_mul(4).wrapping_add(0x1fu64)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(5u32)) as u64; rcx = ((rbp.wrapping_add(0x310u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a01f4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x310u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x254u64))) as u32 as u64; rdx = ((rax.wrapping_mul(4).wrapping_add(0x1fu64)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shr(5u32)) as u64; rcx = ((rbp.wrapping_add(0x320u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a021f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x320u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x98u64), (rax) as u64); rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a0238); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xb8u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0xb4u64))) as u32 as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a024d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0xe0u64))) as u64; rcx = ((rbp.wrapping_add(0x390u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a0260); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x11cu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x110u64))) as u64; rcx = ((rbp.wrapping_add(0x390u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a028f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rbp.wrapping_add(0x3a0u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x398u64), (rsi) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x390u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x100u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x101u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x258u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rcx = ((rbp.wrapping_add(0x330u64)) as u64) as u64; r8 = (r8 & !0xffu64) | (((0x29u8) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a02e1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a0650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14205d410; const P1: u64 = 0x1411e9730; const P2: u64 = 0x14209d670; const P3: u64 = 0x1411e7fc0; const P4: u64 = 0x142442de0; const P5: u64 = 0x142075bc0; const P6: u64 = 0x1420734e0; const P7: u64 = 0x1411e9610; const P8: u64 = 0x142075af0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r12 = r8; rdi = rcx; r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r15 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r13 = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { r12 = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = 0x887a0005; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { fl_l = rsi; fl_r = r12; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) < ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); r15 = rax; bb = 9; continue; }
            9 => { rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rbx; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { r14 = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = 0x887a0005; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { fl_l = rsi; fl_r = r14; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) < ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 23; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rdi.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; let y_addr: u64 = rbx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x10u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rdx = rdx.wrapping_sub(0x5); rdx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), ((xmm1 as u64)) as u64); bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 24; continue; }
            23 => { fl_l = ((rbx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 25; continue; }
            25 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 26; continue; }
            26 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r14 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r13 = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 27; continue; }
            27 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 28; continue; }
            28 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r13); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = rdx.wrapping_sub(0x5); rdx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), ((xmm1 as u64)) as u64); bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; bb = 34; continue; }
            33 => { fl_l = ((rbx).wrapping_add(0x28u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdx = ((r8 as u32)) as u32 as u64; bb = 34; continue; }
            34 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = ((rdx as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((r9.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 35; continue; }
            35 => { rbx = r9; bb = 36; continue; }
            36 => { rcx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rax = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 40; continue; } bb = 37; continue; }
            37 => { let y_addr: u64 = rax.wrapping_sub(0x28u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = 0; fl_l = 0; fl_r = 0; r8 = r15; rdx = 0; fl_l = 0; fl_r = 0; xmm0 = (ymm1_hi) as u128; ymm0_hi = 0; rcx = ((xmm0 as u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x1; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r12); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64), ((xmm1 as u64)) as u64); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x20u64), (1) as u32); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 43; continue; }
            40 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rbx = r14.wrapping_add(0x3); rbx = (((rbx) & (0xfffffffffffffffcu64))) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x20u32) as u32); r9 = ((r13 as u32)) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), (0x1u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = 0; fl_l = 0; fl_r = 0; r8 = r15; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x1; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r12); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 42; continue; } bb = 41; continue; }
            41 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64), ((xmm1 as u64)) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 43; continue; }
            43 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a1a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411e9870; const P1: u64 = 0x14241c0a0; const P2: u64 = 0x1411e98e0; const P3: u64 = 0x1411e9ea0; const P4: u64 = 0x1420a0f60; const P5: u64 = 0x1420a0d70; const P6: u64 = 0x1420a0b40; const P7: u64 = 0x14241c190;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 63>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1420a1a60, funcinfo_rva: 0x379c104, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x1f8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0xf8u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x1b8u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x88u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x98u64), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xa8u64), (xmm11) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xb8u64), (xmm12) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xc8u64), (xmm13) as u128); r13 = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1abb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x110u64), (rax) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r12 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rsi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((rsi as u32)) as u32); rcx = ((rsi as u32)) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (rax) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1af3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((rbx as u32)) as u32); rcx = (rax) as u64; bb = 2; continue; }
            2 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (rbx) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rcx, (rsi) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rsi) as u64); rcx = ((rcx.wrapping_add(0x10u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rbx as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1b25); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x118u64), (rax) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); r14 = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rsi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((rsi as u32)) as u32); rcx = (rsi) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (rax) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = (0x8u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1b5e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((rdi as u32)) as u32); rcx = (rax) as u64; bb = 7; continue; }
            7 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (rdi) as u64; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rcx, (rsi) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (rsi) as u64); rcx = ((rcx.wrapping_add(0x10u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdi as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); r15 = (rsi) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rsi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), ((rsi as u32)) as u32); rax = (rsi) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = (rbx) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = (0x8u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1bbf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), ((rbx as u32)) as u32); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = (rbx) as u64; bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rax, (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rsi) as u64); rax = ((rax.wrapping_add(0x10u64)) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rbx as u32)) as u32); rsi = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x130u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rax = ((crate::simd_ops::ru16(rsi)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x108u64), ((rax as u16)) as u16); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r12)) as u64; rdx = ((rbp.wrapping_add(0x108u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1c51); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; rax = ((crate::simd_ops::ru16(rsi)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x108u64), ((rax as u16)) as u16); rdi = ((rbx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { rcx = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; rdx = ((rbp.wrapping_add(0x108u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1c91); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 22; continue; }
            22 => { rax = ((crate::simd_ops::ru16(rsi)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x108u64), ((rax as u16)) as u16); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0xc8u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xc0u64))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = ((rbp.wrapping_add(0x108u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1cd1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 24; continue; }
            24 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xf8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((rdi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 25; continue; }
            25 => { r13 = (rax) as u64; bb = 26; continue; }
            26 => { rax = ((crate::simd_ops::ru16(rsi)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x108u64), ((rax as u16)) as u16); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r14 = ((rbx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 27; continue; }
            27 => { bb = 28; continue; }
            28 => { rcx = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; rdx = ((rbp.wrapping_add(0x108u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1d31); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 29; continue; }
            29 => { rdi = ((rdi).wrapping_add(0x48u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 30; continue; }
            30 => { rsi = ((rsi).wrapping_add(0x130u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 31; continue; }
            31 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x100u64))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x110u64))) as u64; bb = 32; continue; }
            32 => { rdi = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x130u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(rdi)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 40; continue; } bb = 33; continue; }
            33 => { rcx = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (r12) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), (r12) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (r12) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), (r12) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x118u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), (rsi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), (r15) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (r15) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (r15) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (r15) as u64); fl_l = ((rcx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = ((r12.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x100u64), (rax) as u64); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; xmm10 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x70u64))) as u128; xmm11 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x60u64))) as u128; xmm12 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x50u64))) as u128; xmm13 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x110u64))) as u64; bb = 34; continue; }
            34 => { rbx = (rax) as u64; rbx = ((rbx).wrapping_sub(r12)) as u64; rbx = ((((rbx) as i64).wrapping_shr(4u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x108u64), (rbx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rbx) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; r8 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1e4c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rbx) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1e7c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp, (rbx) as u64); xmm0 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0xb0u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1eaf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), (rbx) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0xc0u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1ee2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rcx) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1f18); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rcx) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm9) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1f4e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm10) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = (rdi) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1f74); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm11) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = (rdi) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1f9a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm12) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0xb0u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = (rdi) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1fc3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm13) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0xc0u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = (rdi) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a1fec); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xf8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), (r15) as u64); r14 = ((rsi as u32)) as u32 as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(r15)) as u64; rax = (r14) as u64; rax = ((rax).wrapping_sub(r15)) as u64; fl_l = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64 as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (r15) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (r12) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x108u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), (r12) as u64); xmm6 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x60u64))) as u128; xmm7 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x70u64))) as u128; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x100u64))) as u64; bb = 36; continue; }
            36 => { rax = (r14) as u64; rax = ((rax).wrapping_sub(r15)) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x8u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; r8 = (rdi) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a2094); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; r8 = (rdi) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a20ba); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rsi) as u64; rax = ((rax).wrapping_sub(r12)) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x8u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; r8 = (rdi) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a20f3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; r8 = (rdi) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a2119); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x48u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 37; continue; }
            37 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x110u64))) as u64; bb = 38; continue; }
            38 => { rdi = ((rdi).wrapping_add(0x130u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 39; continue; }
            39 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 40; continue; }
            40 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x110u64))) as u64; r14 = ((rdi as u32)) as u32 as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(r15)) as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (r15) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 45; continue; } bb = 41; continue; }
            41 => { rbx = (r15) as u64; bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a2178); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 44; continue; }
            44 => { crate::simd_ops::wu64(rbx, (r13) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (r13) as u64); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 42; continue; } bb = 45; continue; }
            45 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a2195); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 47; continue; }
            47 => { r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x118u64))) as u32 as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(rsi)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 52; continue; } bb = 48; continue; }
            48 => { rbx = (rsi) as u64; bb = 49; continue; }
            49 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 50; continue; }
            50 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a21b9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 51; continue; }
            51 => { crate::simd_ops::wu64(rbx, (r13) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 49; continue; } bb = 52; continue; }
            52 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 53; continue; }
            53 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a21da); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 54; continue; }
            54 => { rdi = ((rdi as u32)) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(r12)) as u64; fl_l = (r12) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 59; continue; } bb = 55; continue; }
            55 => { rbx = (r12) as u64; bb = 56; continue; }
            56 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 58; continue; } bb = 57; continue; }
            57 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a21f9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 58; continue; }
            58 => { crate::simd_ops::wu64(rbx, (r13) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 56; continue; } bb = 59; continue; }
            59 => { fl_l = (r12) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 61; continue; } bb = 60; continue; }
            60 => { rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a221a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 61; continue; }
            61 => { r11 = ((rsp.wrapping_add(0x1b8u64)) as u64) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x18u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x28u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x38u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x48u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x58u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x68u64))) as u128; xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x78u64))) as u128; xmm13 = (crate::simd_ops::ru128(r11.wrapping_sub(0x88u64))) as u128; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a25b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140fb3a60; const P1: u64 = 0x1420a2480; const P2: u64 = 0x140215610; const P3: u64 = 0x14241c190;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r13 = rdx; rsi = rcx; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), r14); r10 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), r14); r15 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), r14); r12 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r14); xmm1 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0xfu64), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rax = ((xmm1 as u64)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r9 = ((rax.wrapping_mul(4)) as u64) as u64; rdx = rbx; rcx = ((r9.wrapping_add(rbx)) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = ((r9.wrapping_add(rbx)) as u64) as u64; rcx = rbx; bb = 2; continue; }
            2 => { rdi = rbx; xmm0 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = rcx.wrapping_add(0xc); rdx = rbx; rcx = rbx; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 6; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = rbp.wrapping_sub(0x20); r15 = rbp.wrapping_sub(0x20); r9 = rbp.wrapping_sub(0x20); r12 = rbp.wrapping_sub(0x20); r10 = rbp.wrapping_sub(0x20); r13 = rbp.wrapping_sub(0x20); bb = 8; continue; }
            8 => { rsi = rbp.wrapping_sub(0x50); rcx = rbp.wrapping_sub(0x40); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x2; rsi = (if (((fl_l as u32)) != ((fl_r as u32))) { rcx } else { rsi }) as u64; r11 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; rdx = ((r8.wrapping_add(r8)) as u64) as u64; rcx = ((r11.wrapping_add(r11)) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = r11.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = r15; r9 = r12; r10 = r13; bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rcx.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; crate::simd_ops::wu16(rdx, ((rcx as u16)) as u16); rcx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x3; rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::lk_or_u8(r14.wrapping_add(0x113u64), ((rcx as u8)) as u8); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rdx.wrapping_sub(0x2u64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            12 => { rax = r10; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { r9 } else { rax }) as u64; rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            13 => { crate::simd_ops::lk_or_u8(r14.wrapping_add(0x112u64), ((rcx as u8)) as u8); rbx = ((rbx).wrapping_add(0xcu64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((rcx.wrapping_mul(4)) as u64) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = ((rcx.wrapping_add(r8)) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdx = ((r8.wrapping_add(rcx)) as u64) as u64; bb = 15; continue; }
            15 => { rdi = rbx; xmm0 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 17; continue; } bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_add(0xcu64)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 17; continue; }
            17 => { r15 = rax; r9 = rax; r12 = rax; r10 = rax; r13 = rax; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 18; continue; }
            18 => { r10 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; r12 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x38u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = ((r10 as u32)) as u32 as u64; rcx = ((r14.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(r14)) as u64; rcx = ((((rcx) as i64).wrapping_shr(1u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), r14); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); r8 = rbx; rdx = rbp.wrapping_sub(0x60); rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu128(rsi.wrapping_add(0x18u64), (xmm0) as u128); rax = ((r12 as u32)) as u32 as u64; rcx = ((r15.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_sub(r15)) as u64; rcx = ((((rcx) as i64).wrapping_shr(1u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), r15); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); r8 = rbx; rdx = rbp.wrapping_sub(0x60); rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsi.wrapping_add(0x28u64), (xmm0) as u128); rbx = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; r12 = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { r13 = 0x48; bb = 21; continue; }
            21 => { rdi = 0x38; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x2; rdi = (if (((fl_l as u32)) != ((fl_r as u32))) { r13 } else { rdi }) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; crate::simd_ops::wu16(rdx, ((rax as u16)) as u16); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x3; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::lk_or_u8(rsi.wrapping_add(0x113u64), ((rax as u8)) as u8); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 24; continue; }
            24 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a3910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420a5380; const P1: u64 = 0x1420a4490; const P2: u64 = 0x1420a3e40; const P3: u64 = 0x14241c190; const P4: u64 = 0x14065ffa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1420a3910, funcinfo_rva: 0x379c2cc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x148, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x10u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x110u64)) as u64; rbx = (rdx) as u64; r12 = (rcx) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r15 as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); crate::simd_ops::wu64(rcx, (r15) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (r15) as u64); r14 = ((rcx.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu128(r14, (xmm0) as u128); crate::simd_ops::wu64(r14, (r15) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (r15) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); r9 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; r8 = (rdx) as u64; rdx = (rcx) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a3981); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; r8 = (rbx) as u64; rdx = (r12) as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a3996); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (r14) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x59u64), (0x1u8) as u8); rdi = (crate::simd_ops::ru64(r14)) as u64; rsi = ((r15 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (0xffffu32) as u32 as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (rdi) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x8u64), (xmm0) as u128); fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0x2u64))) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rsi = ((rsi).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; rcx = (rdi) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 5; continue; }
            5 => { r14 = (crate::simd_ops::ru64(rbp)) as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; r13 = ((crate::simd_ops::ru16(rbp.wrapping_add(0x50u64))) as u32) as u32 as u64; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = ((crate::simd_ops::ru16(rbx.wrapping_add(r15.wrapping_mul(2)))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rbx as u16)) as u16); r9 = ((crate::simd_ops::ru16(r14)) as u32) as u32 as u64; r8 = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a3a4f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x62u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru16(rbp.wrapping_add(0x60u64))) as u32) as u32 as u64; crate::simd_ops::wu16(r14.wrapping_add(0x2u64), ((rax as u16)) as u16); bb = 15; continue; }
            8 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(r12)) as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 9; continue; }
            9 => { rcx = (rax) as u64; bb = 10; continue; }
            10 => { r8 = (rax) as u64; rax = ((rax).wrapping_add(0xffffffffffffffc0u64)) as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_sub(0x1eu64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 12; continue; }
            12 => { bb = 14; continue; }
            13 => { r13 = ((crate::simd_ops::ru16(r8.wrapping_sub(0x20u64))) as u32) as u32 as u64; bb = 14; continue; }
            14 => { crate::simd_ops::wu16(r14.wrapping_add(0x2u64), ((r13 as u16)) as u16); bb = 15; continue; }
            15 => { rsi = ((rsi).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = (0xffffu32) as u32 as u64; bb = 17; continue; }
            17 => { r15 = (rsi) as u64; r14 = (rdi) as u64; fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(0x2u64))) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rsi = ((rsi).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 19; continue; }
            19 => { fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 20; continue; }
            20 => { r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a3aee); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (r15) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a3b08); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (r15) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (0x0u64) as u64); rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a3b1d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a3b2d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 27; continue; }
            27 => { rax = (r12) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x158u64))) as u64; rsp = ((rsp).wrapping_add(0x110u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a3b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420a3e40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; rdx = rcx.wrapping_add(0x10); crate::simd_ops::wu8(rsp.wrapping_add(0x49u64), (0x1u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdx); rsi = 0; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rdx)) as u64; r12 = rcx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r13 = r9; rcx = 0xffff; rax = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x78u64), (xmm0) as u128); fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(0x2u64))) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rsi = ((rsi).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r15); r15 = ((crate::simd_ops::ru16(rsp.wrapping_add(0xd8u64))) as u32) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; rdx = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r13)) as u64; rcx = rsp.wrapping_add(0xd8); r9 = ((crate::simd_ops::ru16(r14)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(r12)) as u64; rbx = ((crate::simd_ops::ru16(rbx.wrapping_add(rbp.wrapping_mul(2)))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rbx as u16)) as u16); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xdau64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0xd8u64))) as u32) as u32 as u64; crate::simd_ops::wu16(r14.wrapping_add(0x2u64), ((rax as u16)) as u16); bb = 15; continue; }
            8 => { r8 = (crate::simd_ops::ru64(r12)) as u64; rax = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 9; continue; }
            9 => { rdx = rax; bb = 10; continue; }
            10 => { rcx = rax.wrapping_sub(0x40); fl_l = (crate::simd_ops::ru16(rdx.wrapping_sub(0x1eu64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = rcx; rdx = rcx; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 12; continue; }
            12 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { r15 = ((crate::simd_ops::ru16(rax.wrapping_sub(0x20u64))) as u32) as u32 as u64; bb = 14; continue; }
            14 => { crate::simd_ops::wu16(r14.wrapping_add(0x2u64), ((r15 as u16)) as u16); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = ((rsi).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = 0xffff; bb = 17; continue; }
            17 => { rbp = rsi; r14 = rdi; fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(0x2u64))) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rsi = ((rsi).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 19; continue; }
            19 => { fl_l = rdi; fl_r = rax; bb = 20; continue; }
            20 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 21; continue; }
            21 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 22; continue; }
            22 => { rsp = ((rsp).wrapping_add(0xa8u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a5150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241c0a0; const P1: u64 = 0x14241c190; const P2: u64 = 0x14297fdd4; const P3: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx, ((r14 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r14); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r14); rdx = ((r14.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (0x7u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (0x8u64) as u64); crate::simd_ops::wu32(rbx, (0x3f800000u32) as u32); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((r14 as u32)) as u32 as u64; rax = ((r14 as u32)) as u32 as u64; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = rax; fl_r = (0x10u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = ((r14.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(0x78u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; fl_l = rdx; fl_r = (0x8u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); rax = rdi.wrapping_add(0x80); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi, rsi); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { bb = 10; continue; }
            8 => { rcx = ((rcx).wrapping_add(0x7u64)) as u64; rcx = ((rcx).wrapping_shr(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdi = 0; fl_l = 0; fl_r = 0; rax = rsi; while rcx != 0 { crate::simd_ops::wu64(rdi, rax); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); } bb = 10; continue; }
            10 => { xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rbx.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), r14); rdi = (crate::simd_ops::ru32(P2)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), r14); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = ((rbp.wrapping_mul(4)) as u64) as u64; rdx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x5cu64), ((rbp as u32)) as u32); rax = ((rdi as u32)) as u32 as u64; rdi = rcx; rcx = rbp; while rcx != 0 { crate::simd_ops::wu32(rdi, rax as u32); rdi = rdi.wrapping_add(4); rcx = rcx.wrapping_sub(1); } bb = 12; continue; }
            12 => { rdx = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rdi = ((rax as u32)) as u32 as u64; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = rbp; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rbp } else { rdi }) as u64; rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; rcx = ((rdi.wrapping_add(rdi)) as u64) as u64; rdx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r8 = ((r8).wrapping_add(r8)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), ((rdi as u32)) as u32); bb = 16; continue; }
            16 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a5380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411ebe80; const P1: u64 = 0x14028e1c0; const P2: u64 = 0x14183acc2; const P3: u64 = 0x1420a5dc0; const P4: u64 = 0x1420a5ba0; const P5: u64 = 0x1420a58b0; const P6: u64 = 0x1401de830; const P7: u64 = 0x1401e59a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x18); rsp = ((rsp).wrapping_sub(0x118u64)) as u64; rsi = r9; r12 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdx); crate::simd_ops::wu8(rbp.wrapping_add(0x72u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; r8 = rbp.wrapping_add(0x70); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r9); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((rbx as u32)) as u32); r8 = ((rbx.wrapping_add(rbx)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rbx as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x0u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), r9); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r13 = (crate::simd_ops::ru64(rsi)) as u64; r13 = ((r13).wrapping_shl(4u32)) as u64; r13 = ((r13).wrapping_add(rdi)) as u64; fl_l = rdi; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = ((crate::simd_ops::ru16(rax.wrapping_add(0x8u64))) as u32) as u32 as u64; r15 = (((rsi as u16)) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r10); xmm2 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x68u64), (xmm2) as u128); r8 = (crate::simd_ops::ru32(r10.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), r9); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), r8); crate::simd_ops::wu8(rbp.wrapping_sub(0x3fu64), (0x1u8) as u8); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x28u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x70u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); crate::simd_ops::wu64(rbp, (((xmm2 >> 64) as u64)) as u64); rbx = r9; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { rax = r9; r10 = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; bb = 6; continue; }
            6 => { r14 = (((rax as u16)) as u32) as u32 as u64; rax = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rdx = rcx; rdx = (((rdx) & (rax))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rbx = ((rbx).wrapping_add(1)) as u64; rax = rbx; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { bb = 11; continue; }
            10 => { r14 = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x58u64))) as u32) as u32 as u64; bb = 11; continue; }
            11 => { fl_l = rbx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { r8 = r12.wrapping_add(0x1); rdx = (((r14 as u16)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (r15 & !0xffffu64) | (((((r15 as u16)).wrapping_add(1)) as u16 as u64)); rbx = ((rbx).wrapping_add(1)) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 14; continue; }
            14 => { rdx = rbx; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; r9 = (crate::simd_ops::ru64(rbp)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; bb = 15; continue; }
            15 => { rax = (((rdx as u16)) as u32) as u32 as u64; r14 = (((rax as u16)) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rdx = rcx; rdx = (((rdx) & (rax))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = rax; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rbx = ((rbx).wrapping_add(1)) as u64; rdx = rbx; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 19; continue; }
            19 => { fl_l = ((rsi as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 25; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { rdx = (((rsi as u16)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rax; r10 = (((rsi as u16)) as u32) as u32 as u64; r10 = ((r10).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((crate::simd_ops::ru16(r10.wrapping_add(rdx).wrapping_add(0x22u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x70u64), ((rsi as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x72u64), (0x1u8) as u8); rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8.wrapping_mul(4)), ((rdx as u32)) as u32); rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x78u64), ((r9 as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x7au64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(r10).wrapping_add(0x36u64), ((rax as u32)) as u32); rdx = (((r9 as u16)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), ((rsi as u16)) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x32u64), (0x1u8) as u8); rdx = ((rdx).wrapping_shl(6u32)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx).wrapping_add(0x32u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((crate::simd_ops::ru16(r8.wrapping_add(r10).wrapping_add(0x22u64))) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x10u64)))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rax = ((crate::simd_ops::ru16(r8.wrapping_add(r10).wrapping_add(0x22u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); r8 = (((rsi as u16)) as u32) as u32 as u64; rdx = r11; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rsi = (rsi & !0xffffu64) | (((((rsi as u16)).wrapping_add(1)) as u16 as u64)); fl_l = ((rsi as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 21; continue; } bb = 25; continue; }
            25 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = r13; r9 = 0x0; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 26; continue; }
            26 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; bb = 27; continue; }
            27 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = ((crate::simd_ops::ru16(rdi.wrapping_add(0x8u64))) as u32) as u32 as u64; r14 = (((rdi as u16)) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r9); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rcx); crate::simd_ops::wu8(rbp.wrapping_sub(0x5fu64), (0x1u8) as u8); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x70u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rax); rbx = r9; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { rax = r9; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 29; continue; }
            29 => { rsi = (((rax as u16)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x2u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 33; continue; } bb = 30; continue; }
            30 => { rbx = ((rbx).wrapping_add(1)) as u64; rax = rbx; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 31; continue; }
            31 => { bb = 33; continue; }
            32 => { rsi = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x58u64))) as u32) as u32 as u64; bb = 33; continue; }
            33 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 47; continue; } bb = 34; continue; }
            34 => { r8 = r15.wrapping_add(0x1); rdx = (((rsi as u16)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (r14 & !0xffffu64) | (((((r14 as u16)).wrapping_add(1)) as u16 as u64)); rbx = ((rbx).wrapping_add(1)) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 35; continue; }
            35 => { rcx = rbx; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 36; continue; }
            36 => { rax = (((rcx as u16)) as u32) as u32 as u64; rsi = (((rax as u16)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x2u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = rbx; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 38; continue; }
            38 => { fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 39; continue; }
            39 => { fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r14 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 46; continue; } bb = 40; continue; }
            40 => { bb = 41; continue; }
            41 => { rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rax; rbx = (((rdi as u16)) as u32) as u32 as u64; rbx = ((rbx).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((crate::simd_ops::ru16(rbx.wrapping_add(rdx).wrapping_add(0x22u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x68u64), ((rdi as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x6au64), (0x1u8) as u8); rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x68u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8.wrapping_mul(4)), ((rdx as u32)) as u32); rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 44; continue; } bb = 42; continue; }
            42 => { r9 = (((r9 as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x70u64), ((r9 as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x72u64), (0x1u8) as u8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rbx).wrapping_add(0x36u64), ((rax as u32)) as u32); rax = ((r9 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x78u64), ((rdi as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x7au64), (0x1u8) as u8); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x32u64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru16(rdx.wrapping_add(rbx).wrapping_add(0x22u64))) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x10u64)))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 44; continue; } bb = 43; continue; }
            43 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rax = ((crate::simd_ops::ru16(rdx.wrapping_add(rbx).wrapping_add(0x22u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = r10; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(rax).wrapping_add(0x30u64), (0x1u8) as u8); rdi = (rdi & !0xffffu64) | (((((rdi as u16)).wrapping_add(1)) as u16 as u64)); fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r14 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 41; continue; } bb = 45; continue; }
            45 => { r9 = 0; fl_l = 0; fl_r = 0; bb = 27; continue; }
            46 => { r9 = 0; fl_l = 0; fl_r = 0; bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi, rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x7cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x0u64) as u64); rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; rsp = ((rsp).wrapping_add(0x118u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a5ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241c0a0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x14241c190; const P3: u64 = 0x14039c2d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; r11 = (((rdx as u16)) as u32) as u32 as u64; rdi = rcx; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r10 = ((r11 as u32)) as u32 as u64; r10 = ((r10).wrapping_shl(6u32)) as u64; r10 = ((r10).wrapping_add(crate::simd_ops::ru64(r9))) as u64; rax = ((crate::simd_ops::ru16(r9.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x98u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rsp.wrapping_add(0x9au64), ((rax as u16)) as u16); r14 = r10.wrapping_add(0x3a); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rax = ((crate::simd_ops::ru16(r10.wrapping_add(0x22u64))) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x26u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rax = (((xmm0 >> 64) as u64)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu16(rsp.wrapping_add(0xa0u64), ((r11 as u16)) as u16); rax = 0xffff; crate::simd_ops::wu16(rsp.wrapping_add(0xa2u64), ((rax as u16)) as u16); rsi = rdi.wrapping_add(0x30); bb = 2; continue; }
            2 => { rax = (((r8 as u16)) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r15 = ((rax.wrapping_mul(4)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), r15); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rbx = ((crate::simd_ops::ru16(r15.wrapping_add(rax))) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(rax).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xa4u64), ((rax as u32)) as u32); r13 = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r12 = ((r13.wrapping_mul(8)) as u64) as u64; rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rax = ((rcx.wrapping_mul(8)) as u64) as u64; fl_l = r12; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = r13.wrapping_add(0x1); r15 = ((rcx as u32)) as u32 as u64; r15 = ((r15).wrapping_shr(1u32)) as u64; r15 = ((r15).wrapping_add(rcx)) as u64; fl_l = r15; fl_r = rax; r15 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r15 }) as u64; rcx = ((r15.wrapping_mul(8)) as u64) as u64; rdx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbp); crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((r15 as u32)) as u32); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(r12.wrapping_add(rbp), rax); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32); rax = ((crate::simd_ops::ru16(r14)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x98u64), ((rax as u16)) as u16); rcx = ((crate::simd_ops::ru16(r14.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = (rcx & !0xffffu64) | (((((rcx as u16)).wrapping_add(1)) as u16 as u64)); crate::simd_ops::wu16(rsp.wrapping_add(0x9au64), ((rcx as u16)) as u16); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu32(r14, ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru32(r15.wrapping_add(rcx).wrapping_add(0x8u64))) as u32 as u64; r8 = (((rdx as u16)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(16u32)) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 9; continue; }
            9 => { rax = r14; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a5dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434278a0; const P1: u64 = 0x14241c0a0; const P2: u64 = 0x1411ebe00; const P3: u64 = 0x1411ec170; const P4: u64 = 0x14183acbc;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); rbx = (((rdx as u16)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r15 = ((crate::simd_ops::ru16(r14.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0xc8u64), ((r15 as u16)) as u16); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rsi = ((rax.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rsi); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm6 = (crate::simd_ops::ru128(P0)) as u128; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(0x10u64)))) as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rbp = ((rbp).wrapping_add(r8)) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbp); rcx = ((rcx).wrapping_add(0x40u64)) as u64; rax = r11.wrapping_add(0x8); fl_l = rbp; fl_r = (crate::simd_ops::ru64(rcx)) as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx, rax); xmm7 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); crate::simd_ops::wu16(r11.wrapping_sub(0x78u64), ((r15 as u16)) as u16); crate::simd_ops::wu16(rsp.wrapping_add(0x42u64), ((rbx as u16)) as u16); crate::simd_ops::wu64(r11.wrapping_sub(0x70u64), rbp); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x54u64), (0x0u8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), (0x0u8) as u8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x5au64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; r13 = ((rax as u32)) as u32 as u64; r13 = ((r13).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; r12 = (crate::simd_ops::ru64(r14)) as u64; fl_l = r13; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { r13 = ((((r13) as i64).wrapping_shr(6u32) as u64)) as u64; rax = r13.wrapping_add(0x1); rcx = ((((rcx) as i64).wrapping_shr(6u32) as u64)) as u64; r15 = rcx; r15 = ((r15).wrapping_shr(1u32)) as u64; r15 = ((r15).wrapping_add(rcx)) as u64; fl_l = r15; fl_r = rax; r15 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r15 }) as u64; rcx = r15; rcx = ((rcx).wrapping_shl(6u32)) as u64; rdx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = rax; rdi = (crate::simd_ops::ru64(r14)) as u64; rbp = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rbp = ((rbp).wrapping_shl(6u32)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rsi = rax.wrapping_add(0x22); bb = 3; continue; }
            3 => { rcx = rsi.wrapping_sub(0x22); rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_sub(0x2u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x22u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsi, ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x6u64), rax); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xeu64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x32u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x36u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x3au64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), ((rax as u32)) as u32); rsi = rsi.wrapping_add(0x40); rdi = ((rdi).wrapping_add(0x40u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 5; continue; }
            5 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r14, r12); crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((r15 as u32)) as u32); rax = ((r13 as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r15 = ((crate::simd_ops::ru16(rsp.wrapping_add(0xc8u64))) as u32) as u32 as u64; bb = 6; continue; }
            6 => { rdi = ((rax as u32)) as u32 as u64; rdi = ((rdi).wrapping_shl(6u32)) as u64; rdi = ((rdi).wrapping_add(r12)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); rax = (((xmm6 >> 64) as u64)) as u64; fl_l = rax; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdi, ((xmm7 as u64)) as u64); bb = 9; continue; }
            8 => { r8 = ((xmm6 as u64)) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { crate::simd_ops::wu128(rdi.wrapping_add(0x10u64), (xmm6) as u128); crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((r15 as u16)) as u16); crate::simd_ops::wu16(rdi.wrapping_add(0x22u64), ((rbx as u16)) as u16); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rbp); crate::simd_ops::wu8(rdi.wrapping_add(0x30u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x52u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x32u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x56u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x36u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x5au64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x3au64), ((rax as u32)) as u32); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x8u64), (1) as u32); crate::simd_ops::lk_add_u16(rsi, (1) as u16); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = ((rsp).wrapping_add(0x80u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a6680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241c0a0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x14241c190; const P3: u64 = 0x1420a6ca0; const P4: u64 = 0x14039c2d0; const P5: u64 = 0x1420a6b70;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x18); rsp = ((rsp).wrapping_sub(0x118u64)) as u64; r14 = r9; rsi = r8; r15 = rdx; r11 = rcx; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, r12); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r12); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = ((rax as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax)) as u64) as u64; rdx = ((r12.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; r8 = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_add(r8)) as u64; rdx = (crate::simd_ops::ru64(r15)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r15)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r15, rbx); crate::simd_ops::wu32(r15.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(r15.wrapping_add(0xcu64), ((rdi as u32)) as u32); r11 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru32(r11.wrapping_add(0x18u64))) as u32 as u64; rdi = 0xffff; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((rbx.wrapping_add(rbx)) as u64) as u64; rdx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (((rdi as u16)) as u32) as u32 as u64; rdi = rdx; rcx = ((rbx as u32)) as u32 as u64; while rcx != 0 { crate::simd_ops::wu16(rdi, rax as u16); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); } r11 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r11); xmm2 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x78u64), (xmm2) as u128); rdi = (crate::simd_ops::ru32(r11.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rdi); crate::simd_ops::wu8(rbp.wrapping_sub(0x5fu64), (0x1u8) as u8); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x70u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (((xmm2 >> 64) as u64)) as u64); r13 = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); rax = r12; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 9; continue; }
            9 => { rbx = (((rax as u16)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rax = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rdx = rcx; rdx = (((rdx) & (rax))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = rax; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { r13 = ((r13).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); rax = r13; fl_l = r13; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 12; continue; }
            12 => { bb = 14; continue; }
            13 => { rbx = ((crate::simd_ops::ru16(rbp)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); bb = 14; continue; }
            14 => { fl_l = r13; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm2) as u128); r12 = (((rbx as u16)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); rdi = ((r12.wrapping_add(r12.wrapping_mul(2))) as u64) as u64; fl_l = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; bb = 16; continue; }
            16 => { r9 = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; r10 = ((rdi.wrapping_add(r9)) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; rdx = rax; rdx = (((rdx) & (rsi))) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0xffff; fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(r12.wrapping_mul(2)))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 22; continue; } bb = 19; continue; }
            19 => { xmm0 = (crate::simd_ops::ru128(r10)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rax = ((crate::simd_ops::ru16(r10.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x28u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(r10.wrapping_add(0x22u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); r9 = r14; r8 = rsp.wrapping_add(0x40); rdx = rbp.wrapping_add(0x68); rcx = r11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(r12.wrapping_mul(2)), ((rcx as u16)) as u16); rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r15)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = ((crate::simd_ops::ru16(rbx.wrapping_add(r12.wrapping_mul(2)))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx, ((rax as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u32(r15.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; r11 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; r9 = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; bb = 22; continue; }
            22 => { rbx = ((crate::simd_ops::ru16(rbx.wrapping_add(r12.wrapping_mul(2)))) as u32) as u32 as u64; bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r11); r10 = (crate::simd_ops::ru32(rdi.wrapping_add(r9).wrapping_add(0x26u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), ((r10 as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rax); let y_addr: u64 = rbp.wrapping_sub(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x18u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rax = (((xmm0 >> 64) as u64)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 25; continue; }
            25 => { rax = (((r10 as u16)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r12)) as u64; r10 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r10 as u32)) as u32); r8 = ((crate::simd_ops::ru16(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdx); r9 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; rcx = rax; rcx = (((rcx) & (rdi))) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 26; continue; }
            26 => { fl_l = r13; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 32; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = 0xffff; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(r8.wrapping_mul(2)))) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 31; continue; } bb = 28; continue; }
            28 => { xmm0 = (crate::simd_ops::ru128(r9)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm0) as u128); rax = ((crate::simd_ops::ru16(r9.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x28u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(r9.wrapping_add(0x22u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); r9 = r14; r8 = rbp.wrapping_sub(0x70); rdx = rbp.wrapping_add(0x78); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu16(rdx.wrapping_add(rax.wrapping_mul(2)), ((rcx as u16)) as u16); rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r15)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(rax.wrapping_mul(2)))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx, ((rax as u16)) as u16); fl_l = (crate::simd_ops::lk_add_u32(r15.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r10 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rax = rcx; bb = 31; continue; }
            31 => { r8 = ((crate::simd_ops::ru16(rax.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; bb = 32; continue; }
            32 => { r9 = ((r10 as u32)) as u32 as u64; rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x10u64))) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r12)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((rdx as u32)) as u32); r10 = (((rdx as u16)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(16u32)) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 25; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r11 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; bb = 34; continue; }
            34 => { r13 = ((r13).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); fl_l = r13; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 41; continue; } bb = 35; continue; }
            35 => { rdx = r13; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 36; continue; }
            36 => { r12 = (((rdx as u16)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); rbx = (((r12 as u16)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rdi = ((r12.wrapping_add(r12.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); rax = (crate::simd_ops::ru64(r9.wrapping_add(rdi).wrapping_add(0x10u64))) as u64; rdx = rcx; rdx = (((rdx) & (rax))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 38; continue; }
            38 => { r13 = ((r13).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); rdx = r13; fl_l = r13; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 39; continue; }
            39 => { bb = 42; continue; }
            40 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); bb = 42; continue; }
            41 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 42; continue; }
            42 => { fl_l = r13; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 45; continue; }
            45 => { rax = r15; rsp = ((rsp).wrapping_add(0x118u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a7b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420a7f10; const P1: u64 = 0x14241c0a0; const P2: u64 = 0x1411ec600; const P3: u64 = 0x1420a76b0; const P4: u64 = 0x1411ed390; const P5: u64 = 0x14241c190;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1420a7b20, funcinfo_rva: 0x379c8fc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x128, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = ((rsp.wrapping_sub(0x10u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x110u64)) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a7b48); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rsp.wrapping_add(0x58u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rdx = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r14) as u64); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_mul(8)) as u64) as u64; rdx = ((r14.wrapping_add(0x8u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a7b87); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rbx as u32)) as u32); bb = 2; continue; }
            2 => { rcx = (r14) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdx, (rcx) as u64); rdx = ((rdx.wrapping_add(0x8u64)) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rax = ((rdx.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; r8 = (rax) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; r8 = ((((r8) as i64).wrapping_shr(3u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (rdx) as u64); r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a7be7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x1u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), (rsi) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x60u64), (0x1u8) as u8); rax = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x0u8) as u8); rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a7c3b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rbp.wrapping_sub(0x28u64), (0x0u8) as u8); crate::simd_ops::wu8(rbp, (0x1u8) as u8); rax = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x30u64), (0x0u8) as u8); r8 = ((rbp.wrapping_sub(0x28u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x58u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a7c7c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a7c8c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r14) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a7ca5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (r14) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1420a7cbe); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0x110u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a8050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14297fdd4; const P1: u64 = 0x14241c0a0; const P2: u64 = 0x1420a8900; const P3: u64 = 0x1411ec4e0; const P4: u64 = 0x14241c190; const P5: u64 = 0x1420a8750;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rsi = rcx; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_sub(0x68u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), rdx); crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r8); rbx = (crate::simd_ops::ru32(P0)) as u32 as u64; rbp = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r14); crate::simd_ops::wu32(rax.wrapping_sub(0x1cu64), ((r14 as u32)) as u32); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbp.wrapping_mul(4)) as u64) as u64; rdx = ((r14.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((rbp as u32)) as u32); rax = ((rbx as u32)) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; while rcx != 0 { crate::simd_ops::wu32(rdi, rax as u32); rdi = rdi.wrapping_add(4); rcx = rcx.wrapping_sub(1); } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rbp as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rsi, (xmm0) as u128); crate::simd_ops::wu64(rsi, r14); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r14); rbx = rsi.wrapping_add(0x10); crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx, r14); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsi, rcx); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rdi, r14); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r14); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; rbp = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rdi, (0x0u8) as u8); rdi = ((rdi).wrapping_add(0x38u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx, r14); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r14); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi, r14); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r14); bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rax = rsi; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a84d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241c0a0; const P1: u64 = 0x14183acbc; const P2: u64 = 0x1420a8b80; const P3: u64 = 0x14241c190; const P4: u64 = 0x140200110;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r10 = (crate::simd_ops::ru32(r9.wrapping_add(0x3au64))) as u32 as u64; r9 = r8; r13 = rcx; r11 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = ((r10 as u32)) as u32 as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax).wrapping_sub(r10)) as u64; r8 = (((rax as u16)) as u32) as u32 as u64; rdx = (((r10 as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x31u64), (0x1u8) as u8); rax = ((rbx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = rbx; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { bb = 5; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 6; continue; }
            6 => { rdx = (0x7fffffffffffffffu64) as u64; r8 = 0x16; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; r15 = ((crate::simd_ops::ru16(rdi.wrapping_add(0x2u64))) as u32) as u32 as u64; r15 = ((r15).wrapping_shl(6u32)) as u64; r15 = ((r15).wrapping_add(crate::simd_ops::ru64(rax))) as u64; rbp = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r12 = r15; fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(r15)) as u64; bb = 9; continue; }
            9 => { fl_l = rbp; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 27; continue; } bb = 10; continue; }
            10 => { fl_l = rbp; fl_r = (0xfu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rsi = 0xf; r14 = (crate::simd_ops::ru64(r12)) as u64; bb = 16; continue; }
            12 => { rsi = rbp; rsi = (((rsi) | (0xfu64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rsi = rdx; bb = 15; continue; }
            14 => { fl_l = rsi; fl_r = (0x16u64) as u64; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r8 } else { rsi }) as u64; bb = 15; continue; }
            15 => { rcx = rsi.wrapping_add(0x1); rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; r8 = rbp.wrapping_add(0x1); rdx = r12; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rax = ((crate::simd_ops::ru16(rax.wrapping_add(0x20u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; xmm1 = (crate::simd_ops::ru128(r9)) as u128; r10 = ((crate::simd_ops::ru16(rcx.wrapping_add(rax.wrapping_mul(4)))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(r15.wrapping_add(0x20u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; rcx = (((rax as u16)) as u32) as u32 as u64; rax = (((rdx as u16)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = (((rcx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0xffffffffffffffc8u64)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x0; r9 = (r9 & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rdx = (((r10) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = (((xmm1 >> 64) as u64)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rsi; fl_r = (0xfu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = ((rax.wrapping_mul(8)) as u64) as u64; rax = rsp.wrapping_add(0x20); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 20; continue; }
            20 => { rax = rsp.wrapping_add(0x20); r9 = rsp.wrapping_add(0x20); r10 = rsp.wrapping_add(0x20); bb = 21; continue; }
            21 => { rdi = rbx; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; r9 = rax; r10 = rax; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 23; continue; }
            23 => { bb = 25; continue; }
            24 => { rax = r10; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { r9 } else { rax }) as u64; bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = rbx; fl_r = rcx; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdx = (0x7fffffffffffffffu64) as u64; r8 = 0x16; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            27 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1420a8704, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a8750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241c0a0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x14241c190; const P3: u64 = 0x1420a84d0; const P4: u64 = 0x1420a8b80; const P5: u64 = 0x1420a8260;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = r8; rsi = rdx; rbp = rcx; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r15 as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, r15); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rcx.wrapping_mul(4)) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = ((r15 as u32)) as u32 as u64; fl_l = r15; fl_r = rcx; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rbx }) as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rdx = ((r15.wrapping_add(0x8u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r15); crate::simd_ops::wu64(rsi, rdi); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rbx as u32)) as u32); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rdi = ((rdi).wrapping_shl(6u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = rbx; r8 = rsp.wrapping_add(0x30); rdx = rsi; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x20u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rax = (((rdx as u16)) as u32) as u32 as u64; r8 = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(r15)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x2u16) as u16 as u64; if (((fl_l as u16)) < ((fl_r as u16))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = r8.wrapping_add(0x38); r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = rbx; r8 = rsp.wrapping_add(0x30); rdx = rsi; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rbx = ((rbx).wrapping_add(0x40u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420a8900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411ec410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; r15 = rcx; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, r8); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(rdx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rsi = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rsi).wrapping_shl(6u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r15)) as u64; rdi = ((crate::simd_ops::ru16(rbx.wrapping_add(0x22u64))) as u32) as u32 as u64; rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x10u64)))) as u64; r12 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x30u64))) as u32) as u32 as u64; r13 = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rdx = (((rbp) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbp.wrapping_add(0x1); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r14))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r8); crate::simd_ops::wu16(rcx.wrapping_add(0x20u64), ((rdi as u16)) as u16); crate::simd_ops::wu8(rcx.wrapping_add(0x22u64), ((r12 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r8); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x8u64), (1) as u32); rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rdi = ((crate::simd_ops::ru16(rbx.wrapping_add(0x22u64))) as u32) as u32 as u64; r12 = ((crate::simd_ops::ru8(rbx.wrapping_add(0x30u64))) as u32) as u32 as u64; r13 = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = (((rdx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r14))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r8); crate::simd_ops::wu16(rcx.wrapping_add(0x20u64), ((rdi as u16)) as u16); crate::simd_ops::wu8(rcx.wrapping_add(0x22u64), ((r12 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r8); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rax as u32)) as u32); bb = 13; continue; }
            10 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = rbp.wrapping_add(0x1); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r14))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r8); crate::simd_ops::wu16(rcx.wrapping_add(0x20u64), ((rdi as u16)) as u16); crate::simd_ops::wu8(rcx.wrapping_add(0x22u64), ((r12 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r8); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rax as u32)) as u32); bb = 13; continue; }
            13 => { crate::simd_ops::wu16(rsp.wrapping_add(0x50u64), ((rbp as u16)) as u16); crate::simd_ops::wu16(rsp.wrapping_add(0x52u64), ((rax as u16)) as u16); rdx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x20u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(rdx.wrapping_mul(4)), ((rcx as u32)) as u32); rbx = ((rbx).wrapping_add(0x40u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 14; continue; }
            14 => { rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b6e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x38;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx, rdx);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(r9)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    let y_addr: u64 = rcx.wrapping_add(P2); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rdx);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b6e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x38; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; rdi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_sub(1)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rdx = rsp.wrapping_add(0x30); let y_addr: u64 = rsi.wrapping_add(P1); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = ((rdi.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::lk_add_u64(rsi.wrapping_add(P2), (rax) as u64); rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b6f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = (0x50u64) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b6fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (0x6666666666666667u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(5u32) as u64)) as u64;
    rax = rdx;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b7720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r10 = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64;
    r11 = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64;
    fl_l = r9; fl_r = (0xffffffffffffffffu64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { r9 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r9; fl_r = r8; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; }
    crate::simd_ops::wu64(rdx, r9);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r11);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b7770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P0)))) as u64;
    fl_l = r9; fl_r = (0xffffffffffffffffu64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { r9 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r9; fl_r = r8; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; }
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx, r9);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b7880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b7990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b7a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x28; const P7: u64 = 0x30; const P8: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), r8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), r8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    rax = rcx;
    crate::simd_ops::wu64(rdx.wrapping_add(P6), r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), (0x4000u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P8))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P8), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b7ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P5))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b7b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), r8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b7b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x2c; const P7: u64 = 0x30; const P8: u64 = 0x38; const P9: u64 = 0x40; const P10: u64 = 0x44; const P11: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rdx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P0), r8); crate::simd_ops::wu64(rdx.wrapping_add(P1), r8); crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P4), r8); crate::simd_ops::wu64(rdx.wrapping_add(P5), r8); crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P8), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P10), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(P8), r8); crate::simd_ops::wu64(rdx.wrapping_add(P9), r8); rax = ((crate::simd_ops::ru16(rdx.wrapping_add(P11))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x48u64), ((rax as u16)) as u16); rax = rcx; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b7c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rcx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b7e30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64;
    rdx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    rax = rcx;
    crate::simd_ops::wu64(rcx, rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b7e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P0)))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b7f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P0)))) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b8220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = (((xmm0 >> 64) as u64)) as u64;
    rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b8260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0x28u64) as u64)) as u64; fl_r = fl_l;
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b82a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (((xmm0 >> 64) as u64)) as u64;
    rcx = ((rcx).wrapping_shl(4u32)) as u64;
    fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b82e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0x30u64) as u64)) as u64; fl_r = fl_l;
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b8400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = (((xmm0 >> 64) as u64)) as u64;
    rcx = ((rcx).wrapping_shl(4u32)) as u64;
    fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rax = rdx;
    crate::simd_ops::wu64(rdx, rcx);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b8440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_add_u64(rcx, (0x50u64) as u64)) as u64; fl_r = fl_l;
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b87d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0xc; const P6: u64 = 0x24; const P7: u64 = 0x28; const P8: u64 = 0x30; const P9: u64 = 0x38;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r10 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r10);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), r10);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), r10);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), r10);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r10);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r10);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P8), rax);
    rax = rcx;
    crate::simd_ops::wu64(r8.wrapping_add(P1), r10);
    crate::simd_ops::wu64(r8.wrapping_add(P3), (0x4000u64) as u64);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P9), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b8860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b88a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b88f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x4; const P3: u64 = 0x20; const P4: u64 = 0x18; const P5: u64 = 0x30; const P6: u64 = 0x28; const P7: u64 = 0xffffffffffffffff; const P8: u64 = 0x108; const P9: u64 = 0x100; const P10: u64 = 0xe; const P11: u64 = 0x1425371a0; const P12: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P2))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(P4)))) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; r8 = ((r8).wrapping_add(r8)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rcx = ((r9.wrapping_add(P7)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P8)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P9))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P10))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P4), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P3), r9); if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = rdx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            8 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            9 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b89e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a082f8; const P1: u64 = 0x1433dd9e8; const P2: u64 = 0x142407c80; const P3: u64 = 0x142485aa0; const P4: u64 = 0x14340e320; const P5: u64 = 0x1433dded8; const P6: u64 = 0x142486460; const P7: u64 = 0x142645580; const P8: u64 = 0x142485680; const P9: u64 = 0x1420aa3d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x100u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rsi = rcx; rcx = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x138u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x2au64) as u64); rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rdx = rsp.wrapping_add(0x20); rcx = ((rcx).wrapping_add(0x40u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = 0x1; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0x3; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = P4; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu128(rcx.wrapping_add(0x18u64), (xmm0) as u128); fl_l = ((rax as u8)) as u8 as u64; let cx_src: u8 = (rdx as u8); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u8(rcx.wrapping_add(0x4u64), fl_l as u8, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = (rax & !0xffu64) | ((((fl_r as u8)) as u8 as u64)); } if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rax, (0xffffu32) as u32)) as u32 as u64; fl_r = fl_l; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x2au64), (0x1u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdi = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), r15); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r13 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r12 = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x38u64))) as u128; r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x128u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((r15 as u32)) as u32); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x130u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rax as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x30); rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rsi).wrapping_add(0x48u64))) as u64; rcx = ((rcx).wrapping_sub(0xffffffffffffff80u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = 0x1; r8 = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = P5; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r13); crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), r14); crate::simd_ops::wu32(rdx.wrapping_add(0x38u64), ((rbp as u32)) as u32); crate::simd_ops::wu128(rdx.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rax, (1) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1008u64))) as u64; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x18); rsp = ((rsp).wrapping_add(0x100u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b8c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b8cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x28; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x1420b9480;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = rsp.wrapping_add(0x30);
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    rcx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128);
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b8d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de830; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    fl_l = rcx; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdi, rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); }
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(rdi.wrapping_add(P3), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P4))) as u128;
    crate::simd_ops::wu128(rdi.wrapping_add(P4), (xmm0) as u128);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b8d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1408b4ad0; const P1: u64 = 0x140fae550; const P2: u64 = 0x1420b9ee0; const P3: u64 = 0x1401de830; const P4: u64 = 0x1425371a0; const P5: u64 = 0x142533200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r12 = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r14 = (crate::simd_ops::ru64(rcx)) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; r15 = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; rdx = ((r15.wrapping_add(rdi)) as u64) as u64; rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; rcx = ((rsi.wrapping_add(r13.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_sub(rsi)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; fl_l = rdi; fl_r = (0xffffffffffffffffu64) as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rsi); rcx = ((rcx).wrapping_sub(rdi)) as u64; fl_l = r15; fl_r = (0xffffffffffffffffu64) as u64; rcx = (if (((fl_l as u64)) != ((fl_r as u64))) { r15 } else { rcx }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rcx); rax = ((rsi.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(0x18u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); r8 = rbx.wrapping_add(0xb0); rdx = rbp.wrapping_sub(0x50); rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(0x28u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); r8 = rbx.wrapping_add(0xb0); rdx = rbp.wrapping_sub(0x40); rcx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; rax = rbp.wrapping_sub(0x30); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rsi); bb = 6; continue; }
            6 => { let y_addr: u64 = rbp.wrapping_sub(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(0x10u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            11 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b8f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x143cca830; const P3: u64 = 0x8; const P4: u64 = 0xc; const P5: u64 = 0x140fa5d10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (((r8 as u16)) as u32) as u32 as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0x2710;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rcx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rax = ((rdx.wrapping_add(rdx)) as u64) as u64;
    fl_l = rax; fl_r = rcx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rdi as u16)) as u16);
    crate::simd_ops::lk_add_u32(rbx.wrapping_add(P3), (1) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b92e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f11e0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x14103a0c0; const P3: u64 = 0x1411f1460;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r15 = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; r14 = r8; rbx = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r8 = 0x4; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r15 as u32)) as u32); rsi = r9; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r15 as u32)) as u32); rdx = ((r15.wrapping_mul(4)) as u64) as u64; r12 = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rbp = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdi = rax; bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rbx.wrapping_add(0x10); r8 = ((r8).wrapping_shl(2u32)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; rdi = ((rdi.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdx = ((r15 as u32)) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; r8 = 0x4; rcx = r12; rdx = ((rbx.wrapping_mul(8)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; rcx = ((rdi as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(r10.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; r8 = (crate::simd_ops::ru64(r14)) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; rdx = (((r8 as u16)) as u32) as u32 as u64; r8 = ((r8).wrapping_shr(32u32)) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), r8); rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdx = ((rax.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rcx = rax; r11 = 0xffff; bb = 8; continue; }
            8 => { r10 = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; r8 = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x70u64), ((r11 as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((rdi as u32)) as u32); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(r10.wrapping_mul(8)), r9); fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru64(rax)) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; r9 = ((r8 as u32)) as u32 as u64; r8 = ((r8).wrapping_shr(32u32)) as u64; r8 = (((r8 as u32)).wrapping_add(0x2u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((r8 as u32)) as u32); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(r9.wrapping_mul(8)), r8); crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((r9 as u32)) as u32); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b9610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142489ee0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; r10 = 0xffff; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; xmm1 = (crate::simd_ops::ru128(r9)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm6 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rsi = (crate::simd_ops::ru64(rsi.wrapping_add(rbx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); xmm3 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; xmm4 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; xmm2 = (crate::simd_ops::ru128(rax)) as u128; rax = ((xmm0 as u64)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 4; continue; }
            4 => { rdx = rax; rcx = r8; rdx = ((rdx).wrapping_shr(1u32)) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; rax = rdx; r8 = (if (((fl_l as u32)) >= ((fl_r as u32))) { rcx } else { r8 }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { r10 = 0xffff; bb = 6; continue; }
            6 => { r8 = ((r8).wrapping_sub(1)) as u64; rax = (((xmm1 >> 64) as u64)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 7; continue; }
            7 => { r11 = (((xmm2 >> 64) as u64)) as u64; r14 = ((rcx as u32)) as u32 as u64; r11 = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)))) as u64; rbp = r11; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r11); rbp = ((rbp).wrapping_shr(32u32)) as u64; fl_l = ((rbp as u16)) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 8; continue; }
            8 => { r10 = (crate::simd_ops::ru64(r12)) as u64; r8 = ((r8).wrapping_add(r8)) as u64; rax = (((xmm3 >> 64) as u64)) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rdx = rax; rcx = r8; rdx = ((rdx).wrapping_shr(1u32)) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = rdx; r8 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rcx } else { r8 }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(r8.wrapping_mul(2)).wrapping_sub(0x2u64))) as u32) as u32 as u64; bb = 12; continue; }
            12 => { rax = (((xmm4 >> 64) as u64)) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(r14.wrapping_mul(4)))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r8 = (((rbp as u16)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; r9 = (((rax as u16)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x2eu64))) as u32) as u32 as u64; rax = ((rax).wrapping_shl(16u32)) as u64; r9 = (((r9) | (rax))) as u64; rax = ((r11 as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(32u32)) as u64; r9 = (((r9) | (rax))) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = rax; rcx = r8; rdx = ((rdx).wrapping_shr(1u32)) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = rdx; r8 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rcx } else { r8 }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(r8.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; bb = 16; continue; }
            16 => { rcx = (((rbp as u16)) as u32) as u32 as u64; rax = (((xmm6 >> 64) as u64)) as u64; r11 = (((rsi as u16)) as u32) as u32 as u64; r8 = (((r11 as u16)) as u32) as u32 as u64; rbp = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(0x1806cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x18058u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; r9 = ((r11.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; r9 = ((r9).wrapping_add(r9)) as u64; r8 = ((r14 as u32)) as u32 as u64; rcx = r15; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbx = ((rbx).wrapping_add(1)) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r10 = 0xffff; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 19; continue; }
            19 => { bb = 21; continue; }
            20 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); bb = 21; continue; }
            21 => { rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 22; continue; }
            22 => { bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 24; continue; }
            24 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; xmm1 = (crate::simd_ops::ru128(r9)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm6 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rsi = (crate::simd_ops::ru64(rsi.wrapping_add(rbx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); xmm3 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; xmm4 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; xmm2 = (crate::simd_ops::ru128(rax)) as u128; rax = ((xmm0 as u64)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 26; continue; }
            26 => { rdx = rax; rcx = r8; rdx = ((rdx).wrapping_shr(1u32)) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; rax = rdx; r8 = (if (((fl_l as u32)) >= ((fl_r as u32))) { rcx } else { r8 }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 27; continue; }
            27 => { r8 = ((r8).wrapping_sub(1)) as u64; rax = (((xmm1 >> 64) as u64)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 39; continue; } bb = 28; continue; }
            28 => { r11 = (((xmm2 >> 64) as u64)) as u64; rax = 0xffff; r14 = ((rcx as u32)) as u32 as u64; r11 = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)))) as u64; rbp = r11; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r11); rbp = ((rbp).wrapping_shr(32u32)) as u64; fl_l = ((rbp as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 33; continue; } bb = 29; continue; }
            29 => { r10 = (crate::simd_ops::ru64(r12)) as u64; r8 = ((r8).wrapping_add(r8)) as u64; rax = (((xmm3 >> 64) as u64)) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 30; continue; }
            30 => { bb = 31; continue; }
            31 => { rdx = rax; rcx = r8; rdx = ((rdx).wrapping_shr(1u32)) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = rdx; r8 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rcx } else { r8 }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(r8.wrapping_mul(2)).wrapping_sub(0x2u64))) as u32) as u32 as u64; bb = 33; continue; }
            33 => { rax = (((xmm4 >> 64) as u64)) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(r14.wrapping_mul(4)))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r8 = (((rbp as u16)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x10u64))) as u32 as u64; r10 = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; r9 = (((rax as u16)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x2eu64))) as u32) as u32 as u64; rax = ((rax).wrapping_shl(16u32)) as u64; r9 = (((r9) | (rax))) as u64; rax = ((r11 as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(32u32)) as u64; r9 = (((r9) | (rax))) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 35; continue; }
            35 => { bb = 36; continue; }
            36 => { r8 = rcx; rdx = rax; r8 = ((r8).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_sub(r8)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = ((r8.wrapping_add(rax)) as u64) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = r8; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { rdx } else { rax }) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; bb = 38; continue; }
            38 => { rcx = (((rbp as u16)) as u32) as u32 as u64; rax = (((xmm6 >> 64) as u64)) as u64; r11 = (((rsi as u16)) as u32) as u32 as u64; r8 = (((r11 as u16)) as u32) as u32 as u64; rbp = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(0x1806cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x18058u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; r9 = ((r11.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; r9 = ((r9).wrapping_add(r9)) as u64; r8 = ((r14 as u32)) as u32 as u64; rcx = r15; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 40; continue; }
            40 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbx = ((rbx).wrapping_add(1)) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 41; continue; }
            41 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rsp = ((rsp).wrapping_add(0x58u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b9b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142489ee0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rdi = 0; fl_l = 0; fl_r = 0; r11 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rbx = rdx; r14 = rcx; r8 = ((rdi as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; bb = 2; continue; }
            2 => { rdx = rax; rcx = r8; rdx = ((rdx).wrapping_shr(1u32)) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r11.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; rax = rdx; r8 = (if (((fl_l as u32)) >= ((fl_r as u32))) { rcx } else { r8 }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_sub(1)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(r14.wrapping_add(0x10u64), ((rdi as u8)) as u8); rax = r14; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rbp = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); r11 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = 0xffff; r11 = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)))) as u64; rsi = r11; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r11); rsi = ((rsi).wrapping_shr(32u32)) as u64; fl_l = ((rsi as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r8 = ((r8).wrapping_add(r8)) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(r9)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = rdi; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rdx = rax; rcx = r8; rdx = ((rdx).wrapping_shr(1u32)) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = ((rdx.wrapping_add(r8)) as u64) as u64; fl_l = r10; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = rdx; r8 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rcx } else { r8 }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; rsi = ((crate::simd_ops::ru16(rax.wrapping_add(r8.wrapping_mul(2)).wrapping_sub(0x2u64))) as u32) as u32 as u64; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(rbp.wrapping_mul(4)))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r8 = (((rsi as u16)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r8 = (((rax as u16)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x56u64))) as u32) as u32 as u64; rax = ((rax).wrapping_shl(16u32)) as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; r8 = (((r8) | (rax))) as u64; r8 = ((r8).wrapping_shl(32u32)) as u64; rax = ((r11 as u32)) as u32 as u64; r8 = (((r8) | (rax))) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(0x10u64))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rdx = rax; rcx = rdi; rdx = ((rdx).wrapping_shr(1u32)) as u64; rcx = ((rcx).wrapping_sub(rdx)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = ((rdx.wrapping_add(rdi)) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = rdx; rdi = (if (((fl_l as u64)) >= ((fl_r as u64))) { rcx } else { rdi }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = (((rsi as u16)) as u32) as u32 as u64; r8 = (((rbx as u16)) as u32) as u32 as u64; r11 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(0x1806cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x18058u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r11.wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu64(r14, rdx); rax = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rax); rax = r14; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(r14.wrapping_add(0x10u64), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b9d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    crate::simd_ops::wu64(rcx, r8);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu64(rdx, r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r8);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P2))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b9dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x1408b4ad0; const P6: u64 = 0x140fae550; const P7: u64 = 0xb0; const P8: u64 = 0x1420b9ee0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; rbp = rdx; crate::simd_ops::wu64(rcx, rax); rbx = r8; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rsi = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; r14 = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; rdx = ((r14.wrapping_add(rdi)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; fl_l = rdi; fl_r = (0xffffffffffffffffu64) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = rdi; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rax }) as u64; rdx = ((rdx).wrapping_sub(rdi)) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P1), rax); fl_l = r14; fl_r = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P2), rcx); rdx = (if (((fl_l as u64)) != ((fl_r as u64))) { r14 } else { rdx }) as u64; rax = ((rcx.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P3), rdx); crate::simd_ops::wu64(rsi.wrapping_add(P4), rax); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(P1))) as u128; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x18u64))) as u128; r8 = ((rbx.wrapping_add(P7)) as u64) as u64; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(P3))) as u128; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x28u64))) as u128; r8 = ((rbx.wrapping_add(P7)) as u64) as u64; rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420b9ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241c0a0; const P1: u64 = 0x14183acc2; const P2: u64 = 0x14241c190;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rdi = r8; r9 = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; r10 = 0; fl_l = 0; fl_r = 0; rbx = ((rbx).wrapping_add(0x3fu64)) as u64; rbx = (((rbx) & (0xffffffffffffffc0u64))) as u64; rbx = ((rbx).wrapping_shr(6u32)) as u64; rsi = ((rbx.wrapping_mul(8)) as u64) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((r10.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r15 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r8 = rsi; rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r10 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { r15 = r10; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r10); bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r13 = r10; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rbx = r10; r12 = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rbp = r10; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; bb = 6; continue; }
            5 => { rdx = r10; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rdx); bb = 7; continue; }
            7 => { fl_l = r12; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = rbp; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 9; continue; }
            9 => { rax = r13.wrapping_add(0x1); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(r12.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 10; continue; }
            10 => { r13 = rax; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { r14 = rax; r13 = r10; bb = 13; continue; }
            13 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 15; continue; }
            15 => { rsi = rax; rbx = r10; bb = 32; continue; }
            16 => { rdx = r14; r8 = r13; r13 = rax; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { r14 = rax; r13 = r10; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x10u64))) as u32 as u64; rdx = rsi; r8 = rbx; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rsi = rax; rbx = r10; bb = 22; continue; }
            22 => { rax = ((crate::simd_ops::ru16(rdx.wrapping_add(r8.wrapping_mul(2)).wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0xa8u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rsp.wrapping_add(0xaau64), ((r10 as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0xacu64), ((rcx as u32)) as u32); rdi = rcx; rdi = ((rdi).wrapping_shl(32u32)) as u64; rdi = (((rdi) | (rax))) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r11 = (crate::simd_ops::ru64(r9)) as u64; rax = r10; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 24; continue; }
            24 => { r9 = r11; r9 = ((r9).wrapping_shr(1u32)) as u64; rdx = ((r9.wrapping_add(rax)) as u64) as u64; r10 = rdx.wrapping_add(0x1); r11 = ((r11).wrapping_sub(r9)) as u64; rcx = r11.wrapping_sub(0x1); r11 = r9; fl_l = rdi; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(rdx.wrapping_mul(8)))) as u64; r11 = (if (((fl_l as u64)) > ((fl_r as u64))) { rcx } else { r11 }) as u64; rax = (if (((fl_l as u64)) > ((fl_r as u64))) { r10 } else { rax }) as u64; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 25; continue; }
            25 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r10 = 0; fl_l = 0; fl_r = 0; bb = 26; continue; }
            26 => { r8 = (crate::simd_ops::ru64(r9)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 32; continue; } bb = 27; continue; }
            27 => { rdx = rax; rdx = ((((rdx as u32)) & (0x3fu32))) as u32 as u64; rcx = rax; rcx = ((rcx).wrapping_shr(6u32)) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (((rcx) >> (((rdx) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 27; continue; } bb = 29; continue; }
            29 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 32; continue; } bb = 30; continue; }
            30 => { r8 = ((rax.wrapping_mul(8)) as u64) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(rdx))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rdx = rax; rdx = ((rdx).wrapping_shr(6u32)) as u64; rax = ((((rax as u32)) & (0x3fu32))) as u32 as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = (((rcx) >> (((rax) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((rax) as u32 & 63)))) as u64; crate::simd_ops::wu64(r15.wrapping_add(rdx.wrapping_mul(8)), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(r8))) as u64; rax = (crate::simd_ops::ru64(r12.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(r8), rax); crate::simd_ops::wu64(r12.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x10u64), rdx); bb = 32; continue; }
            32 => { rbp = ((rbp).wrapping_add(1)) as u64; fl_l = rbp; fl_r = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(r12)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 34; continue; }
            34 => { r12 = rax; rbp = r10; bb = 7; continue; }
            35 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = r15; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 37; continue; }
            37 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ba180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420ba1e0; const P1: u64 = 0x30; const P2: u64 = 0x141834ee8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rcx).wrapping_add(0x38u64)) as u64;
    r9 = P0;
    rdx = 0x10;
    r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ba200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(r9)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rdx);
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rdx);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ba270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x448; const P3: u64 = 0x458; const P4: u64 = 0x38; const P5: u64 = 0x440; const P6: u64 = 0x438; const P7: u64 = 0x30; const P8: u64 = 0x18; const P9: u64 = 0x28; const P10: u64 = 0x1420ba900;
    let (mut rax, mut r11) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = ((rsp).wrapping_sub(0xc8u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rcx = r11.wrapping_sub(0x28); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x40u64) as u64); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P2))) as u128; xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128; rax = ((rdx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = r11.wrapping_sub(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm1 as u64)) as u64); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P8))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = r11.wrapping_sub(0x78); crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128; rdx = r11.wrapping_sub(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu128(r11.wrapping_sub(0x28u64), (xmm1) as u128); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0xc8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ba360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f11e0; const P1: u64 = 0x143a082f8; const P2: u64 = 0x1433dda18; const P3: u64 = 0x142407c80; const P4: u64 = 0x142485aa0; const P5: u64 = 0x14340e320; const P6: u64 = 0x1411f1ff0; const P7: u64 = 0x1433dde90; const P8: u64 = 0x142486460; const P9: u64 = 0x142645580; const P10: u64 = 0x1420ba900; const P11: u64 = 0x142485680; const P12: u64 = 0x1411f2570; const P13: u64 = 0x1420ba1e0; const P14: u64 = 0x141834ee8; const P15: u64 = 0x1420bb3a0; const P16: u64 = 0x1420aa3d0; const P17: u64 = 0x1401de830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<512, 193>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x4c8); rsp = ((rsp).wrapping_sub(0x5c8u64)) as u64; r12 = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rax = r8.wrapping_add(0x3f); rax = (((rax) & (0xffffffffffffffc0u64))) as u64; rax = ((rax).wrapping_shr(6u32)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), r8); rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rcx); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); r14 = ((rax.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), r14); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r13); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), r13); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), r13); rax = r14; rax = ((rax).wrapping_shl(9u32)) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rax = rbp.wrapping_sub(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { rbx = r14; rbx = ((rbx).wrapping_shl(6u32)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdi = ((rbx.wrapping_mul(8)) as u64) as u64; r8 = ((r13.wrapping_add(0x8u64)) as u32) as u32 as u64; rdx = rdi; rcx = rbp.wrapping_sub(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = ((r13.wrapping_add(0x8u64)) as u32) as u32 as u64; rdx = rdi; rcx = rbp.wrapping_sub(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rbx as u32)) as u32); rdi = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x528u64), rdi); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x520u64), rbx); rax = P2; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), (0x32u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rcx = ((rcx).wrapping_add(0x40u64)) as u64; rdx = rsp.wrapping_add(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0x1; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r13); rax = P5; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(0x18u64), (xmm0) as u128); rcx = 0x3; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; let cx_src: u8 = (rcx as u8); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u8(rdx.wrapping_add(0x4u64), fl_l as u8, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = (rax & !0xffu64) | ((((fl_r as u8)) as u8 as u64)); } if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rax, (0xffffu32) as u32)) as u32 as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { crate::simd_ops::wu8(rsp.wrapping_add(0x72u64), (0x1u8) as u8); rax = r14; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (crate::simd_ops::ru64(rdi)) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } r15 = rax; rdi = r14; rsi = r13; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rcx); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rcx); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), rcx); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), rcx); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), rcx); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x88u64), rcx); crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), r14); rcx = rbp.wrapping_add(0x98); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r12.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x498u64), rax); rax = (crate::simd_ops::ru64(r12.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4a0u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4a8u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4b0u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4b8u64), rax); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = r14; fl_r = r15; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 13; continue; } bb = 9; continue; }
            9 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x520u64))) as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x528u64))) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rcx = ((rcx).wrapping_sub(0xffffffffffffff80u64)) as u64; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0x1; rax = (crate::simd_ops::ru64(rbp)) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rax = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (0x0u64) as u64); rcx = P7; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rcx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r15); rcx = rbp.wrapping_add(0x60); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::lk_add_u32(rax, (1) as u32); rdx = (crate::simd_ops::ru64(rbp)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = ((r13).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_sub(r15)) as u64; rsi = ((rsi).wrapping_add(r15)) as u64; fl_l = rdi; fl_r = r15; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 11; continue; }
            11 => { fl_l = r13; fl_r = r13; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x510u64))) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x520u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r13 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x1008u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(r12.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(r12.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (0x40u64) as u64); rcx = rbp.wrapping_add(0x98); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rcx); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x50u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x60u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x40u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), ((xmm0 as u64)) as u64); rcx = rbp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rax = rbp.wrapping_sub(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rdi; r8 = rsi; rdx = rbp.wrapping_add(0x40); rcx = rbp.wrapping_sub(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x68); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x528u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rbp.wrapping_add(0x98); r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; r15 = ((r14 as u32)) as u32 as u64; r14 = ((r14 as u32)) as u32 as u64; r14 = ((r14).wrapping_add(rsi)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x64u64))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = r14; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdx = ((rax.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = rdx; fl_r = rbx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rbx), rax); rbx = rbx.wrapping_add(0x8); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; rax = rbp.wrapping_add(0x498); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 20; continue; }
            20 => { r9 = P13; rdx = 0x10; r8 = ((rdx.wrapping_add(0x30u64)) as u32) as u32 as u64; rcx = rbp.wrapping_add(0x98); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(r12.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(r12.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru64(r12)) as u64; rdx = ((rdx).wrapping_add(0x150u64)) as u64; r8 = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r9 = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; rcx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x50); rcx = (crate::simd_ops::ru64(P1)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_sub(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_sub(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x5c8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ba820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a082f8; const P1: u64 = 0x10; const P2: u64 = 0x142407c80; const P3: u64 = 0x48; const P4: u64 = 0x142485aa0; const P5: u64 = 0x8; const P6: u64 = 0x1433ddec0; const P7: u64 = 0x18; const P8: u64 = 0x20; const P9: u64 = 0x28; const P10: u64 = 0x30; const P11: u64 = 0x38; const P12: u64 = 0x142486460; const P13: u64 = 0x142645580;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = (crate::simd_ops::ru64(P0)) as u64; rbp = r9; rsi = r8; rdi = rdx; r14 = rcx; rbx = (crate::simd_ops::ru64(r15.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r10 = ((rax as u32)) as u32 as u64; rdx = r14; r10 = ((r10).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rbx).wrapping_add(P3))) as u64; rcx = ((rcx).wrapping_sub(0xffffffffffffff80u64)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14)) as u64; rax = 0x1; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rcx, ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(P5), (0x0u64) as u64); rax = P6; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P7), rdi); crate::simd_ops::wu64(rcx.wrapping_add(P8), rsi); crate::simd_ops::wu64(rcx.wrapping_add(P9), rbp); crate::simd_ops::wu64(rcx.wrapping_add(P10), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P11), rax); rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1008u64))) as u64; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = r14; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ba900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x1411f2fa0; const P2: u64 = 0x1420baae0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = r9; rsi = r8; r13 = rdx; r14 = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(rsi)) as u64; r10 = rbp; fl_l = rax; fl_r = rbp; r10 = (if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { rax } else { r10 }) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = r10; fl_r = r10; r10 = (if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { rbx } else { r10 }) as u64; rbp = ((rbp).wrapping_sub(r10)) as u64; rax = rsi; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x10u64)))) as u64; r12 = ((rbx as u32)) as u32 as u64; fl_l = r10; fl_r = r10; r12 = (if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { rax } else { r12 }) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = ((rbx as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rdx = rsi; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = r8; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_sub(r8)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = r8; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 9; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); xmm0 = (crate::simd_ops::ru128(r15)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r8 = rdi; rdx = r10; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 11; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 11; continue; }
            11 => { fl_l = rbp; fl_r = rbp; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 19; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r13)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { rbx = r12; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = r12; fl_r = rdx; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 18; continue; } bb = 14; continue; }
            14 => { rbx = ((rbx).wrapping_sub(rdx)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rdx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 14; continue; } bb = 16; continue; }
            16 => { bb = 18; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); xmm0 = (crate::simd_ops::ru128(r15)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r8 = rdi; rdx = rbp; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420baae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f2fa0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r10 = r9; rsi = r8; r13 = rdx; r14 = rcx; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r15 = (crate::simd_ops::ru64(rcx)) as u64; r11 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { r13 = ((r13).wrapping_sub(1)) as u64; xmm0 = (crate::simd_ops::ru128(r14)) as u128; rbp = ((rbp).wrapping_add(1)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rbp); fl_l = rbp; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r15)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r15 = rax; crate::simd_ops::wu64(r14, rax); rbp = r11; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r11); bb = 5; continue; }
            5 => { rcx = ((xmm0 as u64)) as u64; rax = (((xmm0 >> 64) as u64)) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((r9 as u32)) as u32); rcx = (crate::simd_ops::ru64(r10)) as u64; rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(r9.wrapping_mul(8)))) as u32) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx).wrapping_add(0x18068u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_add(0x18050u64))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shr(6u32)) as u64; rdx = (crate::simd_ops::ru64(r12.wrapping_add(rcx.wrapping_mul(8)))) as u64; r8 = ((((r8 as u32)) & (0x3fu32))) as u32 as u64; fl_l = (((rdx) >> (((r8) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x24u64), ((r11 as u16)) as u16); crate::simd_ops::wu16(rsp.wrapping_add(0x26u64), ((rax as u16)) as u16); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx, rax); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 12; continue; }
            12 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 13; continue; }
            13 => { fl_l = r13; fl_r = r13; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 2; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 15; continue; }
            15 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bac40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x24; const P5: u64 = 0x28; const P6: u64 = 0x38;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64;
    crate::simd_ops::wu16(rcx, ((rax as u16)) as u16);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r9);
    rdx = (crate::simd_ops::ru32(rax)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rdx as u32)) as u32);
    rdx = (crate::simd_ops::ru32(rax)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rdx as u32)) as u32);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bacc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1420bad10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rcx)) as u128;
    r9 = r8;
    r8 = rdx;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bad10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241c0a0; const P1: u64 = 0x142489ee0; const P2: u64 = 0x14183acc8; const P3: u64 = 0x14241c190;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x1f); rsp = ((rsp).wrapping_sub(0xd8u64)) as u64; rdi = rdx; let y_addr: u64 = r8; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x29u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = r8.wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x9u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((rax as u32)) as u32); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x79u64), (xmm0) as u128); r12 = 0; fl_l = 0; fl_r = 0; r14 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), r12); rsi = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), r12); r13 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r15 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r12 as u32)) as u32); rdx = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rdx); crate::simd_ops::wu32(rbp.wrapping_sub(0x7du64), ((rdx as u32)) as u32); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rax.wrapping_mul(8)) as u64) as u64; rdx = ((r12.wrapping_add(0x4u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x71u64), ((r12 as u32)) as u32); r13 = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x6du64), ((r13 as u32)) as u32); bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_shl(4u32)) as u64; rdx = 0x8; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r12 as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x7du64), ((rax as u32)) as u32); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rax); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = rdx; rax = ((rax).wrapping_shr(32u32)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); rax = (((rdx as u16)) as u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), rax); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; r11 = ((rax.wrapping_add(rcx)) as u64) as u64; rdx = (crate::simd_ops::ru32(r11.wrapping_add(0x1806cu64))) as u32 as u64; r8 = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x9u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r11.wrapping_add(0x18058u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x18060u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rax = ((rax).wrapping_add(rdi)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); rdx = ((r12 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r15)) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_sub(r15)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x77u64), rdx); r12 = rdx; r12 = ((((r12) as i64).wrapping_shr(4u32) as u64)) as u64; rax = r12.wrapping_add(0x1); rcx = ((rcx).wrapping_sub(r15)) as u64; rcx = ((((rcx) as i64).wrapping_shr(4u32) as u64)) as u64; rdi = rcx; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; fl_l = rdi; fl_r = rax; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rdi); rcx = rdi; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; rdx = r15; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu32(rbp.wrapping_sub(0x7du64), ((rdi as u32)) as u32); bb = 10; continue; }
            10 => { rax = ((r12 as u32)) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(rax.wrapping_mul(8)), rcx); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64), rcx); r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r12 as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rdx = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = ((r13 as u32)) as u32 as u64; rcx = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rdx = ((rdx).wrapping_sub(r14)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x77u64), rdx); rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rsi.wrapping_add(0x1); rcx = ((rcx).wrapping_sub(r14)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; r13 = rcx; r13 = ((r13).wrapping_shr(1u32)) as u64; r13 = ((r13).wrapping_add(rcx)) as u64; fl_l = r13; fl_r = rax; r13 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r13 }) as u64; rcx = ((r13.wrapping_mul(8)) as u64) as u64; rdx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), rax); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; rdx = r14; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), r14); crate::simd_ops::wu32(rbp.wrapping_sub(0x6du64), ((r13 as u32)) as u32); bb = 14; continue; }
            14 => { rax = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(r14.wrapping_add(rax.wrapping_mul(8)), rdi); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x71u64), ((rsi as u32)) as u32); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 15; continue; }
            15 => { r11 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rax = (crate::simd_ops::ru64(r11)) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32 as u64; rax = ((r12 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), r15); rax = ((rsi as u32)) as u32 as u64; rcx = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_sub(r14)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), r14); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x59u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x69u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x69u64), (xmm1) as u128); r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((rdx as u32))) as u32 as u64; r9 = ((r9).wrapping_shl(32u32)) as u64; r9 = (((r9) | (rdx))) as u64; rax = rbp.wrapping_sub(0x1); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rbp.wrapping_sub(0x59); rdx = rbp.wrapping_sub(0x69); rcx = r11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rsp = ((rsp).wrapping_add(0xd8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bb050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f1ca0; const P1: u64 = 0x1420bad10; const P2: u64 = 0x14241c0a0; const P3: u64 = 0x14241c190;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; r14 = rcx; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; r15 = (((r15 as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = ((crate::simd_ops::ru16(r14)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x18u64)))) as u64; r12 = (crate::simd_ops::ru64(rax)) as u64; rbx = ((r15 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rcx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rcx); rax = (crate::simd_ops::ru64(r12)) as u64; r8 = ((r15 as u32)) as u32 as u64; rdx = rbp.wrapping_sub(0x19); rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), r12); xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xfu64), (xmm0) as u128); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), rax); rax = ((crate::simd_ops::ru16(r14)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x27u64), ((rax as u16)) as u16); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x19u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x2fu64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x3fu64), ((xmm1 as u64)) as u64); rbx = (((rbx) as i64).wrapping_mul((crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as i64) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; r8 = rbp.wrapping_add(0x7); crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); fl_l = rbx; fl_r = (0x7d000u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            4 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); rsi = 0; fl_l = 0; fl_r = 0; rdi = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rsi); r10 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rsi); fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = ((r15.wrapping_mul(8)) as u64) as u64; rdx = ((rsi.wrapping_add(0x4u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((r15 as u32)) as u32); r10 = ((r15 as u32)) as u32 as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rax, (0xffffffffffffffffu64) as u64); rax = rax.wrapping_add(0x8); fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((r15 as u32)) as u32); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r11 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; rax = (((r9 as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; r9 = ((r9).wrapping_shr(32u32)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rdx = ((((rsi as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rdx.wrapping_mul(8)), rax); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = r11; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r12)) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rax = ((r10 as u32)) as u32 as u64; rdx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = ((rdx).wrapping_sub(rdi)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rdx); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rdi); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x29); rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bb260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = (crate::simd_ops::ru128(rcx)) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm3 = (crate::simd_ops::ru128(rdx)) as u128;
    rax = ((xmm1 as u64)) as u64;
    rcx = ((rcx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (((xmm1 >> 64) as u64)) as u64;
    fl_l = rax; fl_r = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    rdx = (((xmm3 >> 64) as u64)) as u64;
    rdx = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    fl_l = ((rdx).wrapping_add(0x6u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    loop { r8 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu16(rdx, ((r8 as u16)) as u16); rdx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = rcx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bb2b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((rax.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64;
    fl_l = rax; fl_r = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    rdx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = ((rdx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64;
    fl_l = ((rdx).wrapping_add(0x6u64)) as u64; fl_r = fl_l;
    rdx = ((fl_l as u64)) as u64;
    loop { r8 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu16(rdx, ((r8 as u16)) as u16); rdx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = rcx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bb2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = (crate::simd_ops::ru128(rcx)) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm3 = (crate::simd_ops::ru128(rdx)) as u128;
    rax = ((xmm1 as u64)) as u64;
    rcx = ((rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = (((xmm1 >> 64) as u64)) as u64;
    fl_l = rax; fl_r = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    rdx = (((xmm3 >> 64) as u64)) as u64;
    r10 = 0;
    fl_l = 0; fl_r = 0;
    rdx = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64;
    loop { r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; r9 = rdx; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(r9, ((r8 as u32)) as u32); crate::simd_ops::wu16(r9.wrapping_add(P1), ((r10 as u16)) as u16); fl_l = rax; fl_r = rcx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bb350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
    fl_l = rax; fl_r = rcx;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    rdx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r10 = 0;
    fl_l = 0; fl_r = 0;
    rdx = ((rdx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64;
    loop { r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; r9 = rdx; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu32(r9, ((r8 as u32)) as u32); crate::simd_ops::wu16(r9.wrapping_add(P1), ((r10 as u16)) as u16); fl_l = rax; fl_r = rcx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bba30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x10;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (((rdx as u16)) as u32) as u32 as u64;
    fl_l = ((rdx).wrapping_shr(32u32)) as u64;
    rdx = ((fl_l as u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64;
    r8 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P1))) as u64;
    rax = rcx;
    crate::simd_ops::wu64(rcx, r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bc460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142489ee0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); r14 = r9; r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rbx = rdx; rbp = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; r12 = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); r13 = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(r12)) as u128; rbp = ((rbp).wrapping_add(1)) as u64; crate::simd_ops::wu64(r12.wrapping_add(0x8u64), rbp); fl_l = rbp; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r13)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r13 = rax; crate::simd_ops::wu64(r12, rax); rbp = rdi; crate::simd_ops::wu64(r12.wrapping_add(0x8u64), rdi); bb = 5; continue; }
            5 => { rax = ((xmm0 as u64)) as u64; r8 = (((xmm0 >> 64) as u64)) as u64; xmm0 = (crate::simd_ops::ru128(r14)) as u128; r9 = ((r9).wrapping_add(1)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r9); r8 = ((crate::simd_ops::ru16(rax.wrapping_add(r8.wrapping_mul(2)).wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(r14, rax); r9 = rdi; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdi); rdx = rax; bb = 8; continue; }
            8 => { rcx = ((xmm0 as u64)) as u64; rax = (((xmm0 >> 64) as u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); bb = 23; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); bb = 15; continue; }
            15 => { rcx = ((xmm0 as u64)) as u64; rax = (((xmm0 >> 64) as u64)) as u64; rsi = rdx; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r11 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = rdi; rdi = r9; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { r9 = rcx; rdx = rax; r9 = ((r9).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_sub(r9)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = ((r9.wrapping_add(rax)) as u64) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rcx = r9; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { rdx } else { rax }) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(r14)) as u64; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r9 = rdi; rdx = rsi; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r15 = ((r10 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(r10.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u32 as u64; rax = 0xffff; fl_l = ((rcx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r11 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(0x1806cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x18058u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; r9 = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(r9.wrapping_mul(4)))) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r11.wrapping_add(0x8u64)))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(r15.wrapping_mul(4)))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rdi; rdx = rsi; bb = 22; continue; }
            22 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_r = (0x1u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 24; continue; }
            24 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 25; continue; }
            25 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bc6e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu64(rdx.wrapping_add(P4), r8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bd4d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a082f8; const P1: u64 = 0x142407c80; const P2: u64 = 0x142485aa0; const P3: u64 = 0x1433ddce0; const P4: u64 = 0x142486460; const P5: u64 = 0x142645580; const P6: u64 = 0x142489ee0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rax.wrapping_sub(0x58); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; r15 = (crate::simd_ops::ru64(rcx)) as u64; r11 = rcx; rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(P0)) as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), rsi); rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r13); r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x40u64), r14); r14 = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; r15 = (crate::simd_ops::ru64(r15.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rbx); rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r10 = ((r10).wrapping_add(r10)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r10); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); rcx = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 31; continue; } bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rdi; let d = (r10) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), rdx); rdi = rdx; crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), rax); rdx = rax; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r11); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rdx); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), r13); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rax as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x60); rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdi).wrapping_add(0x48u64))) as u64; rcx = ((rcx).wrapping_sub(0xffffffffffffff80u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = 0x1; let y_addr: u64 = rbp.wrapping_sub(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x50u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = P3; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (0x0u64) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; let y_addr: u64 = rdx.wrapping_add(0x18u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rdx.wrapping_add(0x38u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::lk_add_u32(rax, (1) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1008u64))) as u64; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = rdx; rax = ((rax).wrapping_sub(r12)) as u64; fl_l = rdx; fl_r = rax; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 9; continue; }
            9 => { rsi = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rax; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 8; continue; } bb = 10; continue; }
            10 => { fl_l = ((r12).wrapping_add(rcx)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); bb = 12; continue; }
            12 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = rdx; rax = ((rax).wrapping_sub(r13)) as u64; fl_l = rdx; fl_r = rax; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 17; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 16; continue; }
            16 => { r14 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r13 = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rax; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 15; continue; } bb = 17; continue; }
            17 => { fl_l = ((r13).wrapping_add(rcx)) as u64; fl_r = fl_l; r13 = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rcx = rdx; rax = ((rax).wrapping_sub(r8)) as u64; fl_l = rdx; fl_r = rax; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 23; continue; } bb = 21; continue; }
            21 => { rcx = ((rcx).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 22; continue; }
            22 => { r15 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rax; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 21; continue; } bb = 23; continue; }
            23 => { fl_l = ((r8).wrapping_add(rcx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r8); bb = 25; continue; }
            25 => { r11 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 26; continue; }
            26 => { r9 = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r9); bb = 4; continue; }
            27 => { r12 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; bb = 11; continue; }
            28 => { r13 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; bb = 18; continue; }
            29 => { r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; bb = 24; continue; }
            30 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rax); xmm0 = (crate::simd_ops::ru128(r11.wrapping_add(0x58u64))) as u128; ymm0_hi = 0; xmm1 = (crate::simd_ops::ru128(r11.wrapping_add(0x40u64))) as u128; ymm1_hi = 0; xmm2 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rbp, (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(r11.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(r11.wrapping_add(0x8u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(r11.wrapping_add(0x20u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdx); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm2) as u128); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 32; continue; }
            32 => { rbx = (((xmm0 >> 64) as u64)) as u64; r14 = ((xmm0 as u64)) as u64; rdi = (((xmm2 >> 64) as u64)) as u64; r15 = ((xmm2 as u64)) as u64; r12 = (((xmm1 >> 64) as u64)) as u64; r13 = ((xmm1 as u64)) as u64; bb = 33; continue; }
            33 => { r12 = ((r12).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); fl_l = r12; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(r13)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); r13 = rax; bb = 36; continue; }
            36 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = ((xmm1 as u64)) as u64; rax = (((xmm1 >> 64) as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 43; continue; } bb = 37; continue; }
            37 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); r14 = rax; bb = 40; continue; }
            40 => { rdi = ((rdi).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); fl_l = rdi; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 53; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru64(r15)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 42; continue; }
            42 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); r15 = rax; bb = 53; continue; }
            43 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 46; continue; } bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); r14 = rax; bb = 46; continue; }
            46 => { rcx = ((xmm0 as u64)) as u64; rdi = ((rdi).wrapping_add(1)) as u64; rax = (((xmm0 >> 64) as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), rax); r8 = ((crate::simd_ops::ru16(rcx.wrapping_add(rax.wrapping_mul(2)))) as u32) as u32 as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 49; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(r15)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); r15 = rax; bb = 49; continue; }
            49 => { rax = ((xmm2 as u64)) as u64; r11 = (((xmm2 >> 64) as u64)) as u64; r11 = ((crate::simd_ops::ru16(rax.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x10u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rcx = ((r11.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = r8; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rsi = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; fl_l = ((rsi).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64)))) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 52; continue; }
            51 => { rsi = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; fl_l = ((rsi).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64)))) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; r8 = (((r11 as u16)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(0x1806cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x18058u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x8u64)))) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 53; continue; }
            53 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; fl_r = (0x1u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 55; continue; } bb = 54; continue; }
            54 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; xmm2 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; ymm1_hi = 0; bb = 33; continue; }
            55 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; bb = 56; continue; }
            56 => { rsp = ((rsp).wrapping_add(0x140u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bda00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142489ee0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 1; continue; }
            1 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r12); rbx = rdx; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); r14 = r9; r15 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rsi = r8; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r13); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14)) as u64; r15 = ((r15).wrapping_add(1)) as u64; xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r15); fl_l = r15; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r14, rax); r15 = rdx; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdx); bb = 5; continue; }
            5 => { rcx = ((xmm0 as u64)) as u64; rax = (((xmm0 >> 64) as u64)) as u64; r12 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdx); bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x8u64), (1) as u64); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rdx); bb = 22; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x8u64), (1) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdx); bb = 15; continue; }
            15 => { rcx = ((xmm0 as u64)) as u64; rax = (((xmm0 >> 64) as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x8u64), (1) as u64); r13 = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = ((crate::simd_ops::ru16(r8.wrapping_add(r13.wrapping_mul(2)))) as u32) as u32 as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rdx); bb = 18; continue; }
            18 => { rax = ((xmm0 as u64)) as u64; r11 = (((xmm0 >> 64) as u64)) as u64; r11 = ((crate::simd_ops::ru16(rax.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x10u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rcx = ((r11.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; xmm6 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; xmm7 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = r8; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rbp = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rax = (((xmm6 >> 64) as u64)) as u64; bb = 21; continue; }
            20 => { rbp = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = (((xmm7 >> 64) as u64)) as u64; bb = 21; continue; }
            21 => { rcx = ((rcx).wrapping_shl(5u32)) as u64; r8 = (((r11 as u16)) as u32) as u32 as u64; rbp = ((rbp).wrapping_shl(4u32)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(0x1806cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x18058u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(r13.wrapping_mul(4)))) as u32 as u64; rcx = r12; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x8u64)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_r = (0x1u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 23; continue; }
            23 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 24; continue; }
            24 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bdc70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142489ee0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; r10 = r8; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r15); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm8) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rcx); bb = 3; continue; }
            3 => { rax = rdi; rax = ((rax).wrapping_add(rax)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; xmm6 = (crate::simd_ops::ru128(rcx.wrapping_add(rax.wrapping_mul(8)))) as u128; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdi).wrapping_add(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 15; continue; }
            5 => { xmm7 = (crate::simd_ops::ru128(r9)) as u128; xmm8 = (crate::simd_ops::ru128(r10)) as u128; r12 = ((xmm6 as u64)) as u64; rax = r12; r13 = ((r12 as u32)) as u32 as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = (((rax as u16)) as u32) as u32 as u64; r14 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(rdx)) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r8 = ((crate::simd_ops::ru16(r8.wrapping_add(r13.wrapping_mul(2)))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((r8 as u32)) as u32 as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r11 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rax = (((xmm7 >> 64) as u64)) as u64; bb = 8; continue; }
            7 => { r11 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = (((xmm8 >> 64) as u64)) as u64; bb = 8; continue; }
            8 => { rcx = ((rcx).wrapping_shl(5u32)) as u64; rbp = (((xmm6 >> 32) as u16) as u32) as u32 as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r8 = (((rbp as u16)) as u32) as u32 as u64; r11 = ((r11).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax).wrapping_add(0x1806cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x18058u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = rbx; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; r15 = ((r15).wrapping_add(crate::simd_ops::ru64(r11.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rdx = r15; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r13 = (crate::simd_ops::ru32(rax.wrapping_add(r13.wrapping_mul(4)))) as u32 as u64; r8 = ((r13 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = rdi; rax = ((rax).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru16(rbx.wrapping_sub(0x4u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r13 as u32)) as u32 as u64; rdx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { bb = 16; continue; }
            14 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 15; continue; }
            15 => { fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 16; continue; }
            16 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 18; continue; }
            18 => { rsp = ((rsp).wrapping_add(0x90u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420be280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x14; const P3: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64;
    r8 = ((rdx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu16(rcx, ((rax as u16)) as u16);
    r9 = ((rcx.wrapping_add(P0)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    fl_l = r8; fl_r = r9;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r9, rax); rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), rax); rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(P2), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); crate::simd_ops::wu64(r8.wrapping_add(P0), rax); }
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420be310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xc; const P4: u64 = 0x1c; const P5: u64 = 0x20;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rcx, rdx);
    r10 = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r10.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(r10.wrapping_add(P2), rcx);
    rax = (crate::simd_ops::ru64(r9)) as u64;
    crate::simd_ops::wu64(r10.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r10.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(r10.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(r9, rcx);
    crate::simd_ops::wu64(r9.wrapping_add(P0), rcx);
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = r10;
    crate::simd_ops::wu64(r10.wrapping_add(P5), rcx);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420be360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x40; const P4: u64 = 0x20; const P5: u64 = 0x1420b1ae0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    r9 = ((rax.wrapping_add(P3)) as u64) as u64;
    r8 = ((rax.wrapping_add(P2)) as u64) as u64;
    rdx = ((rdx).wrapping_add(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bf200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x28; const P3: u64 = 0x38;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r10 = rcx;
    crate::simd_ops::wu64(rcx, rax);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(r9)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rax = r10;
    crate::simd_ops::wu64(r10.wrapping_add(P3), rcx);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bf250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f36a0; const P1: u64 = 0x142489ee0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rbp = r11.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xe8u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; xmm2 = (crate::simd_ops::ru128(rcx.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm2) as u128); rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), rdx); crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), r8); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x40u64))) as u128; xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x8u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x79u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x69u64), (xmm1) as u128); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), r15); rdx = ((xmm1 as u64)) as u64; r8 = ((xmm2 as u64)) as u64; r15 = (((xmm1 >> 64) as u64)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); r14 = (((xmm0 >> 64) as u64)) as u64; r13 = ((xmm0 as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rdx); r12 = (((xmm2 >> 64) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x77u64), r8); bb = 2; continue; }
            2 => { r14 = ((r14).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), r14); fl_l = r14; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r13)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), r14); r13 = rax; bb = 5; continue; }
            5 => { r15 = ((r15).wrapping_add(1)) as u64; rcx = ((xmm0 as u64)) as u64; rax = (((xmm0 >> 64) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), r15); r9 = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64))) as u32 as u64; fl_l = r15; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), rax); bb = 8; continue; }
            8 => { r12 = ((r12).wrapping_add(1)) as u64; rax = ((xmm1 as u64)) as u64; rdi = (((xmm1 >> 64) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), r12); rdi = ((crate::simd_ops::ru16(rax.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = r12; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x77u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rax); bb = 11; continue; }
            11 => { rcx = ((xmm2 as u64)) as u64; rax = (((xmm2 >> 64) as u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; r8 = ((r9 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; r8 = (((rdi as u16)) as u32) as u32 as u64; rax = (rax as u32 as i32 as i64) as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; rax = ((crate::simd_ops::ru16(rax.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u32 as u64; r11 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64)))) as u64; rax = ((crate::simd_ops::ru16(r11)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx).wrapping_add(0x1806cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx).wrapping_add(0x18058u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; r9 = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((rdi.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(r9.wrapping_mul(4)))) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r11.wrapping_add(0x8u64)))) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(rsi.wrapping_mul(4)))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x79u64))) as u128; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x69u64))) as u128; xmm2 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x59u64))) as u128; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 2; continue; }
            16 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; bb = 17; continue; }
            17 => { rsp = ((rsp).wrapping_add(0xe8u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bf4a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f36a0; const P1: u64 = 0x142489ee0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r12 = r8; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); r10 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rbp); rbp = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdi); rdi = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r13); r13 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r14); r14 = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r15); r15 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rsi); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r12)) as u64; r13 = ((r13).wrapping_add(1)) as u64; xmm0 = (crate::simd_ops::ru128(r12)) as u128; crate::simd_ops::wu64(r12.wrapping_add(0x8u64), r13); fl_l = r13; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r12, rax); r13 = r10; crate::simd_ops::wu64(r12.wrapping_add(0x8u64), r10); bb = 5; continue; }
            5 => { rcx = ((xmm0 as u64)) as u64; rax = (((xmm0 >> 64) as u64)) as u64; xmm0 = (crate::simd_ops::ru128(r9)) as u128; rbp = ((rbp).wrapping_add(1)) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), rbp); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10u64))) as u32 as u64; fl_l = rbp; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(r9, rax); rbp = r10; crate::simd_ops::wu64(r9.wrapping_add(0x8u64), r10); rdi = rax; bb = 8; continue; }
            8 => { rax = ((xmm0 as u64)) as u64; rbx = (((xmm0 >> 64) as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; r15 = ((r15).wrapping_add(1)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r15); rbx = ((crate::simd_ops::ru16(rax.wrapping_add(rbx.wrapping_mul(2)).wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = r15; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdx, rax); r15 = r10; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r10); r14 = rax; bb = 11; continue; }
            11 => { rcx = ((xmm0 as u64)) as u64; rax = (((xmm0 >> 64) as u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r8 = (((rbx as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r11 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r11 = ((r11).wrapping_shl(4u32)) as u64; r11 = ((r11).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x8u64)))) as u64; rax = ((crate::simd_ops::ru16(r11)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx).wrapping_add(0x1806cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx).wrapping_add(0x18058u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; r9 = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(r9.wrapping_mul(4)))) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r11.wrapping_add(0x8u64)))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(rsi.wrapping_mul(4)))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r10 = 0; fl_l = 0; fl_r = 0; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_r = (0x1u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 16; continue; }
            16 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 17; continue; }
            17 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bf6c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r9);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420bf720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x48; const P2: u64 = 0x10; const P3: u64 = 0xfffffffffffffffc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r11 = rcx; r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp, rdi); bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; r10 = ((r9 as u32)) as u32 as u64; r9 = ((r9).wrapping_shr(48u32)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(P1))) as u64; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(8)).wrapping_add(P2))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { r9 = r10; r9 = ((r9).wrapping_shr(1u32)) as u64; r10 = ((r10).wrapping_sub(r9)) as u64; r10 = ((r10).wrapping_add(rcx)) as u64; r8 = ((r9.wrapping_add(rcx)) as u64) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; rcx = (if (((fl_l as u32)) >= ((fl_r as u32))) { r10 } else { rcx }) as u64; r10 = r9; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru32(r11.wrapping_add(0x10u64))) as u32 as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::lk_add_u64(r11.wrapping_add(0x10u64), (1) as u64); r8 = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(P3), ((r9 as u32)) as u32); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c0400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f11e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rbx = (crate::simd_ops::ru64(r8)) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); rax = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); r13 = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r14 = r9; crate::simd_ops::wu64(rcx, rdx); rdi = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rbp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rsi = ((rbx.wrapping_add(rbx)) as u64) as u64; rcx = rax; rdx = rsi; r8 = ((rbp.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = ((rbp.wrapping_add(0x2u64)) as u32) as u32 as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; r9 = rax; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = r9; fl_l = r8; fl_r = rax; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; crate::simd_ops::wu16(r8.wrapping_add(rax), ((rcx as u16)) as u16); rax = rax.wrapping_add(0x2); fl_l = (((rdx as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r9); bb = 6; continue; }
            6 => { r12 = (crate::simd_ops::ru64(r13)) as u64; rsi = rbp; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rbx = ((crate::simd_ops::ru16(rbx.wrapping_add(rbp.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(rsi.wrapping_mul(8)))) as u32) as u32 as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 22; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 10; continue; }
            10 => { r10 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r15 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; r9 = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = r15; fl_r = rcx; r15 = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { r15 }) as u64; r8 = ((r8).wrapping_add(r8)) as u64; r14 = ((r15.wrapping_add(r15)) as u64) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { r8 = ((r10.wrapping_add(0x2u64)) as u32) as u32 as u64; rdx = r14; rcx = r9; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = 0x2; rdx = r14; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; r10 = rax; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { r8 = r10; bb = 14; continue; }
            14 => { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = rax.wrapping_add(0x2); crate::simd_ops::wu16(r8, ((rcx as u16)) as u16); r8 = r8.wrapping_add(0x2); fl_l = (((rdx as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r10); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((r15 as u32)) as u32); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu16(rcx, ((rbx as u16)) as u16); bb = 23; continue; }
            18 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_sub(r8)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 20; continue; }
            20 => { rax = ((r14.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rax); bb = 15; continue; }
            22 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = ((rcx as u16)) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) < ((fl_r as u16))) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = ((rbp).wrapping_add(1)) as u64; rbp = ((fl_l as u64)) as u64; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rbp; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(r13)) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = ((rsi).wrapping_sub(rbp)) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = ((rax.wrapping_add(rbp.wrapping_mul(2))) as u64) as u64; rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; r14 = ((rsi.wrapping_add(rbp)) as u64) as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 32; continue; } bb = 26; continue; }
            26 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r15 = ((r14.wrapping_add(r14)) as u64) as u64; r10 = (crate::simd_ops::ru64(rdi)) as u64; r8 = ((rax.wrapping_add(rax)) as u64) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 27; continue; }
            27 => { r8 = ((r9.wrapping_add(0x2u64)) as u32) as u32 as u64; rdx = r15; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = 0x2; rdx = r15; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; r9 = rax; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 29; continue; }
            29 => { r8 = r9; fl_l = r8; fl_r = rax; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 30; continue; }
            30 => { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; crate::simd_ops::wu16(rax.wrapping_add(r8), ((rcx as u16)) as u16); rax = rax.wrapping_add(0x2); fl_l = (((rdx as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 30; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r9); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((r14 as u32)) as u32); bb = 32; continue; }
            32 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((r14 as u32)) as u32); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = ((rax.wrapping_add(rbp.wrapping_mul(2))) as u64) as u64; bb = 34; continue; }
            34 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rbx = rbx.wrapping_add(0x2); crate::simd_ops::wu16(rdx, ((rax as u16)) as u16); rdx = rdx.wrapping_add(0x2); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 35; continue; }
            35 => { rax = rdi; rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            36 => { rcx = (crate::simd_ops::ru64(r10)) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 37; continue; }
            37 => { rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_sub(r8)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 38; continue; }
            38 => { rax = ((r15.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 28; continue; } bb = 39; continue; }
            39 => { crate::simd_ops::wu64(r10.wrapping_add(0x10u64), rax); bb = 31; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c0900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14; const P2: u64 = 0x1411f11e0; const P3: u64 = 0x8; const P4: u64 = 0x14183acc2; const P5: u64 = 0x1411f1520;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = (crate::simd_ops::ru64(r8)) as u64; rax = rdx; crate::simd_ops::wu64(rcx, rdx); rdi = r8; r14 = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r15 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r15 as u32)) as u32); r8 = 0x2; rdx = ((r15.wrapping_add(r15)) as u64) as u64; rcx = rax; rsi = r9; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(r14.wrapping_add(P0))) as u32 as u64; rdx = 0xff; r8 = ((r8).wrapping_add(r8)) as u64; crate::simd_ops::wu64(r14.wrapping_add(P3), rax); rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(P3))) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rcx = ((crate::simd_ops::ru16(rax.wrapping_add(rcx.wrapping_mul(2)))) as u32) as u32 as u64; crate::simd_ops::wu16(rdx.wrapping_add(rbx.wrapping_mul(2)), ((rcx as u16)) as u16); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = r14; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c09d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acbc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; r10 = rdx; rdi = (crate::simd_ops::ru64(r8)) as u64; r13 = r9; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdx = rcx; rdi = ((rdi).wrapping_add(rax)) as u64; rbx = rax; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rbp); rax = 0xffff; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 2; continue; }
            2 => { r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rbx)) as u128; r15 = r14; r15 = ((r15).wrapping_shr(32u32)) as u64; fl_l = ((r15 as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rcx = r14; rcx = ((rcx).wrapping_shr(48u32)) as u64; rbp = ((xmm0 as u64)) as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = rbp; rax = ((rax).wrapping_shr(32u32)) as u64; r12 = ((r12).wrapping_add(0x10u64)) as u64; rax = (((rax as u32)).wrapping_sub((rbp as u32))) as u32 as u64; rcx = ((r14 as u32)) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; r8 = ((rax.wrapping_mul(8)) as u64) as u64; rax = ((rbp as u32)) as u32 as u64; rdx = ((r12.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (((r15 as u16)) as u32) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; r9 = ((r9).wrapping_add(r9)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; r14 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = ((r14.wrapping_add(rbp)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x70u64), ((r15 as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rbp as u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rdx = (crate::simd_ops::ru32(r12.wrapping_add(rax.wrapping_mul(8)))) as u32 as u64; rax = (crate::simd_ops::ru64(r13)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rdx.wrapping_mul(8)), rcx); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = 0xffff; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 7; continue; }
            7 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x40u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c0b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r10 = (crate::simd_ops::ru64(r8)) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = r9; rsi = rdx; rbp = rcx; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); r15 = 0xffff; bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rcx = r8; rcx = ((rcx).wrapping_shr(32u32)) as u64; fl_l = ((rcx as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; r9 = r8; r8 = ((r8 as u32)) as u32 as u64; r9 = ((r9).wrapping_shr(48u32)) as u64; r11 = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r11 as u32)) as u32); rdx = (crate::simd_ops::ru64(rdx.wrapping_add(r9.wrapping_mul(8)))) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P1))) as u64; rdx = (((rcx as u16)) as u32) as u32 as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; fl_l = ((r8).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(P0)))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x10u64), r9); rcx = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), ((rcx as u16)) as u16); rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r8 = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(r8.wrapping_mul(8)), rdx); bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 6; continue; }
            6 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c1290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x18; const P4: u64 = 0x30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rdi = rax; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; r11 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rax = rcx; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rax = ((rax).wrapping_shr(32u32)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r10 = ((rcx as u32)) as u32 as u64; rax = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rcx = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(r11, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi.wrapping_add(P4)) as u64) as u64; rax = ((rdi.wrapping_add(P2)) as u64) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c1410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f11e0; const P1: u64 = 0x14183acbc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 49>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x48u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x148u64)) as u64; r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = (rdx) as u64; r15 = (r8) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r12 as u32)) as u32); rdx = (0x100u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((r12 as u32)) as u32); r8 = ((r12.wrapping_add(0x4u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = (0x20u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rbx) as u64); rax = ((rbx as u32)) as u32 as u64; bb = 3; continue; }
            2 => { rdx = (0x100u32) as u32 as u64; r8 = (0x4u32) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); rax = (0x20u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); bb = 3; continue; }
            3 => { rdx = (0x500u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rax) as u64); r8 = (0x8u32) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); r14 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (0x500u32) as u32 as u64; r8 = ((rax.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); bb = 5; continue; }
            5 => { rdx = (0x600u32) as u32 as u64; r8 = (0x8u32) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (0x600u32) as u32 as u64; r8 = ((rax.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; bb = 7; continue; }
            7 => { r13 = ((crate::simd_ops::ru16(r15)) as u32) as u32 as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rcx = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), (r12) as u64); r8 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rcx as u32)) as u32); rdi = (r12) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (r8) as u64); r9 = (r12) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (r12) as u64); r10 = (r12) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), (r12) as u64); r11 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0xa0u64), ((r12 as u16)) as u16); crate::simd_ops::wu16(rsp.wrapping_add(0x34u64), ((r13 as u16)) as u16); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 74; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (r9) as u64); crate::simd_ops::wu16(rsp.wrapping_add(0x24u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (r11) as u64); bb = 9; continue; }
            9 => { rcx = (((rax as u16)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rcx) as u64); rax = (crate::simd_ops::ru64(r9.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 72; continue; } bb = 10; continue; }
            10 => { rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 72; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 71; continue; } bb = 12; continue; }
            12 => { r14 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r10 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((r14 as u32)) as u32); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 72; continue; } bb = 13; continue; }
            13 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r10 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((r9 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); rax = ((r9.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = ((rdx as u32)) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rcx }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rcx) as u64); r15 = ((rcx.wrapping_mul(8)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { r8 = (0x4u32) as u32 as u64; rdx = (r15) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 16; continue; }
            16 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r10 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); bb = 25; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rcx = ((rdx.wrapping_mul(8)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = ((r15.wrapping_add(rdx)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), (rax) as u64); r10 = (r14) as u64; rax = (r8) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); bb = 25; continue; }
            21 => { r8 = (0x4u32) as u32 as u64; rdx = (r15) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r10 = (r14) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r10 as u32)) as u32); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = (rax) as u64; fl_l = (r12) as u64 as u64; fl_r = (rax) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r10 as u32)) as u32); r8 = ((r9 as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(r12.wrapping_add(rdx))) as u64; crate::simd_ops::wu64(rdx, (rcx) as u64); rdx = ((rdx.wrapping_add(0x8u64)) as u64) as u64; fl_l = (((r8 as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 23; continue; } bb = 24; continue; }
            24 => { r12 = ((crate::simd_ops::ru16(rbp.wrapping_add(0xa0u64))) as u32) as u32 as u64; bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((r9 as u32)) as u32 as u64; r9 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rax.wrapping_mul(8)), (r15) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (r9) as u64); rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rax) as u64); rax = (rcx) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(r14).wrapping_add(0x18058u64))) as u64; r14 = (crate::simd_ops::ru32(rax.wrapping_add(r14).wrapping_add(0x1806cu64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rdx) as u64); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = ((rdx as u32)) as u32 as u64; fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 27; continue; }
            27 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; r8 = ((rcx.wrapping_add(0xc03u64)) as u64) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; fl_l = ((r8).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64)))) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (r8) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (r10) as u64); bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; rax = ((crate::simd_ops::ru16(rax.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)); xmm1 = core::ptr::read_unaligned(y_addr as *const u128); ymm1_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64); xmm2 = core::ptr::read_unaligned(y_addr as *const u128); ymm2_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); rax = (crate::simd_ops::ru64(r8)) as u64; xmm0 = (ymm1_hi) as u128; ymm0_hi = 0; r8 = ((xmm0 as u64)) as u64; r10 = (((r10) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64)))) as u64; xmm0 = (ymm2_hi) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r10) as u64); rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x28u64))) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64)))) as u64; r9 = (((xmm0 >> 64) as u64)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 29; continue; }
            29 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), (rcx) as u64); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), ((rcx as u32)) as u32); rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (r9) as u64); crate::simd_ops::wu64(rbp, (rdx) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0xcu64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), (r10) as u64); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 42; continue; } bb = 30; continue; }
            30 => { r8 = ((rdi as u32)) as u32 as u64; rdi = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; rdx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rdx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 31; continue; }
            31 => { r8 = ((rbx.wrapping_add(0x8u64)) as u32) as u32 as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 32; continue; }
            32 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; bb = 33; continue; }
            33 => { r8 = (0x8u32) as u32 as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 36; continue; } bb = 34; continue; }
            34 => { rdx = (rax) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rbx) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 35; continue; }
            35 => { let y_addr: u64 = rbx; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbx.wrapping_add(rdx); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x20u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbx.wrapping_add(rdx).wrapping_add(0x20u64), (xmm1) as u128); rbx = ((rbx.wrapping_add(0x30u64)) as u64) as u64; fl_l = (((rcx as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 35; continue; } bb = 36; continue; }
            36 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (rax) as u64; bb = 43; continue; }
            37 => { rcx = (crate::simd_ops::ru64(r15)) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 39; continue; }
            39 => { rax = ((rdx.wrapping_add(r8)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 33; continue; } bb = 40; continue; }
            40 => { crate::simd_ops::wu64(r15.wrapping_add(0x10u64), (rax) as u64); rax = (rbx) as u64; bb = 41; continue; }
            41 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; rbx = (rax) as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 43; continue; }
            42 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); bb = 43; continue; }
            43 => { let y_addr: u64 = rbp.wrapping_sub(0x10u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x10u64))) as u128; ymm1_hi = 0; rax = ((r8 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_sub(0x68u64), (r10) as u64)) as u64; fl_r = fl_l; let y_addr: u64 = rbx.wrapping_add(rcx.wrapping_mul(8)); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu128(rbx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64), (xmm1) as u128); bb = 53; continue; }
            44 => { rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rcx) as u64); rax = ((rax).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbp, (r8) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), (r10) as u64); fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 52; continue; } bb = 45; continue; }
            45 => { rcx = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((r11 as u32)) as u32 as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rax) as u64); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r9 = ((rax.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (r9) as u64); fl_l = (r13) as u64 as u64; fl_r = (r13) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 63; continue; } bb = 46; continue; }
            46 => { r8 = ((r13.wrapping_add(0x8u64)) as u32) as u32 as u64; rdx = (r9) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 51; continue; } bb = 47; continue; }
            47 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; bb = 48; continue; }
            48 => { r8 = (0x8u32) as u32 as u64; rdx = (r9) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((r12 as u32)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 51; continue; } bb = 49; continue; }
            49 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (rax) as u64; fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 50; continue; }
            50 => { let y_addr: u64 = rcx.wrapping_add(r8); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rcx; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(r8).wrapping_add(0x20u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), ((xmm1 as u64)) as u64); rcx = ((rcx.wrapping_add(0x28u64)) as u64) as u64; fl_l = (((rdx as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 50; continue; } bb = 51; continue; }
            51 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r13 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); bb = 52; continue; }
            52 => { let y_addr: u64 = rbp.wrapping_sub(0x10u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64) as u128; ymm1_hi = 0; rax = ((r12 as u32)) as u32 as u64; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_sub(0x70u64), (r10) as u64)) as u64; fl_r = fl_l; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; let y_addr: u64 = r13.wrapping_add(rcx.wrapping_mul(8)); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu64(r13.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64), ((xmm1 as u64)) as u64); bb = 53; continue; }
            53 => { r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = ((rsi).wrapping_add(1)) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdx = (rsi) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 54; continue; }
            54 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r13 = ((crate::simd_ops::ru16(rsp.wrapping_add(0x34u64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdi) as u64); rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); r12 = ((crate::simd_ops::ru16(rbp.wrapping_add(0xa0u64))) as u32) as u32 as u64; bb = 55; continue; }
            55 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (r15) as u64); r15 = (crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x50u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x4cu64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbp.wrapping_sub(0x4au64), ((rax as u16)) as u16); fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 62; continue; } bb = 56; continue; }
            56 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; rcx = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rax }) as u64; r14 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); r14 = ((r14).wrapping_shl(4u32)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 67; continue; } bb = 57; continue; }
            57 => { r8 = ((rdi.wrapping_add(0x4u64)) as u32) as u32 as u64; rdx = (r14) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 61; continue; } bb = 58; continue; }
            58 => { r8 = (0x4u32) as u32 as u64; rdx = (r14) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((r15 as u32)) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 61; continue; } bb = 59; continue; }
            59 => { rdx = (rax) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdi) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 60; continue; }
            60 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_add(rdi), (xmm0) as u128); rdi = ((rdi.wrapping_add(0x10u64)) as u64) as u64; fl_l = (((rcx as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 60; continue; } bb = 61; continue; }
            61 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (rax) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), (rax) as u64); bb = 62; continue; }
            62 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x58u64))) as u128; ymm0_hi = 0; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = (r15) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; rax = ((rax).wrapping_add(rax)) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((r15 as u32)) as u32); crate::simd_ops::wu128(rdi.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128); crate::simd_ops::lk_add_u32(r9.wrapping_add(0x28u64), ((rcx as u32)) as u32); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 72; continue; }
            63 => { rcx = (crate::simd_ops::ru64(r15)) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 48; continue; } bb = 64; continue; }
            64 => { rax = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 48; continue; } bb = 65; continue; }
            65 => { rax = ((r9.wrapping_add(rdx)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 48; continue; } bb = 66; continue; }
            66 => { crate::simd_ops::wu64(r15.wrapping_add(0x10u64), (rax) as u64); rax = (r13) as u64; bb = 51; continue; }
            67 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 58; continue; } bb = 68; continue; }
            68 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_sub(0x8u64))) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 58; continue; } bb = 69; continue; }
            69 => { rax = ((r8.wrapping_add(r14)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 58; continue; } bb = 70; continue; }
            70 => { crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), (rax) as u64); rax = (rdi) as u64; bb = 61; continue; }
            71 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x0u64) as u64); bb = 72; continue; }
            72 => { r12 = (r12 & !0xffffu64) | (((((r12 as u16)).wrapping_add(1)) as u16 as u64)); crate::simd_ops::wu16(rbp.wrapping_add(0xa0u64), ((r12 as u16)) as u16); rax = (((r12 as u16)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x24u64), ((rax as u16)) as u16); fl_l = ((r12 as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 73; continue; }
            73 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; bb = 74; continue; }
            74 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; r8 = (0x4u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x8u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu64(r15.wrapping_add(0x38u64), (rbx) as u64); rbx = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; crate::simd_ops::wu32(r15.wrapping_add(0x14u64), ((rdx as u32)) as u32); rdx = (rbx) as u64; crate::simd_ops::wu32(r15.wrapping_add(0x68u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(r15.wrapping_add(0x6cu64), ((rcx as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu32(r15.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu64(r15, (rsi) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu32(r15.wrapping_add(0x2cu64), ((r11 as u32)) as u32); crate::simd_ops::wu64(r15.wrapping_add(0x30u64), (rsi) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x48u64), (r9) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x50u64), (r10) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x58u64), (rsi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rbx) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x60u64), (rax) as u64); rdx = (rdi) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (r15) as u64; rsp = ((rsp).wrapping_add(0x148u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c1cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14; const P2: u64 = 0x1411f11e0; const P3: u64 = 0x8; const P4: u64 = 0x14183acbc;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = rdx;
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = 0x4;
    crate::simd_ops::wu64(rdi, rcx);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rdx = ((rax as u32)) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rax as u32)) as u32);
    rdx = ((rdx).wrapping_shl(4u32)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi.wrapping_add(P3), rax);
    rcx = rax;
    r8 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c3020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f11e0; const P1: u64 = 0x1411f2490;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r13 = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(r9)) as u64; rdi = r9; crate::simd_ops::wu64(rax.wrapping_sub(0x48u64), rdx); r15 = rdx; crate::simd_ops::wu64(rax.wrapping_sub(0x40u64), r10); r12 = rcx; crate::simd_ops::wu32(rax.wrapping_sub(0x38u64), ((r10 as u32)) as u32); rbp = ((r13.wrapping_mul(8).wrapping_add(0x10u64)) as u64) as u64; rsi = ((r10 as u32)) as u32 as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = ((r14.wrapping_mul(4)) as u64) as u64; rcx = r15; rdx = rbx; r8 = ((r10.wrapping_add(0x4u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((rax.wrapping_add(0x4u64)) as u32) as u32 as u64; rdx = rbx; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rax; bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); bb = 4; continue; }
            4 => { xmm1 = (crate::simd_ops::ru128(rdi)) as u128; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = (((xmm1 >> 64) as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r14 as u32)) as u32); rdi = ((rcx.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rcx = rbp.wrapping_sub(0x1); r15 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r15 = ((r15).wrapping_shl(4u32)) as u64; r15 = ((r15).wrapping_add(crate::simd_ops::ru64(r8))) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = ((rax).wrapping_sub(1)) as u64; fl_l = ((!(rax)) & (rcx)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = ((r14 as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = rdx; fl_r = rcx; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rcx } else { rdx }) as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; bb = 8; continue; }
            8 => { rax = ((rsi as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(0x2u64)) as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(r10.wrapping_add(rax.wrapping_mul(4)), ((rbp as u32)) as u32); rax = r13; rax = (((rax) as i64).wrapping_mul((crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = r12; crate::simd_ops::wu32(r12.wrapping_add(0x10u64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(r12.wrapping_add(0x18u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(r12, r15); crate::simd_ops::wu64(r12.wrapping_add(0x8u64), r10); crate::simd_ops::wu32(r12.wrapping_add(0x14u64), ((r14 as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c3190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142489ee0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    r11 = r9;
    r8 = (((rdx as u16)) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((((rax as u32)) as i32 as i64) as u64) as u64;
    fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r11.wrapping_add(0x8u64)))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(P0)))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c31f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P4))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c3250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c3280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x28; const P2: u64 = 0x30; const P3: u64 = 0x38; const P4: u64 = 0x8; const P5: u64 = 0x18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), r9);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c32c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x8; const P2: u64 = 0x20; const P3: u64 = 0x18; const P4: u64 = 0x30; const P5: u64 = 0x28; const P6: u64 = 0x1420c43d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = rsp.wrapping_add(0x40);
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128;
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c3b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x28; const P2: u64 = 0x8; const P3: u64 = 0x18; const P4: u64 = 0x38; const P5: u64 = 0x30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), r9);
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rdx);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c48a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x28; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x8; const P5: u64 = 0x20; const P6: u64 = 0xfffffffffffffff8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rax)) as u64; rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rcx); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x14u64))) as u32 as u64; bb = 4; continue; }
            4 => { r11 = r9; r10 = r8; r11 = ((r11).wrapping_shr(1u32)) as u64; r10 = ((r10).wrapping_sub(r11)) as u64; r10 = ((r10).wrapping_add(r9)) as u64; r9 = ((r11.wrapping_add(r8)) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(r9.wrapping_mul(4)))) as u32 as u64; r9 = r11; r8 = (if (((fl_l as u32)) >= ((fl_r as u32))) { r10 } else { r8 }) as u64; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { r10 = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_add(r8)) as u64; r9 = (((rcx as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)))) as u64; crate::simd_ops::lk_xor_u64(r10.wrapping_add(P6), (rcx) as u64); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; bb = 7; continue; }
            7 => { rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c53f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a082f8; const P1: u64 = 0x1433ddb40; const P2: u64 = 0x142407c80; const P3: u64 = 0x142485aa0; const P4: u64 = 0x14340e320; const P5: u64 = 0x1420c5340; const P6: u64 = 0x1411f30e0; const P7: u64 = 0x1420c52b0; const P8: u64 = 0x1411f3240; const P9: u64 = 0x142486460; const P10: u64 = 0x142645580; const P11: u64 = 0x1424854f0; const P12: u64 = 0x142645868; const P13: u64 = 0x1420afc40; const P14: u64 = 0x1420af9a0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbp = rcx; rsi = (crate::simd_ops::ru64(P0)) as u64; rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x2cu64) as u64); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rcx = ((rcx).wrapping_add(0x40u64)) as u64; rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0x1; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r12); rax = P4; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x18u64), (xmm0) as u128); rcx = 0x3; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; let cx_src: u8 = (rcx as u8); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u8(rbx.wrapping_add(0x4u64), fl_l as u8, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = (rax & !0xffu64) | ((((fl_r as u8)) as u8 as u64)); } if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rax, (0xffffu32) as u32)) as u32 as u64; fl_r = fl_l; bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rsp.wrapping_add(0x3au64), (0x1u8) as u8); xmm0 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rax = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = P5; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rax = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = P7; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x3bu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = 0x4; bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x3u8) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; let cx_src: u8 = (rdx as u8); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u8(rbx.wrapping_add(0x4u64), fl_l as u8, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = (rax & !0xffu64) | ((((fl_r as u8)) as u8 as u64)); } if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rax, (0xffff0001u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = 0x2; r15 = 0xfff8; bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x6u64))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_shr(3u32)) as u16 as u64)); fl_l = (crate::simd_ops::ru16(rsp.wrapping_add(0x38u64))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 18; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = (rax & !0xffu64) | (((0x4u8) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; let cx_src: u8 = (r14 as u8); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u8(rbx.wrapping_add(0x4u64), fl_l as u8, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = (rax & !0xffu64) | ((((fl_r as u8)) as u8 as u64)); } if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::lk_sub_u32(rax, (1) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (((r14 as u8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; let cx_src: u8 = (r12 as u8); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u8(rbx.wrapping_add(0x4u64), fl_l as u8, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = (rax & !0xffu64) | ((((fl_r as u8)) as u8 as u64)); } rcx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x6u64))) as u32) as u32 as u64; rax = (((rcx as u16)) as u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((r15 as u16)))) as u16 as u64)); rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add(0x8u16)) as u16 as u64)); rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) & (0x7u16))) as u16 as u64)); rax = (rax & !0xffffu64) | ((((((rax as u16)) | ((rcx as u16)))) as u16 as u64)); crate::simd_ops::wu16(rbx.wrapping_add(0x6u64), ((rax as u16)) as u16); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 15; continue; }
            15 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1008u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            16 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 17; continue; }
            17 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            18 => { rdx = (((crate::simd_ops::ru32(rbp)) as i32 as i64) as u64) as u64; r8 = P13; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c5640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f11e0; const P1: u64 = 0x143a082f8; const P2: u64 = 0x142407c80; const P3: u64 = 0x142485aa0; const P4: u64 = 0x1433ddc50; const P5: u64 = 0x142486460; const P6: u64 = 0x142645580;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r14 = rdx; rbx = r8; rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rsi = rcx; rbp = ((rdx as u32)) as u32 as u64; r8 = 0x8; rcx = r14; rdi = r9; rdx = ((rdx.wrapping_mul(8)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rbx)) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r15 = rax; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rdi)) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64) as u128; ymm0_hi = 0; rdi = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x58u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), ((xmm0 as u64)) as u64); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rax as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x20); rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rcx = ((rcx).wrapping_sub(0xffffffffffffff80u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = 0x1; let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = P4; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (0x0u64) as u64); let y_addr: u64 = rax.wrapping_add(0x18u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rax.wrapping_add(0x38u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rcx); rax = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1008u64))) as u64; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsi, r14); rax = rsi; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r15); crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), ((rbp as u32)) as u32); crate::simd_ops::wu128(rsi.wrapping_add(0x18u64), (xmm0) as u128); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c57a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    rax = rcx;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c57e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14183acc8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r9 = r8;
    rax = rdx;
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = ((rax.wrapping_add(r9.wrapping_mul(4))) as u64) as u64;
    r8 = ((r8.wrapping_mul(4)) as u64) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c5810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14183acc8;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rcx = ((r8.wrapping_add(r9.wrapping_mul(4))) as u64) as u64;
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    fl_l = ((r8).wrapping_shl(2u32)) as u64;
    r8 = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c5830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a082f8; const P1: u64 = 0x1433dde48; const P2: u64 = 0x142407c80; const P3: u64 = 0x142485aa0; const P4: u64 = 0x1420c57e0; const P5: u64 = 0x142486460; const P6: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = r8; r15 = rcx; rbx = (crate::simd_ops::ru64(rdx)) as u64; rbp = (crate::simd_ops::ru64(P0)) as u64; r12 = 0; fl_l = 0; fl_r = 0; rsi = ((r12 as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r13 = P1; bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdi).wrapping_add(0x48u64))) as u64; rcx = ((rcx).wrapping_sub(0xffffffffffffff80u64)) as u64; rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = 0x1; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rax, ((rcx as u32)) as u32); rcx = (tmp_xchg) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r12); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r13); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (0x3u64) as u64); rcx = P4; crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rax.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rax.wrapping_add(0x38u64), rsi); rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::lk_add_u32(rcx, (1) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x1008u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = 0x3; bb = 5; continue; }
            5 => { rax = ((rax).wrapping_sub(1)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x8u64)))) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c5950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xfffffffffffffff8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; xmm2 = (crate::simd_ops::ru128(rcx)) as u128; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = ((xmm2 as u64)) as u64; let y_addr: u64 = rsp; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rcx = ((rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (((xmm2 >> 64) as u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; bb = 2; continue; }
            2 => { r11 = (crate::simd_ops::ru32(rax)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { r10 = r8; r9 = rdx; r10 = ((r10).wrapping_shr(1u32)) as u64; r9 = ((r9).wrapping_sub(r10)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(rdx)) as u64) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; r8 = r10; rdx = (if (((fl_l as u32)) >= ((fl_r as u32))) { r9 } else { rdx }) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(rdx)) as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P1), (0xffffffffffffffffu64) as u64); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c5a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xfffffffffffffff8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; let y_addr: u64 = r8; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; bb = 2; continue; }
            2 => { r11 = (crate::simd_ops::ru32(rax)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { r10 = r8; r9 = rdx; r10 = ((r10).wrapping_shr(1u32)) as u64; r9 = ((r9).wrapping_sub(r10)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; r8 = ((r10.wrapping_add(rdx)) as u64) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; r8 = r10; rdx = (if (((fl_l as u32)) >= ((fl_r as u32))) { r9 } else { rdx }) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(rdx)) as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P1), (0xffffffffffffffffu64) as u64); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c5ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411f33a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128; ymm0_hi = 0;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    xmm1 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128);
    let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rsp.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    r9 = rsp.wrapping_add(0x20);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c5b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rdx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    fl_l = rax; fl_r = rdx;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
    loop { r10 = (crate::simd_ops::ru64(rax)) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; r9 = r10; r9 = ((r9).wrapping_shr(32u32)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; r11 = ((r8.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; r9 = (((r10 as u16)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)))) as u64; crate::simd_ops::lk_xor_u64(r11, (rcx) as u64); fl_l = rax; fl_r = rdx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c5bf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x20; const P3: u64 = 0xffffffffffffffe0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    r8 = ((r8).wrapping_shl(3u32)) as u64;
    r8 = ((r8).wrapping_shr(5u32)) as u64;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    loop { rax = ((rax.wrapping_add(P2)) as u64) as u64; let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rdx = ((rdx.wrapping_add(P2)) as u64) as u64; let y_addr: u64 = rax.wrapping_add(P3); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c5c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r10 = rdx;
    rax = r9;
    rax = ((rax).wrapping_sub(r8)) as u64;
    rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 };
    rax = ((rax).wrapping_sub(rdx)) as u64;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(r9.wrapping_mul(8)))) as u64;
    rax = ((((rax) as i64).wrapping_shr(1u32) as u64)) as u64;
    fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    r8 = ((r10.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    rax = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)))) as u64;
    crate::simd_ops::wu64(r10.wrapping_add(r9.wrapping_mul(8)), rax);
    crate::simd_ops::wu64(r8, rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c65e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r11 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; return rax; }
    loop { r10 = rdx; r9 = rax; r10 = ((r10).wrapping_shr(1u32)) as u64; r9 = ((r9).wrapping_sub(r10)) as u64; r9 = ((r9).wrapping_add(rdx)) as u64; rdx = r10; rcx = ((r10.wrapping_add(rax)) as u64) as u64; fl_l = r8; fl_r = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { r9 } else { rax }) as u64; fl_l = r10; fl_r = r10; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    fl_l = ((rax).wrapping_sub(1)) as u64;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c6620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    r11 = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_l = ((rax).wrapping_sub(1)) as u64; rax = ((fl_l as u64)) as u64; return rax; }
    loop { r10 = rdx; r9 = rax; r10 = ((r10).wrapping_shr(1u32)) as u64; r9 = ((r9).wrapping_sub(r10)) as u64; r9 = ((r9).wrapping_add(rdx)) as u64; rdx = r10; rcx = ((r10.wrapping_add(rax)) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { r9 } else { rax }) as u64; fl_l = r10; fl_r = r10; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    fl_l = ((rax).wrapping_sub(1)) as u64;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c66a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x142489ee0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r8 = (((rdx as u16)) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (rax as u32 as i32 as i64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c7ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; bb = 3; continue; }
            2 => { r10 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::lk_add_u32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1), ((rdx as u32)) as u32); rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r8)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 7; continue; }
            7 => { r8 = rcx; rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            8 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c80b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x143428840; const P2: u64 = 0x1434281b0; const P3: u64 = 0x2; const P4: u64 = 0x1434281d0; const P5: u64 = 0x143428760; const P6: u64 = 0x1434280c0; const P7: u64 = 0x143428120; const P8: u64 = 0x143428240; const P9: u64 = 0x1434284c0; const P10: u64 = 0x1434282c0; const P11: u64 = 0x1434288d0; const P12: u64 = 0x143428280; const P13: u64 = 0x143428460;
    let (mut rax, mut r11) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; xmm3 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rax = ((rax).wrapping_add(rax)) as u64; xmm6 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm9) as u128); xmm9 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P3))) as u32) as u32 as u64; crate::simd_ops::wu128(r11.wrapping_sub(0x58u64), (xmm10) as u128); rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(r11.wrapping_sub(0x68u64), (xmm11) as u128); crate::simd_ops::wu128(rsp, (xmm12) as u128); xmm12 = (crate::simd_ops::ru128(P4)) as u128; xmm1 = crate::simd_ops::vdpps(xmm12, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0xffu8); xmm8 = (xmm1) & (crate::simd_ops::ru128(P5)); xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x58u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x68u64))) as u128; xmm5 = (xmm8) ^ (xmm1); xmm3 = crate::simd_ops::vfmadd_ps(xmm5, crate::simd_ops::ru128(P6), xmm3, false, false); xmm9 = crate::simd_ops::vbroadcastss((xmm9 as u32)); xmm4 = crate::simd_ops::vsubps(xmm9, crate::simd_ops::ru128(P7)); xmm2 = crate::simd_ops::vsubps(xmm9, crate::simd_ops::ru128(P8)); xmm1 = crate::simd_ops::vmulps(xmm4, xmm9); xmm7 = crate::simd_ops::vmulps(xmm2, xmm1); xmm1 = (crate::simd_ops::ru128(P9)) as u128; xmm1 = crate::simd_ops::vfmadd_ps(xmm5, crate::simd_ops::ru128(P10), xmm1, true, false); xmm2 = (crate::simd_ops::ru128(P11)) as u128; xmm6 = crate::simd_ops::vfmadd_ps(xmm5, xmm3, xmm6, false, false); xmm3 = (crate::simd_ops::ru128(P12)) as u128; xmm2 = crate::simd_ops::vfmadd_ps(xmm5, xmm1, xmm2, false, false); xmm1 = (crate::simd_ops::ru128(P13)) as u128; xmm3 = crate::simd_ops::vfmadd_ps(xmm5, xmm2, xmm3, false, false); xmm4 = crate::simd_ops::vmulps(xmm4, xmm4); xmm6 = crate::simd_ops::vfmadd_ps(xmm3, xmm4, xmm6, false, false); xmm9 = crate::simd_ops::vfmadd_ps(xmm6, xmm7, xmm9, false, false); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm0 = (xmm9) ^ (xmm8); xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x38u64))) as u128; xmm12 = crate::simd_ops::vfmadd_ps(xmm9, xmm12, xmm12, true, false); xmm12 = crate::simd_ops::vfmadd_ps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), xmm12, false, false); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x48u64))) as u128; rax = ((rax).wrapping_add(rax)) as u64; xmm0 = crate::simd_ops::vdpps(xmm12, xmm12, 0xffu8); xmm4 = crate::simd_ops::vrsqrtps(xmm0); xmm0 = crate::simd_ops::vmulps(xmm4, xmm0); xmm2 = crate::simd_ops::vmulps(xmm0, xmm4); xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P7)); xmm3 = crate::simd_ops::vsubps(xmm1, xmm2); xmm2 = crate::simd_ops::vmulps(xmm3, xmm0); xmm1 = crate::simd_ops::vmulps(xmm2, xmm12); xmm12 = (crate::simd_ops::ru128(rsp)) as u128; crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x78u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c8210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x4; const P2: u64 = 0x6; const P3: u64 = 0x143428840; const P4: u64 = 0x1434281b0; const P5: u64 = 0x143428760; const P6: u64 = 0x1434280c0; const P7: u64 = 0x143428120; const P8: u64 = 0x143428240; const P9: u64 = 0x143428280; const P10: u64 = 0x1434284c0; const P11: u64 = 0x1434282c0; const P12: u64 = 0x1434288d0; const P13: u64 = 0x143428460;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x68u64)) as u64; r9 = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64; r9 = ((r9).wrapping_add(r9)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; xmm3 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); rax = ((rax).wrapping_add(rax)) as u64; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(r9.wrapping_mul(8)))) as u128; xmm2 = crate::simd_ops::vdpps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rdx.wrapping_mul(8))), 0xffu8); xmm6 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); xmm8 = (xmm2) & (crate::simd_ops::ru128(P5)); xmm5 = (xmm8) ^ (xmm2); xmm3 = crate::simd_ops::vfmadd_ps(xmm5, crate::simd_ops::ru128(P6), xmm3, false, false); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm9) as u128); xmm9 = crate::simd_ops::vbroadcastss(crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))); xmm4 = crate::simd_ops::vsubps(xmm9, crate::simd_ops::ru128(P7)); xmm2 = crate::simd_ops::vsubps(xmm9, crate::simd_ops::ru128(P8)); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm10) as u128); crate::simd_ops::wu128(rsp, (xmm11) as u128); rax = ((rax).wrapping_add(rax)) as u64; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm11 = (crate::simd_ops::ru128(rsp)) as u128; xmm6 = crate::simd_ops::vfmadd_ps(xmm5, xmm3, xmm6, false, false); xmm3 = (crate::simd_ops::ru128(P9)) as u128; xmm1 = crate::simd_ops::vmulps(xmm4, xmm9); xmm7 = crate::simd_ops::vmulps(xmm2, xmm1); xmm1 = (crate::simd_ops::ru128(P10)) as u128; xmm1 = crate::simd_ops::vfmadd_ps(xmm5, crate::simd_ops::ru128(P11), xmm1, true, false); xmm2 = (crate::simd_ops::ru128(P12)) as u128; xmm2 = crate::simd_ops::vfmadd_ps(xmm5, xmm1, xmm2, false, false); xmm1 = (crate::simd_ops::ru128(P13)) as u128; xmm3 = crate::simd_ops::vfmadd_ps(xmm5, xmm2, xmm3, false, false); xmm5 = (xmm0) as u128; xmm4 = crate::simd_ops::vmulps(xmm4, xmm4); xmm6 = crate::simd_ops::vfmadd_ps(xmm3, xmm4, xmm6, false, false); xmm9 = crate::simd_ops::vfmadd_ps(xmm6, xmm7, xmm9, false, false); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm0 = (xmm9) ^ (xmm8); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm5 = crate::simd_ops::vfmadd_ps(xmm9, xmm5, xmm5, true, false); xmm5 = crate::simd_ops::vfmadd_ps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rdx.wrapping_mul(8))), xmm5, false, false); xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm0 = crate::simd_ops::vdpps(xmm5, xmm5, 0xffu8); xmm4 = crate::simd_ops::vrsqrtps(xmm0); xmm0 = crate::simd_ops::vmulps(xmm4, xmm0); xmm2 = crate::simd_ops::vmulps(xmm0, xmm4); xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P7)); xmm3 = crate::simd_ops::vsubps(xmm1, xmm2); xmm2 = crate::simd_ops::vmulps(xmm3, xmm0); xmm1 = crate::simd_ops::vmulps(xmm2, xmm5); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c8360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x143428460; const P2: u64 = 0x143428120;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm1 = (crate::simd_ops::ru128(P1)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm5 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm0 = crate::simd_ops::vdpps(xmm5, xmm5, 0xffu8);
    xmm4 = crate::simd_ops::vrsqrtps(xmm0);
    xmm0 = crate::simd_ops::vmulps(xmm4, xmm0);
    xmm2 = crate::simd_ops::vmulps(xmm0, xmm4);
    xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P2));
    xmm3 = crate::simd_ops::vsubps(xmm1, xmm2);
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm1 = crate::simd_ops::vmulps(xmm3, xmm0);
    xmm2 = crate::simd_ops::vmulps(xmm1, xmm5);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c83f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    crate::simd_ops::wu128(rsp, (xmm6) as u128);
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    xmm3 = crate::simd_ops::vpermilps(xmm0, 0x11u8);
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = crate::simd_ops::vpermilps(xmm0, 0xbbu8);
    xmm5 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm6 = crate::simd_ops::vpermilps(xmm5, 0x1bu8);
    xmm0 = crate::simd_ops::vmulps(xmm2, xmm6);
    xmm1 = crate::simd_ops::vmulps(xmm3, xmm5);
    xmm4 = crate::simd_ops::vhsubps(xmm1, xmm0);
    xmm0 = crate::simd_ops::vmulps(xmm3, xmm6);
    xmm6 = (crate::simd_ops::ru128(rsp)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = crate::simd_ops::vmulps(xmm2, xmm5);
    xmm3 = crate::simd_ops::vhaddps(xmm2, xmm0);
    xmm1 = crate::simd_ops::vblendps(xmm3, xmm4, 0xcu8);
    xmm0 = crate::simd_ops::vshufps(xmm4, xmm3, 0xb1u8);
    xmm1 = crate::simd_ops::vaddsubps(xmm1, xmm0);
    xmm2 = crate::simd_ops::vpermilps(xmm1, 0x9cu8);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c8480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x143427b00; const P2: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm1 = (crate::simd_ops::ru128(P1)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    crate::simd_ops::wu128(rsp, (xmm6) as u128);
    xmm6 = (xmm1) ^ (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm5 = crate::simd_ops::vpermilps(xmm6, 0x1bu8);
    xmm3 = crate::simd_ops::vpermilps(crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0x11u8);
    xmm2 = crate::simd_ops::vpermilps(crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0xbbu8);
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm1 = crate::simd_ops::vmulps(xmm6, xmm3);
    xmm0 = crate::simd_ops::vmulps(xmm2, xmm5);
    xmm4 = crate::simd_ops::vhsubps(xmm1, xmm0);
    xmm2 = crate::simd_ops::vmulps(xmm6, xmm2);
    xmm6 = (crate::simd_ops::ru128(rsp)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = crate::simd_ops::vmulps(xmm3, xmm5);
    xmm3 = crate::simd_ops::vhaddps(xmm2, xmm0);
    xmm1 = crate::simd_ops::vblendps(xmm3, xmm4, 0xcu8);
    xmm0 = crate::simd_ops::vshufps(xmm4, xmm3, 0xb1u8);
    xmm1 = crate::simd_ops::vaddsubps(xmm1, xmm0);
    xmm2 = crate::simd_ops::vpermilps(xmm1, 0x9cu8);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c8510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428120; const P1: u64 = 0x1426fd060; const P2: u64 = 0x1426fcaa0; const P3: u64 = 0x1426fce00; const P4: u64 = 0x1426fcc90; const P5: u64 = 0x1426fcfc0; const P6: u64 = 0x1426fd020; const P7: u64 = 0x1426fccc0; const P8: u64 = 0x1426fcf60; const P9: u64 = 0x1426fcb90; const P10: u64 = 0x1426fccd0; const P11: u64 = 0x1426fcb00; const P12: u64 = 0x1426fcd70; const P13: u64 = 0x1426fd070; const P14: u64 = 0x1426fcd60; const P15: u64 = 0x1426fcce0; const P16: u64 = 0x1426fcbf0; const P17: u64 = 0x1426fcb70; const P18: u64 = 0x1426fd090;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm9) as u128); xmm1 = crate::simd_ops::vinsertps(xmm1, ((crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))) as u128), 0x20u8); xmm1 = crate::simd_ops::vinsertps(xmm1, ((crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))) as u128), 0x30u8); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm10) as u128); rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp, (xmm11) as u128); xmm11 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P0)); xmm3 = (xmm11) & (crate::simd_ops::ru128(P1)); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P2)); xmm2 = crate::simd_ops::vcvttps2dq(xmm1); xmm0 = crate::simd_ops::vpaddd(xmm2, crate::simd_ops::ru128(P3)); xmm7 = (xmm0) & (crate::simd_ops::ru128(P4)); xmm0 = (xmm7) & (crate::simd_ops::ru128(P5)); xmm1 = (xmm7) & (crate::simd_ops::ru128(P6)); xmm10 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm0 = 0u128; xmm9 = crate::simd_ops::vpcmpeqd(xmm1, xmm0); xmm5 = crate::simd_ops::vcvtdq2ps(xmm7); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P7)); xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P8)); xmm2 = crate::simd_ops::vaddps(xmm0, xmm3); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P9)); xmm3 = crate::simd_ops::vaddps(xmm2, xmm1); xmm1 = crate::simd_ops::vpsubd(xmm7, crate::simd_ops::ru128(P6)); xmm5 = crate::simd_ops::vaddps(xmm3, xmm0); xmm0 = (!(xmm1)) & (crate::simd_ops::ru128(P5)); xmm3 = crate::simd_ops::vmulps(xmm5, xmm5); xmm8 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P10)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P11)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P12)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P13)); xmm2 = crate::simd_ops::vmulps(xmm0, xmm3); xmm0 = crate::simd_ops::vsubps(xmm2, xmm1); xmm7 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P14)); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P15)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P16)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P17)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm2 = crate::simd_ops::vmulps(xmm0, xmm5); xmm0 = (xmm11) & (crate::simd_ops::ru128(P18)); xmm11 = (crate::simd_ops::ru128(rsp)) as u128; xmm6 = crate::simd_ops::vaddps(xmm2, xmm5); xmm5 = (!(xmm9)) & (xmm7); xmm3 = (xmm6) & (xmm9); xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = crate::simd_ops::vaddps(xmm5, xmm3); xmm2 = (xmm0) ^ (xmm10); xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm0 = crate::simd_ops::vsubps(xmm7, xmm5); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm4 = (xmm2) ^ (xmm1); xmm3 = crate::simd_ops::vsubps(xmm6, xmm3); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm1 = crate::simd_ops::vaddps(xmm3, xmm0); xmm2 = (xmm1) ^ (xmm8); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm5 = crate::simd_ops::vblendps(xmm2, xmm4, 0x7u8); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm5) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c86b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428120; const P1: u64 = 0x1426fd060; const P2: u64 = 0x1426fcaa0; const P3: u64 = 0x1426fce00; const P4: u64 = 0x1426fcc90; const P5: u64 = 0x1426fcfc0; const P6: u64 = 0x1426fd020; const P7: u64 = 0x1426fccc0; const P8: u64 = 0x1426fcf60; const P9: u64 = 0x1426fcb90; const P10: u64 = 0x1426fccd0; const P11: u64 = 0x1426fcb00; const P12: u64 = 0x1426fcd70; const P13: u64 = 0x1426fd070; const P14: u64 = 0x1426fcd60; const P15: u64 = 0x1426fcce0; const P16: u64 = 0x1426fcbf0; const P17: u64 = 0x1426fcb70; const P18: u64 = 0x1426fd090;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm9) as u128); xmm1 = crate::simd_ops::vinsertps(xmm1, ((crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))) as u128), 0x10u8); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm1 = (xmm1 & 0xffff_ffff_ffff_ffffu128) | ((xmm1 & 0xffff_ffff_ffff_ffffu128) << 64); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm10) as u128); rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp, (xmm11) as u128); xmm11 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P0)); xmm3 = (xmm11) & (crate::simd_ops::ru128(P1)); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P2)); xmm2 = crate::simd_ops::vcvttps2dq(xmm1); xmm0 = crate::simd_ops::vpaddd(xmm2, crate::simd_ops::ru128(P3)); xmm7 = (xmm0) & (crate::simd_ops::ru128(P4)); xmm0 = (xmm7) & (crate::simd_ops::ru128(P5)); xmm1 = (xmm7) & (crate::simd_ops::ru128(P6)); xmm10 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm0 = 0u128; xmm9 = crate::simd_ops::vpcmpeqd(xmm1, xmm0); xmm5 = crate::simd_ops::vcvtdq2ps(xmm7); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P7)); xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P8)); xmm2 = crate::simd_ops::vaddps(xmm0, xmm3); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P9)); xmm3 = crate::simd_ops::vaddps(xmm2, xmm1); xmm1 = crate::simd_ops::vpsubd(xmm7, crate::simd_ops::ru128(P6)); xmm5 = crate::simd_ops::vaddps(xmm3, xmm0); xmm0 = (!(xmm1)) & (crate::simd_ops::ru128(P5)); xmm3 = crate::simd_ops::vmulps(xmm5, xmm5); xmm8 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P10)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P11)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P12)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P13)); xmm2 = crate::simd_ops::vmulps(xmm0, xmm3); xmm0 = crate::simd_ops::vsubps(xmm2, xmm1); xmm7 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P14)); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P15)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P16)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P17)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm2 = crate::simd_ops::vmulps(xmm0, xmm5); xmm0 = (xmm11) & (crate::simd_ops::ru128(P18)); xmm11 = (crate::simd_ops::ru128(rsp)) as u128; xmm6 = crate::simd_ops::vaddps(xmm2, xmm5); xmm5 = (!(xmm9)) & (xmm7); xmm3 = (xmm6) & (xmm9); xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = crate::simd_ops::vaddps(xmm5, xmm3); xmm2 = (xmm0) ^ (xmm10); xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm0 = crate::simd_ops::vsubps(xmm7, xmm5); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm4 = (xmm2) ^ (xmm1); xmm3 = crate::simd_ops::vsubps(xmm6, xmm3); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm1 = crate::simd_ops::vaddps(xmm3, xmm0); xmm2 = (xmm1) ^ (xmm8); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm5 = crate::simd_ops::vblendps(xmm2, xmm4, 0x7u8); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm5) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c8850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428120; const P1: u64 = 0x1426fd060; const P2: u64 = 0x1426fcaa0; const P3: u64 = 0x1426fce00; const P4: u64 = 0x1426fcc90; const P5: u64 = 0x1426fcfc0; const P6: u64 = 0x1426fd020; const P7: u64 = 0x1426fccc0; const P8: u64 = 0x1426fcf60; const P9: u64 = 0x1426fcb90; const P10: u64 = 0x1426fccd0; const P11: u64 = 0x1426fcb00; const P12: u64 = 0x1426fcd70; const P13: u64 = 0x1426fd070; const P14: u64 = 0x1426fcd60; const P15: u64 = 0x1426fcce0; const P16: u64 = 0x1426fcbf0; const P17: u64 = 0x1426fcb70; const P18: u64 = 0x1426fd090;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm9) as u128); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm1 = crate::simd_ops::vblendps(xmm1, xmm0, 0x1u8); xmm1 = crate::simd_ops::vinsertps(xmm1, xmm0, 0x30u8); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm10) as u128); rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp, (xmm11) as u128); xmm11 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P0)); xmm3 = (xmm11) & (crate::simd_ops::ru128(P1)); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P2)); xmm2 = crate::simd_ops::vcvttps2dq(xmm1); xmm0 = crate::simd_ops::vpaddd(xmm2, crate::simd_ops::ru128(P3)); xmm7 = (xmm0) & (crate::simd_ops::ru128(P4)); xmm0 = (xmm7) & (crate::simd_ops::ru128(P5)); xmm1 = (xmm7) & (crate::simd_ops::ru128(P6)); xmm10 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm0 = 0u128; xmm9 = crate::simd_ops::vpcmpeqd(xmm1, xmm0); xmm5 = crate::simd_ops::vcvtdq2ps(xmm7); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P7)); xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P8)); xmm2 = crate::simd_ops::vaddps(xmm0, xmm3); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P9)); xmm3 = crate::simd_ops::vaddps(xmm2, xmm1); xmm1 = crate::simd_ops::vpsubd(xmm7, crate::simd_ops::ru128(P6)); xmm5 = crate::simd_ops::vaddps(xmm3, xmm0); xmm0 = (!(xmm1)) & (crate::simd_ops::ru128(P5)); xmm3 = crate::simd_ops::vmulps(xmm5, xmm5); xmm8 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P10)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P11)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P12)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P13)); xmm2 = crate::simd_ops::vmulps(xmm0, xmm3); xmm0 = crate::simd_ops::vsubps(xmm2, xmm1); xmm7 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P14)); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P15)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P16)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P17)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm2 = crate::simd_ops::vmulps(xmm0, xmm5); xmm0 = (xmm11) & (crate::simd_ops::ru128(P18)); xmm11 = (crate::simd_ops::ru128(rsp)) as u128; xmm6 = crate::simd_ops::vaddps(xmm2, xmm5); xmm5 = (!(xmm9)) & (xmm7); xmm3 = (xmm6) & (xmm9); xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = crate::simd_ops::vaddps(xmm5, xmm3); xmm2 = (xmm0) ^ (xmm10); xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm0 = crate::simd_ops::vsubps(xmm7, xmm5); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm4 = (xmm2) ^ (xmm1); xmm3 = crate::simd_ops::vsubps(xmm6, xmm3); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm1 = crate::simd_ops::vaddps(xmm3, xmm0); xmm2 = (xmm1) ^ (xmm8); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm5 = crate::simd_ops::vblendps(xmm2, xmm4, 0x7u8); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm5) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c8a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428120; const P1: u64 = 0x1426fd060; const P2: u64 = 0x1426fcaa0; const P3: u64 = 0x1426fce00; const P4: u64 = 0x1426fcc90; const P5: u64 = 0x1426fcfc0; const P6: u64 = 0x1426fd020; const P7: u64 = 0x1426fccc0; const P8: u64 = 0x1426fcf60; const P9: u64 = 0x1426fcb90; const P10: u64 = 0x1426fccd0; const P11: u64 = 0x1426fcb00; const P12: u64 = 0x1426fcd70; const P13: u64 = 0x1426fd070; const P14: u64 = 0x1426fcd60; const P15: u64 = 0x1426fcce0; const P16: u64 = 0x1426fcbf0; const P17: u64 = 0x1426fcb70; const P18: u64 = 0x1426fd090;
    let (mut rax, mut r11) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm9) as u128); xmm1 = crate::simd_ops::vbroadcastss(crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(8)))); rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; crate::simd_ops::wu128(r11.wrapping_sub(0x58u64), (xmm10) as u128); rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(r11.wrapping_sub(0x68u64), (xmm11) as u128); crate::simd_ops::wu128(rsp, (xmm12) as u128); xmm12 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P0)); xmm3 = (xmm12) & (crate::simd_ops::ru128(P1)); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P2)); xmm2 = crate::simd_ops::vcvttps2dq(xmm1); xmm0 = crate::simd_ops::vpaddd(xmm2, crate::simd_ops::ru128(P3)); xmm7 = (xmm0) & (crate::simd_ops::ru128(P4)); xmm0 = (xmm7) & (crate::simd_ops::ru128(P5)); xmm1 = (xmm7) & (crate::simd_ops::ru128(P6)); xmm11 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm0 = 0u128; xmm9 = crate::simd_ops::vpcmpeqd(xmm1, xmm0); xmm5 = crate::simd_ops::vcvtdq2ps(xmm7); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P7)); xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P8)); xmm2 = crate::simd_ops::vaddps(xmm0, xmm3); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P9)); xmm3 = crate::simd_ops::vaddps(xmm2, xmm1); xmm1 = crate::simd_ops::vpsubd(xmm7, crate::simd_ops::ru128(P6)); xmm5 = crate::simd_ops::vaddps(xmm3, xmm0); xmm0 = (!(xmm1)) & (crate::simd_ops::ru128(P5)); xmm3 = crate::simd_ops::vmulps(xmm5, xmm5); xmm10 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P10)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P11)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P12)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P13)); xmm2 = crate::simd_ops::vmulps(xmm0, xmm3); xmm0 = crate::simd_ops::vsubps(xmm2, xmm1); xmm8 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P14)); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P15)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P16)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P17)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm2 = crate::simd_ops::vmulps(xmm0, xmm5); xmm0 = (xmm12) & (crate::simd_ops::ru128(P18)); xmm12 = (crate::simd_ops::ru128(rsp)) as u128; xmm7 = crate::simd_ops::vaddps(xmm2, xmm5); xmm6 = (xmm7) & (xmm9); xmm5 = (!(xmm9)) & (xmm8); xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x48u64))) as u128; xmm1 = crate::simd_ops::vaddps(xmm5, xmm6); xmm2 = (xmm0) ^ (xmm11); xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x68u64))) as u128; xmm2 = (xmm2) ^ (xmm1); xmm4 = crate::simd_ops::vmulps(xmm2, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm3 = crate::simd_ops::vsubps(xmm8, xmm5); xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x38u64))) as u128; rax = ((rax).wrapping_add(rax)) as u64; xmm0 = crate::simd_ops::vsubps(xmm7, xmm6); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm1 = crate::simd_ops::vaddps(xmm3, xmm0); xmm2 = (xmm1) ^ (xmm10); xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x58u64))) as u128; xmm5 = crate::simd_ops::vblendps(xmm2, xmm4, 0x7u8); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm5) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x78u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c8bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428120; const P1: u64 = 0x1426fd060; const P2: u64 = 0x1426fcaa0; const P3: u64 = 0x1426fce00; const P4: u64 = 0x1426fcc90; const P5: u64 = 0x1426fcfc0; const P6: u64 = 0x1426fd020; const P7: u64 = 0x1426fccc0; const P8: u64 = 0x1426fcf60; const P9: u64 = 0x1426fcb90; const P10: u64 = 0x1426fccd0; const P11: u64 = 0x1426fcb00; const P12: u64 = 0x1426fcd70; const P13: u64 = 0x1426fd070; const P14: u64 = 0x1426fcd60; const P15: u64 = 0x1426fcce0; const P16: u64 = 0x1426fcbf0; const P17: u64 = 0x1426fcb70; const P18: u64 = 0x1426fd090;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = ((rsp).wrapping_sub(0xc8u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; crate::simd_ops::wu128(r11.wrapping_sub(0x18u64), (xmm6) as u128); rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(r11.wrapping_sub(0x28u64), (xmm7) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm9) as u128); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0xc9u8); crate::simd_ops::wu128(rsp, (xmm0) as u128); xmm0 = crate::simd_ops::vpermilps(xmm0, 0x50u8); crate::simd_ops::wu128(r11.wrapping_sub(0x58u64), (xmm10) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x68u64), (xmm11) as u128); xmm11 = crate::simd_ops::vmulps(xmm0, crate::simd_ops::ru128(P0)); xmm3 = (xmm11) & (crate::simd_ops::ru128(P1)); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P2)); xmm1 = crate::simd_ops::vcvttps2dq(xmm0); xmm2 = crate::simd_ops::vpaddd(xmm1, crate::simd_ops::ru128(P3)); xmm5 = (xmm2) & (crate::simd_ops::ru128(P4)); xmm0 = (xmm5) & (crate::simd_ops::ru128(P5)); crate::simd_ops::wu128(r11.wrapping_sub(0x78u64), (xmm12) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm13) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm14) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm15) as u128); xmm10 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm0 = (xmm5) & (crate::simd_ops::ru128(P6)); xmm1 = 0u128; xmm9 = crate::simd_ops::vpcmpeqd(xmm0, xmm1); xmm4 = crate::simd_ops::vcvtdq2ps(xmm5); xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P7)); xmm1 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P8)); xmm2 = crate::simd_ops::vaddps(xmm0, xmm3); xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P9)); xmm3 = crate::simd_ops::vaddps(xmm2, xmm1); xmm1 = crate::simd_ops::vpsubd(xmm5, crate::simd_ops::ru128(P6)); xmm4 = crate::simd_ops::vaddps(xmm3, xmm0); xmm0 = (!(xmm1)) & (crate::simd_ops::ru128(P5)); xmm8 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm3 = crate::simd_ops::vmulps(xmm4, xmm4); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P10)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P11)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P12)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P13)); xmm2 = crate::simd_ops::vmulps(xmm0, xmm3); xmm0 = crate::simd_ops::vsubps(xmm2, xmm1); xmm7 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P14)); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P15)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P16)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P17)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm2 = crate::simd_ops::vmulps(xmm0, xmm4); xmm0 = (xmm11) & (crate::simd_ops::ru128(P18)); xmm6 = crate::simd_ops::vaddps(xmm2, xmm4); xmm2 = (xmm0) ^ (xmm10); xmm4 = (!(xmm9)) & (xmm7); xmm0 = crate::simd_ops::vsubps(xmm7, xmm4); xmm3 = (xmm6) & (xmm9); xmm1 = crate::simd_ops::vaddps(xmm4, xmm3); xmm5 = (xmm2) ^ (xmm1); xmm3 = crate::simd_ops::vsubps(xmm6, xmm3); xmm1 = crate::simd_ops::vaddps(xmm3, xmm0); xmm4 = (xmm1) ^ (xmm8); xmm0 = crate::simd_ops::vshufps(xmm4, xmm5, 0xaau8); xmm2 = crate::simd_ops::vpermilps(xmm0, 0x88u8); xmm1 = crate::simd_ops::vshufps(xmm4, xmm5, 0x0u8); xmm0 = crate::simd_ops::vmulps(xmm2, xmm1); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm0) as u128); xmm0 = crate::simd_ops::vpermilps(crate::simd_ops::ru128(rsp), 0xaau8); xmm11 = crate::simd_ops::vmulps(xmm0, crate::simd_ops::ru128(P0)); xmm3 = (xmm11) & (crate::simd_ops::ru128(P1)); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P2)); xmm1 = crate::simd_ops::vcvttps2dq(xmm0); xmm2 = crate::simd_ops::vpaddd(xmm1, crate::simd_ops::ru128(P3)); xmm5 = (xmm2) & (crate::simd_ops::ru128(P4)); xmm0 = (xmm5) & (crate::simd_ops::ru128(P5)); xmm1 = (xmm5) & (crate::simd_ops::ru128(P6)); xmm10 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm0 = 0u128; xmm4 = crate::simd_ops::vcvtdq2ps(xmm5); xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x78u64))) as u128; r11 = r11; xmm13 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rax = 0xffff0000; xmm14 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm15 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm8 = crate::simd_ops::vpcmpeqd(xmm1, xmm0); xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P7)); xmm1 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P8)); xmm2 = crate::simd_ops::vaddps(xmm0, xmm3); xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P9)); xmm3 = crate::simd_ops::vaddps(xmm2, xmm1); xmm1 = crate::simd_ops::vpsubd(xmm5, crate::simd_ops::ru128(P6)); xmm6 = crate::simd_ops::vaddps(xmm3, xmm0); xmm0 = (!(xmm1)) & (crate::simd_ops::ru128(P5)); xmm5 = crate::simd_ops::vmulps(xmm6, xmm6); xmm9 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P10)); xmm1 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P11)); xmm2 = crate::simd_ops::vmulps(xmm1, xmm5); xmm3 = crate::simd_ops::vaddps(xmm2, crate::simd_ops::ru128(P12)); xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P13)); xmm0 = crate::simd_ops::vmulps(xmm3, xmm5); xmm4 = crate::simd_ops::vmulps(xmm0, xmm5); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P15)); xmm2 = crate::simd_ops::vsubps(xmm4, xmm1); xmm4 = crate::simd_ops::vaddps(xmm2, crate::simd_ops::ru128(P14)); xmm1 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P16)); xmm2 = crate::simd_ops::vmulps(xmm1, xmm5); xmm3 = crate::simd_ops::vaddps(xmm2, crate::simd_ops::ru128(P17)); xmm0 = crate::simd_ops::vmulps(xmm3, xmm5); xmm1 = crate::simd_ops::vmulps(xmm0, xmm6); xmm2 = crate::simd_ops::vaddps(xmm1, xmm6); xmm7 = (xmm2) & (xmm8); xmm0 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm1 = crate::simd_ops::vpunpcklbw(xmm0, xmm0); xmm0 = crate::simd_ops::vpunpcklwd(xmm1, xmm1); xmm6 = (!(xmm8)) & (xmm4); xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x38u64))) as u128; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = crate::simd_ops::vpsrld(xmm0, 0x1fu64); xmm2 = crate::simd_ops::vsubps(xmm2, xmm7); xmm0 = crate::simd_ops::vsubps(xmm4, xmm6); xmm5 = crate::simd_ops::vpslld(xmm1, 0x1fu64); xmm1 = crate::simd_ops::vaddps(xmm2, xmm0); xmm0 = (xmm11) & (crate::simd_ops::ru128(P18)); xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x68u64))) as u128; xmm2 = (xmm1) ^ (xmm9); xmm3 = crate::simd_ops::vmulps(xmm2, crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))); xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x48u64))) as u128; xmm1 = crate::simd_ops::vaddps(xmm6, xmm7); xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x18u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x28u64))) as u128; xmm2 = (xmm0) ^ (xmm10); xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x58u64))) as u128; xmm2 = (xmm2) ^ (xmm1); xmm0 = crate::simd_ops::vmulps(xmm2, crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))); xmm4 = crate::simd_ops::vpermilps(xmm3, 0x1bu8); xmm3 = (xmm0) ^ (xmm5); xmm1 = crate::simd_ops::vaddps(xmm4, xmm3); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128); rax = 0x1; rsp = r11; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c8f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x1434268d0; const P2: u64 = 0x2; const P3: u64 = 0x143427080; const P4: u64 = 0x14183adac;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; rbx = r8; xmm5 = ((crate::simd_ops::ru32(P1)) as u32) as u128; rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rdi = rcx; xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; xmm6 = crate::simd_ops::vdpps(xmm0, xmm1, 0x7fu8); xmm0 = crate::simd_ops::vdpps(xmm0, xmm0, 0x7fu8); xmm3 = crate::simd_ops::vsqrtps(xmm0); xmm1 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8); xmm2 = crate::simd_ops::vsqrtps(xmm1); xmm4 = crate::simd_ops::vmulps(xmm3, xmm2); xmm0 = 0u128; fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 2; continue; } bb = 1; continue; }
            1 => { if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) / f32::from_bits(xmm4 as u32)).to_bits() as u128); bb = 3; continue; }
            3 => { xmm2 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm5 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm5 as u32) }).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = ((rax).wrapping_add(rax)) as u64; xmm0 = crate::simd_ops::vbroadcastss((xmm0 as u32)); crate::simd_ops::wu128(rbx.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c8fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428240; const P1: u64 = 0x6; const P2: u64 = 0x2; const P3: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm3 = (crate::simd_ops::ru128(P0)) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm7) as u128); crate::simd_ops::wu128(rsp, (xmm8) as u128); xmm8 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rdx.wrapping_mul(8)))) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P3))) as u32) as u32 as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; rax = ((rax).wrapping_add(rax)) as u64; xmm1 = crate::simd_ops::vmulps(xmm8, xmm8); xmm5 = 0u128; xmm6 = crate::simd_ops::vdpps(xmm0, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0x7fu8); xmm0 = crate::simd_ops::vmulps(xmm6, xmm6); xmm2 = crate::simd_ops::vsubps(xmm3, xmm0); xmm2 = crate::simd_ops::vmulps(xmm2, xmm1); xmm4 = crate::simd_ops::vsubps(xmm3, xmm2); xmm3 = crate::simd_ops::vmulps(xmm8, crate::simd_ops::ru128(r8.wrapping_add(rdx.wrapping_mul(8)))); xmm1 = crate::simd_ops::vsqrtps(xmm4); xmm2 = crate::simd_ops::vmulps(xmm6, xmm8); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp)) as u128; xmm2 = crate::simd_ops::vaddps(xmm2, xmm1); xmm0 = crate::simd_ops::vcmpps(xmm4, xmm5, 0x1u8); xmm4 = crate::simd_ops::vmulps(xmm2, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; xmm1 = crate::simd_ops::vsubps(xmm3, xmm4); xmm2 = crate::simd_ops::vblendvps(xmm1, xmm5, xmm0); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm3 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm0 = crate::simd_ops::vdpps(xmm3, xmm2, 0x7fu8);
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm1 = crate::simd_ops::vaddps(xmm0, xmm0);
    xmm3 = crate::simd_ops::vfmadd_ps(xmm2, xmm1, xmm3, true, false);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm3) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c90d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x4; const P2: u64 = 0x143427ab0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm7) as u128); crate::simd_ops::wu128(rsp, (xmm8) as u128); xmm8 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; xmm0 = crate::simd_ops::vshufps(xmm8, xmm8, 0xc9u8); xmm1 = crate::simd_ops::vshufps(xmm8, xmm8, 0xd2u8); xmm7 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm6 = crate::simd_ops::vpermilps(xmm7, 0xc9u8); xmm2 = crate::simd_ops::vmulps(xmm1, xmm6); xmm4 = crate::simd_ops::vpermilps(xmm7, 0xd2u8); xmm3 = crate::simd_ops::vmulps(xmm0, xmm4); xmm3 = crate::simd_ops::vsubps(xmm2, xmm3); xmm5 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P2)); xmm1 = crate::simd_ops::vpermilps(xmm5, 0xc9u8); xmm3 = crate::simd_ops::vmulps(xmm1, xmm4); xmm0 = crate::simd_ops::vpermilps(xmm5, 0xd2u8); xmm2 = crate::simd_ops::vmulps(xmm0, xmm6); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = ((rax).wrapping_add(rax)) as u64; xmm1 = crate::simd_ops::vpermilps(xmm7, 0xffu8); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm4 = crate::simd_ops::vsubps(xmm2, xmm3); xmm0 = crate::simd_ops::vmulps(xmm1, xmm5); xmm2 = crate::simd_ops::vaddps(xmm0, xmm8); xmm8 = (crate::simd_ops::ru128(rsp)) as u128; xmm3 = crate::simd_ops::vaddps(xmm4, xmm2); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm3) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x143428460; const P2: u64 = 0x143428120;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm1 = (crate::simd_ops::ru128(P1)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm5 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm0 = crate::simd_ops::vdpps(xmm5, xmm5, 0x7fu8);
    xmm4 = crate::simd_ops::vrsqrtps(xmm0);
    xmm0 = crate::simd_ops::vmulps(xmm4, xmm0);
    xmm2 = crate::simd_ops::vmulps(xmm0, xmm4);
    xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P2));
    xmm3 = crate::simd_ops::vsubps(xmm1, xmm2);
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm1 = crate::simd_ops::vmulps(xmm3, xmm0);
    xmm2 = crate::simd_ops::vmulps(xmm1, xmm5);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c91f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm1 = crate::simd_ops::vsubps(xmm0, xmm1);
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm1 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8);
    xmm0 = crate::simd_ops::vsqrtps(xmm1);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm1 = crate::simd_ops::vdpps(xmm0, xmm0, 0x7fu8);
    xmm2 = crate::simd_ops::vsqrtps(xmm1);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm5: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vshufps(xmm2, xmm2, 0xd2u8);
    xmm2 = crate::simd_ops::vshufps(xmm2, xmm2, 0xc9u8);
    xmm0 = crate::simd_ops::vpermilps(crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0xc9u8);
    xmm3 = crate::simd_ops::vmulps(xmm1, xmm0);
    xmm1 = crate::simd_ops::vpermilps(crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0xd2u8);
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = crate::simd_ops::vmulps(xmm2, xmm1);
    xmm5 = crate::simd_ops::vsubps(xmm3, xmm0);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm5) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c92b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm2 = crate::simd_ops::vdpps(xmm0, xmm1, 0x7fu8);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c92f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6; const P1: u64 = 0x4; const P2: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r9 = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    xmm1 = crate::simd_ops::vcmpps(xmm0, xmm1, 0x2u8);
    rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64;
    rax = ((((rax as u32)) & ((r9 as u32)))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rdx = (rdx & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(8)), ((rdx as u32)) as u32);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6; const P1: u64 = 0x4; const P2: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r9 = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    xmm1 = crate::simd_ops::vcmpps(xmm0, xmm1, 0x1u8);
    rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64;
    rax = ((((rax as u32)) & ((r9 as u32)))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rdx = (rdx & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(8)), ((rdx as u32)) as u32);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2; const P2: u64 = 0x6;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    xmm1 = crate::simd_ops::vcmpps(xmm0, xmm1, 0x4u8);
    rdx = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rdx = 0x0;
    rdx = (rdx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(8)), ((rdx as u32)) as u32);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c93e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6; const P1: u64 = 0x4; const P2: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r9 = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    xmm1 = crate::simd_ops::vcmpps(xmm0, xmm1, 0x0u8);
    rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64;
    rax = ((((rax as u32)) & ((r9 as u32)))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    rdx = (rdx & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rax.wrapping_mul(8)), ((rdx as u32)) as u32);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428480; const P1: u64 = 0x4; const P2: u64 = 0x2; const P3: u64 = 0x6; const P4: u64 = 0x143428550; const P5: u64 = 0x143428760; const P6: u64 = 0x1434288c0; const P7: u64 = 0x143428720; const P8: u64 = 0x143428240;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm5 = (crate::simd_ops::ru128(P0)) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm7) as u128); crate::simd_ops::wu128(rsp, (xmm8) as u128); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64; xmm8 = (crate::simd_ops::ru128(rsp)) as u128; rax = ((rax).wrapping_add(rax)) as u64; xmm6 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P3))) as u32) as u32 as u64; xmm2 = crate::simd_ops::vsubps(xmm0, xmm6); xmm1 = crate::simd_ops::vaddps(xmm2, crate::simd_ops::ru128(P4)); xmm0 = (xmm5) ^ (crate::simd_ops::ru128(P5)); xmm0 = crate::simd_ops::vcmpps(xmm2, xmm0, 0x1u8); xmm3 = crate::simd_ops::vblendvps(xmm2, xmm1, xmm0); xmm1 = crate::simd_ops::vsubps(xmm3, crate::simd_ops::ru128(P4)); xmm0 = crate::simd_ops::vcmpps(xmm5, xmm3, 0x1u8); xmm1 = crate::simd_ops::vblendvps(xmm3, xmm1, xmm0); rax = ((rax).wrapping_add(rax)) as u64; xmm2 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm3 = crate::simd_ops::vaddps(xmm2, xmm6); xmm0 = crate::simd_ops::vsubps(xmm3, crate::simd_ops::ru128(P6)); xmm6 = crate::simd_ops::vdivps(xmm0, crate::simd_ops::ru128(P4)); xmm0 = (xmm6) & (crate::simd_ops::ru128(P7)); xmm5 = crate::simd_ops::vroundps(xmm6, 0x1u8); xmm1 = 0u128; xmm2 = crate::simd_ops::vcmpps(xmm6, xmm1, 0x4u8); xmm3 = crate::simd_ops::vcmpps(xmm0, xmm5, 0x0u8); xmm0 = (xmm3) & (xmm2); xmm1 = crate::simd_ops::vsubps(xmm6, xmm5); xmm1 = crate::simd_ops::vblendvps(xmm1, crate::simd_ops::ru128(P8), xmm0); xmm2 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P4)); xmm3 = crate::simd_ops::vaddps(xmm2, crate::simd_ops::ru128(P6)); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm3) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x1434288c0; const P2: u64 = 0x143428550; const P3: u64 = 0x143428720; const P4: u64 = 0x143428240;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128);
    rax = ((rax).wrapping_add(rax)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm7) as u128);
    crate::simd_ops::wu128(rsp, (xmm8) as u128);
    xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    xmm1 = crate::simd_ops::vsubps(xmm0, crate::simd_ops::ru128(P1));
    xmm6 = crate::simd_ops::vdivps(xmm1, crate::simd_ops::ru128(P2));
    xmm0 = (xmm6) & (crate::simd_ops::ru128(P3));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm8 = (crate::simd_ops::ru128(rsp)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm5 = crate::simd_ops::vroundps(xmm6, 0x1u8);
    xmm1 = 0u128;
    xmm2 = crate::simd_ops::vcmpps(xmm6, xmm1, 0x4u8);
    xmm3 = crate::simd_ops::vcmpps(xmm0, xmm5, 0x0u8);
    xmm0 = (xmm3) & (xmm2);
    xmm1 = crate::simd_ops::vsubps(xmm6, xmm5);
    xmm1 = crate::simd_ops::vblendvps(xmm1, crate::simd_ops::ru128(P4), xmm0);
    xmm2 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P2));
    xmm3 = crate::simd_ops::vaddps(xmm2, crate::simd_ops::ru128(P1));
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm3) as u128);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6; const P1: u64 = 0x4; const P2: u64 = 0x2; const P3: u64 = 0x143428240; const P4: u64 = 0x1434288a0; const P5: u64 = 0x143428460;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    xmm1 = crate::simd_ops::vsubps(xmm1, xmm2);
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm0 = crate::simd_ops::vsubps(xmm0, xmm2);
    xmm2 = crate::simd_ops::vdivps(xmm0, xmm1);
    xmm1 = 0u128;
    xmm3 = crate::simd_ops::vmaxps(xmm2, xmm1);
    xmm4 = crate::simd_ops::vminps(xmm3, crate::simd_ops::ru128(P3));
    xmm5 = crate::simd_ops::vmulps(xmm4, xmm4);
    xmm1 = crate::simd_ops::vmulps(xmm5, xmm4);
    xmm2 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P4));
    xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P5));
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = crate::simd_ops::vaddps(xmm2, xmm1);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c96b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x4; const P2: u64 = 0x143428240;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = crate::simd_ops::vcmpps(xmm0, xmm1, 0x2u8);
    xmm1 = 0u128;
    xmm3 = crate::simd_ops::vblendvps(xmm1, crate::simd_ops::ru128(P2), xmm0);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm3) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6; const P1: u64 = 0x4; const P2: u64 = 0x2; const P3: u64 = 0x143428240;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vsubps(xmm1, xmm2);
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = crate::simd_ops::vsubps(xmm0, xmm2);
    xmm2 = crate::simd_ops::vdivps(xmm0, xmm1);
    xmm1 = 0u128;
    xmm3 = crate::simd_ops::vmaxps(xmm2, xmm1);
    xmm4 = crate::simd_ops::vminps(xmm3, crate::simd_ops::ru128(P3));
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm4) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6; const P1: u64 = 0x4; const P2: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm1 = crate::simd_ops::vfmadd_ps(xmm1, xmm2, xmm1, true, false);
    xmm1 = crate::simd_ops::vfmadd_ps(xmm2, xmm0, xmm1, false, false);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c97b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6; const P1: u64 = 0x4; const P2: u64 = 0x2; const P3: u64 = 0xa; const P4: u64 = 0x8; const P5: u64 = 0x143428240;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vsubps(xmm1, xmm2);
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P3))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = crate::simd_ops::vsubps(xmm0, xmm2);
    xmm2 = crate::simd_ops::vdivps(xmm0, xmm1);
    xmm1 = 0u128;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P4))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm3 = crate::simd_ops::vmaxps(xmm2, xmm1);
    xmm4 = crate::simd_ops::vminps(xmm3, crate::simd_ops::ru128(P5));
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm1 = crate::simd_ops::vfmadd_ps(xmm1, xmm4, xmm1, true, false);
    xmm1 = crate::simd_ops::vfmadd_ps(xmm4, xmm0, xmm1, false, false);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x143428240;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = 0u128;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm2 = crate::simd_ops::vmaxps(xmm0, xmm1);
    xmm3 = crate::simd_ops::vminps(xmm2, crate::simd_ops::ru128(P1));
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm3) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6; const P1: u64 = 0x4; const P2: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm1 = crate::simd_ops::vmaxps(xmm0, xmm1);
    xmm2 = crate::simd_ops::vminps(xmm1, xmm2);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c98c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x6; const P1: u64 = 0x4; const P2: u64 = 0x2; const P3: u64 = 0x143428720; const P4: u64 = 0x143428240;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm7) as u128); crate::simd_ops::wu128(rsp, (xmm8) as u128); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; xmm8 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P2))) as u32) as u32 as u64; xmm7 = crate::simd_ops::vsubps(xmm0, xmm8); rax = ((rax).wrapping_add(rax)) as u64; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm1 = crate::simd_ops::vsubps(xmm0, xmm8); xmm6 = crate::simd_ops::vdivps(xmm1, xmm7); xmm0 = (xmm6) & (crate::simd_ops::ru128(P3)); xmm5 = crate::simd_ops::vroundps(xmm6, 0x1u8); xmm1 = 0u128; xmm2 = crate::simd_ops::vcmpps(xmm6, xmm1, 0x4u8); xmm3 = crate::simd_ops::vcmpps(xmm0, xmm5, 0x0u8); xmm0 = (xmm3) & (xmm2); xmm1 = crate::simd_ops::vsubps(xmm6, xmm5); xmm1 = crate::simd_ops::vblendvps(xmm1, crate::simd_ops::ru128(P4), xmm0); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = ((rax).wrapping_add(rax)) as u64; xmm2 = crate::simd_ops::vmulps(xmm1, xmm7); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm3 = crate::simd_ops::vaddps(xmm2, xmm8); xmm8 = (crate::simd_ops::ru128(rsp)) as u128; crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm3) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm1 = crate::simd_ops::vmaxps(xmm0, xmm1);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c99b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm1 = crate::simd_ops::vminps(xmm0, xmm1);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c99f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    xmm0 = crate::simd_ops::vroundps(xmm1, 0x1u8);
    xmm2 = crate::simd_ops::vsubps(xmm1, xmm0);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vroundps(crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0x3u8);
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vroundps(crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0xcu8);
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vroundps(crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0x2u8);
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm1 = crate::simd_ops::vroundps(crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))), 0x1u8);
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = crate::simd_ops::vsqrtps(crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142980fb0; const P1: u64 = 0x142980d40; const P2: u64 = 0x142980d80; const P3: u64 = 0x142980dc0; const P4: u64 = 0x142980f40; const P5: u64 = 0x142980ab0; const P6: u64 = 0x142980560; const P7: u64 = 0x142980ca0; const P8: u64 = 0x142980cf0; const P9: u64 = 0x1429805a0; const P10: u64 = 0x142980d70; const P11: u64 = 0x142980620; const P12: u64 = 0x142980c70;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; xmm1 = (crate::simd_ops::ru128(P0)) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(r11.wrapping_sub(0x18u64), (xmm6) as u128); rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu128(r11.wrapping_sub(0x28u64), (xmm7) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x38u64), (xmm8) as u128); xmm2 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rdx.wrapping_mul(8)))) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm9) as u128); xmm9 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x58u64), (xmm10) as u128); xmm10 = crate::simd_ops::vdivps(xmm0, xmm2); xmm5 = (xmm10) & (crate::simd_ops::ru128(P2)); crate::simd_ops::wu128(r11.wrapping_sub(0x68u64), (xmm11) as u128); xmm11 = crate::simd_ops::vcmpps(xmm2, xmm1, 0x1u8); crate::simd_ops::wu128(r11.wrapping_sub(0x78u64), (xmm12) as u128); xmm12 = crate::simd_ops::vcmpps(xmm0, xmm1, 0x1u8); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm13) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm14) as u128); xmm14 = crate::simd_ops::vcmpps(xmm1, xmm2, 0x0u8); crate::simd_ops::wu128(rsp, (xmm15) as u128); xmm2 = (crate::simd_ops::ru128(P3)) as u128; xmm15 = crate::simd_ops::vcmpps(xmm1, xmm0, 0x0u8); xmm0 = (crate::simd_ops::ru128(P4)) as u128; xmm1 = (crate::simd_ops::ru128(P5)) as u128; xmm8 = crate::simd_ops::vcmpps(xmm0, xmm5, 0x1u8); xmm0 = crate::simd_ops::vcmpps(xmm1, xmm5, 0x1u8); xmm6 = (!(xmm8)) & (xmm0); xmm0 = crate::simd_ops::vaddps(xmm5, xmm2); xmm1 = crate::simd_ops::vsubps(xmm5, xmm2); xmm1 = crate::simd_ops::vdivps(xmm1, xmm0); xmm3 = (xmm1) & (xmm6); xmm2 = crate::simd_ops::vdivps(xmm2, xmm5); xmm0 = (xmm2) ^ (xmm9); xmm1 = (xmm0) & (xmm8); xmm2 = (xmm3) | (xmm1); xmm4 = (xmm6) | (xmm8); xmm3 = (xmm2) & (xmm4); xmm0 = (!(xmm4)) & (xmm5); xmm5 = (xmm3) | (xmm0); xmm2 = crate::simd_ops::vmulps(xmm5, xmm5); xmm0 = crate::simd_ops::vmulps(xmm2, crate::simd_ops::ru128(P6)); xmm0 = crate::simd_ops::vsubps(xmm0, crate::simd_ops::ru128(P7)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm2); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P8)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm2); xmm0 = crate::simd_ops::vsubps(xmm0, crate::simd_ops::ru128(P9)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm2); xmm0 = (xmm8) & (crate::simd_ops::ru128(P10)); xmm8 = (crate::simd_ops::ru128(P0)) as u128; xmm2 = crate::simd_ops::vmulps(xmm1, xmm5); xmm1 = (xmm6) & (crate::simd_ops::ru128(P11)); xmm1 = (xmm1) | (xmm0); xmm3 = crate::simd_ops::vaddps(xmm2, xmm5); xmm3 = crate::simd_ops::vaddps(xmm3, xmm1); xmm2 = (xmm10) & (xmm9); xmm10 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; xmm4 = (xmm3) ^ (xmm2); xmm13 = (!(xmm15)) & (xmm14); xmm0 = (xmm12) & (xmm11); xmm1 = (xmm0) & (xmm9); xmm1 = (xmm1) ^ (crate::simd_ops::ru128(P12)); xmm0 = (xmm1) & (xmm11); xmm2 = crate::simd_ops::vaddps(xmm4, xmm0); xmm3 = (!(xmm13)) & (xmm2); xmm5 = (!(xmm13)) & (xmm3); xmm0 = crate::simd_ops::vcmpps(xmm8, xmm10, 0x1u8); xmm2 = (xmm0) & (xmm15); xmm1 = (xmm15) & (xmm14); xmm3 = (xmm2) | (xmm1); xmm0 = (xmm9) & (xmm12); xmm2 = (xmm0) ^ (crate::simd_ops::ru128(P10)); xmm1 = (xmm2) & (xmm13); xmm3 = (!(xmm3)) & (xmm1); xmm0 = crate::simd_ops::vcmpps(xmm10, xmm8, 0x2u8); xmm1 = (xmm0) & (xmm15); xmm2 = (xmm1) & (crate::simd_ops::ru128(P12)); xmm4 = (xmm5) | (xmm3); xmm3 = (xmm4) | (xmm2); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; r11 = r11; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x18u64))) as u128; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x28u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x38u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x48u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x58u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x68u64))) as u128; xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x78u64))) as u128; xmm13 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm14 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm15 = (crate::simd_ops::ru128(rsp)) as u128; crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm3) as u128); rax = 0x1; rsp = r11; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x1434280f0; const P2: u64 = 0x143428850; const P3: u64 = 0x143428240; const P4: u64 = 0x1434281c0; const P5: u64 = 0x1434287f0; const P6: u64 = 0x143428320; const P7: u64 = 0x143428760;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    crate::simd_ops::wu128(rsp, (xmm6) as u128);
    xmm6 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm4 = crate::simd_ops::vmulps(xmm6, xmm6);
    xmm1 = crate::simd_ops::vmulps(xmm4, xmm4);
    xmm2 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P1));
    xmm1 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P2));
    xmm2 = crate::simd_ops::vaddps(xmm2, xmm1);
    xmm2 = crate::simd_ops::vaddps(xmm2, crate::simd_ops::ru128(P3));
    xmm1 = crate::simd_ops::vmulps(xmm6, xmm4);
    xmm3 = crate::simd_ops::vrcpps(xmm2);
    xmm2 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P4));
    xmm1 = crate::simd_ops::vmulps(xmm6, crate::simd_ops::ru128(P5));
    xmm6 = (crate::simd_ops::ru128(rsp)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = crate::simd_ops::vaddps(xmm2, xmm1);
    xmm1 = (crate::simd_ops::ru128(P6)) as u128;
    xmm2 = crate::simd_ops::vmulps(xmm3, xmm0);
    xmm3 = (xmm2) ^ (crate::simd_ops::ru128(P7));
    xmm0 = crate::simd_ops::vsubps(xmm1, xmm3);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x1434280f0; const P2: u64 = 0x143428850; const P3: u64 = 0x143428240; const P4: u64 = 0x1434281c0; const P5: u64 = 0x1434287f0; const P6: u64 = 0x143428760;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    crate::simd_ops::wu128(rsp, (xmm6) as u128);
    xmm6 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm4 = crate::simd_ops::vmulps(xmm6, xmm6);
    xmm1 = crate::simd_ops::vmulps(xmm4, xmm4);
    xmm2 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P1));
    xmm1 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P2));
    xmm2 = crate::simd_ops::vaddps(xmm2, xmm1);
    xmm2 = crate::simd_ops::vaddps(xmm2, crate::simd_ops::ru128(P3));
    xmm1 = crate::simd_ops::vmulps(xmm6, xmm4);
    xmm3 = crate::simd_ops::vrcpps(xmm2);
    xmm2 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P4));
    xmm1 = crate::simd_ops::vmulps(xmm6, crate::simd_ops::ru128(P5));
    xmm6 = (crate::simd_ops::ru128(rsp)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm0 = crate::simd_ops::vaddps(xmm2, xmm1);
    xmm2 = crate::simd_ops::vmulps(xmm3, xmm0);
    xmm3 = (xmm2) ^ (crate::simd_ops::ru128(P6));
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm3) as u128);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x143427e80; const P2: u64 = 0x143428240; const P3: u64 = 0x143428040; const P4: u64 = 0x1434287b0; const P5: u64 = 0x143428300; const P6: u64 = 0x1434288b0; const P7: u64 = 0x1434286f0; const P8: u64 = 0x143428440; const P9: u64 = 0x143427e40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    xmm0 = (crate::simd_ops::ru128(P1)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128);
    crate::simd_ops::wu128(rsp, (xmm7) as u128);
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128;
    xmm7 = (crate::simd_ops::ru128(rsp)) as u128;
    xmm1 = (xmm0) & (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    xmm5 = (xmm1) | (crate::simd_ops::ru128(P2));
    xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P3));
    xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P4));
    xmm3 = crate::simd_ops::vmulps(xmm1, xmm5);
    xmm0 = crate::simd_ops::vaddps(xmm3, crate::simd_ops::ru128(P5));
    xmm1 = crate::simd_ops::vsubps(xmm5, crate::simd_ops::ru128(P2));
    xmm4 = crate::simd_ops::vmulps(xmm0, xmm5);
    xmm2 = crate::simd_ops::vaddps(xmm4, crate::simd_ops::ru128(P6));
    xmm0 = (crate::simd_ops::ru128(P7)) as u128;
    xmm3 = crate::simd_ops::vmulps(xmm2, xmm5);
    xmm2 = (xmm0) & (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8))));
    xmm4 = crate::simd_ops::vaddps(xmm3, crate::simd_ops::ru128(P8));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm3 = crate::simd_ops::vpsrld(xmm2, 0x17u64);
    xmm0 = crate::simd_ops::vpsubd(xmm3, crate::simd_ops::ru128(P9));
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = crate::simd_ops::vcvtdq2ps(xmm0);
    xmm5 = crate::simd_ops::vmulps(xmm4, xmm1);
    xmm4 = crate::simd_ops::vaddps(xmm5, xmm2);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm4) as u128);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420c9f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142980dc0; const P1: u64 = 0x2; const P2: u64 = 0x142980580; const P3: u64 = 0x142980b10; const P4: u64 = 0x142980d60; const P5: u64 = 0x142980590; const P6: u64 = 0x142980d50; const P7: u64 = 0x142980c50; const P8: u64 = 0x142981040; const P9: u64 = 0x142980c20; const P10: u64 = 0x142980ac0; const P11: u64 = 0x142980e70; const P12: u64 = 0x142980570; const P13: u64 = 0x142980aa0; const P14: u64 = 0x142980460; const P15: u64 = 0x142980be0; const P16: u64 = 0x1429805f0; const P17: u64 = 0x142980660;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm3 = (crate::simd_ops::ru128(P0)) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm7) as u128); crate::simd_ops::wu128(rsp, (xmm8) as u128); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; xmm2 = crate::simd_ops::vmaxps(xmm1, crate::simd_ops::ru128(P2)); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm8 = crate::simd_ops::vcmpps(xmm1, xmm0, 0x2u8); xmm1 = (xmm2) & (crate::simd_ops::ru128(P3)); xmm5 = (xmm1) | (crate::simd_ops::ru128(P4)); xmm4 = crate::simd_ops::vcmpps(xmm5, crate::simd_ops::ru128(P5), 0x1u8); xmm0 = crate::simd_ops::vpsrld(xmm2, 0x17u64); xmm0 = crate::simd_ops::vpsubd(xmm0, crate::simd_ops::ru128(P6)); xmm1 = crate::simd_ops::vcvtdq2ps(xmm0); xmm2 = crate::simd_ops::vaddps(xmm1, xmm3); xmm0 = (xmm3) & (xmm4); xmm6 = crate::simd_ops::vsubps(xmm2, xmm0); xmm1 = crate::simd_ops::vsubps(xmm5, xmm3); xmm0 = (xmm4) & (xmm5); xmm5 = crate::simd_ops::vaddps(xmm1, xmm0); xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P7)); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P8)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm5); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P9)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm5); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P10)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm5); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P11)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm5); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P12)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm5); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P13)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm5); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P14)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm5); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P15)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm5); xmm0 = crate::simd_ops::vmulps(xmm6, crate::simd_ops::ru128(P16)); xmm4 = crate::simd_ops::vmulps(xmm5, xmm5); xmm3 = crate::simd_ops::vmulps(xmm1, xmm4); xmm1 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P4)); xmm3 = crate::simd_ops::vaddps(xmm3, xmm0); xmm2 = crate::simd_ops::vsubps(xmm3, xmm1); xmm1 = crate::simd_ops::vmulps(xmm6, crate::simd_ops::ru128(P17)); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = ((rax).wrapping_add(rax)) as u64; xmm4 = crate::simd_ops::vaddps(xmm2, xmm5); xmm2 = crate::simd_ops::vaddps(xmm4, xmm1); xmm3 = (xmm2) | (xmm8); xmm8 = (crate::simd_ops::ru128(rsp)) as u128; crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm3) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ca060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142980d60; const P1: u64 = 0x142980dc0; const P2: u64 = 0x142980580; const P3: u64 = 0x142980b10; const P4: u64 = 0x142980590; const P5: u64 = 0x142980d50; const P6: u64 = 0x142980c50; const P7: u64 = 0x142981040; const P8: u64 = 0x142980c20; const P9: u64 = 0x142980ac0; const P10: u64 = 0x142980e70; const P11: u64 = 0x142980570; const P12: u64 = 0x142980aa0; const P13: u64 = 0x142980460; const P14: u64 = 0x142980be0; const P15: u64 = 0x1429805f0; const P16: u64 = 0x142980660; const P17: u64 = 0x142980b80; const P18: u64 = 0x142980540; const P19: u64 = 0x142980d00; const P20: u64 = 0x142980c30; const P21: u64 = 0x142980f90; const P22: u64 = 0x142980550; const P23: u64 = 0x142980420; const P24: u64 = 0x142980c40; const P25: u64 = 0x142980ce0; const P26: u64 = 0x142980b20; const P27: u64 = 0x1429805e0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); xmm8 = (crate::simd_ops::ru128(P0)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm9) as u128); xmm9 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm10) as u128); crate::simd_ops::wu128(rsp, (xmm11) as u128); xmm11 = (crate::simd_ops::ru128(P1)) as u128; xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; xmm2 = crate::simd_ops::vmaxps(xmm1, crate::simd_ops::ru128(P2)); xmm7 = crate::simd_ops::vcmpps(xmm1, xmm0, 0x2u8); xmm1 = (xmm2) & (crate::simd_ops::ru128(P3)); xmm4 = (xmm1) | (xmm8); xmm3 = crate::simd_ops::vcmpps(xmm4, crate::simd_ops::ru128(P4), 0x1u8); xmm0 = crate::simd_ops::vpsrld(xmm2, 0x17u64); xmm0 = crate::simd_ops::vpsubd(xmm0, crate::simd_ops::ru128(P5)); xmm1 = crate::simd_ops::vcvtdq2ps(xmm0); xmm2 = crate::simd_ops::vaddps(xmm1, xmm11); xmm1 = (xmm3) & (xmm4); xmm0 = (xmm3) & (xmm11); xmm6 = crate::simd_ops::vsubps(xmm2, xmm0); xmm0 = crate::simd_ops::vsubps(xmm4, xmm11); xmm5 = crate::simd_ops::vaddps(xmm1, xmm0); xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P6)); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P7)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm5); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P8)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm5); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P9)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm5); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P10)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm5); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P11)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm5); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P12)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm5); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P13)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm5); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P14)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm5); xmm0 = crate::simd_ops::vmulps(xmm6, crate::simd_ops::ru128(P15)); xmm4 = crate::simd_ops::vmulps(xmm5, xmm5); xmm3 = crate::simd_ops::vmulps(xmm1, xmm4); xmm3 = crate::simd_ops::vaddps(xmm3, xmm0); xmm1 = crate::simd_ops::vmulps(xmm4, xmm8); xmm2 = crate::simd_ops::vsubps(xmm3, xmm1); xmm1 = crate::simd_ops::vmulps(xmm6, crate::simd_ops::ru128(P16)); xmm4 = crate::simd_ops::vaddps(xmm2, xmm5); xmm2 = crate::simd_ops::vaddps(xmm4, xmm1); xmm3 = (xmm2) | (xmm7); xmm0 = crate::simd_ops::vmulps(xmm3, xmm9); xmm1 = crate::simd_ops::vminps(xmm0, crate::simd_ops::ru128(P17)); xmm5 = crate::simd_ops::vmaxps(xmm1, crate::simd_ops::ru128(P18)); xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P19)); xmm3 = crate::simd_ops::vaddps(xmm1, xmm8); xmm0 = crate::simd_ops::vcvttps2dq(xmm3); xmm2 = crate::simd_ops::vcvtdq2ps(xmm0); xmm1 = crate::simd_ops::vcmpps(xmm3, xmm2, 0x1u8); xmm0 = (xmm1) & (xmm11); xmm4 = crate::simd_ops::vsubps(xmm2, xmm0); xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P20)); xmm1 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P21)); xmm3 = crate::simd_ops::vsubps(xmm5, xmm0); xmm3 = crate::simd_ops::vsubps(xmm3, xmm1); xmm0 = crate::simd_ops::vcvttps2dq(xmm4); xmm1 = crate::simd_ops::vpaddd(xmm0, crate::simd_ops::ru128(P5)); xmm5 = crate::simd_ops::vpslld(xmm1, 0x17u64); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P22)); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P23)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P24)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P25)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P26)); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rax = ((rax).wrapping_add(rax)) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm2 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P27)); xmm0 = crate::simd_ops::vmulps(xmm3, xmm3); xmm1 = crate::simd_ops::vmulps(xmm2, xmm0); xmm2 = crate::simd_ops::vaddps(xmm1, xmm3); xmm3 = crate::simd_ops::vaddps(xmm2, xmm11); xmm11 = (crate::simd_ops::ru128(rsp)) as u128; xmm0 = crate::simd_ops::vmulps(xmm3, xmm5); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ca2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x1434285f0; const P2: u64 = 0x143428900; const P3: u64 = 0x143428120; const P4: u64 = 0x143427e40; const P5: u64 = 0x143427fd0; const P6: u64 = 0x143428030; const P7: u64 = 0x1434280d0; const P8: u64 = 0x143428160; const P9: u64 = 0x143428270;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rax).wrapping_add(rax)) as u64;
    crate::simd_ops::wu128(rsp, (xmm6) as u128);
    xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128;
    xmm2 = crate::simd_ops::vminps(xmm0, crate::simd_ops::ru128(P1));
    xmm3 = crate::simd_ops::vmaxps(xmm2, crate::simd_ops::ru128(P2));
    xmm1 = crate::simd_ops::vsubps(xmm3, crate::simd_ops::ru128(P3));
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    xmm2 = crate::simd_ops::vcvtps2dq(xmm1);
    xmm0 = crate::simd_ops::vcvtdq2ps(xmm2);
    xmm6 = crate::simd_ops::vsubps(xmm3, xmm0);
    xmm0 = crate::simd_ops::vpaddd(xmm2, crate::simd_ops::ru128(P4));
    xmm1 = crate::simd_ops::vmulps(xmm6, crate::simd_ops::ru128(P5));
    xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P6));
    xmm3 = crate::simd_ops::vmulps(xmm1, xmm6);
    xmm5 = crate::simd_ops::vpslld(xmm0, 0x17u64);
    xmm0 = crate::simd_ops::vaddps(xmm3, crate::simd_ops::ru128(P7));
    xmm4 = crate::simd_ops::vmulps(xmm0, xmm6);
    xmm2 = crate::simd_ops::vaddps(xmm4, crate::simd_ops::ru128(P8));
    xmm3 = crate::simd_ops::vmulps(xmm2, xmm6);
    xmm1 = crate::simd_ops::vaddps(xmm3, crate::simd_ops::ru128(P9));
    xmm6 = (crate::simd_ops::ru128(rsp)) as u128;
    rax = ((rax).wrapping_add(rax)) as u64;
    xmm2 = crate::simd_ops::vmulps(xmm1, xmm5);
    crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ca340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x142980b80; const P2: u64 = 0x142980540; const P3: u64 = 0x142980d00; const P4: u64 = 0x142980d60; const P5: u64 = 0x142980dc0; const P6: u64 = 0x142980c30; const P7: u64 = 0x142980f90; const P8: u64 = 0x142980d50; const P9: u64 = 0x142980550; const P10: u64 = 0x142980420; const P11: u64 = 0x142980c40; const P12: u64 = 0x142980ce0; const P13: u64 = 0x142980b20; const P14: u64 = 0x1429805e0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x18u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp, (xmm6) as u128); xmm6 = (crate::simd_ops::ru128(rsp)) as u128; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; xmm1 = crate::simd_ops::vminps(xmm0, crate::simd_ops::ru128(P1)); xmm5 = crate::simd_ops::vmaxps(xmm1, crate::simd_ops::ru128(P2)); xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P3)); xmm2 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P4)); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm0 = crate::simd_ops::vcvttps2dq(xmm2); xmm3 = crate::simd_ops::vcvtdq2ps(xmm0); xmm1 = crate::simd_ops::vcmpps(xmm2, xmm3, 0x1u8); xmm0 = (xmm1) & (crate::simd_ops::ru128(P5)); xmm4 = crate::simd_ops::vsubps(xmm3, xmm0); xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P6)); xmm1 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P7)); xmm3 = crate::simd_ops::vsubps(xmm5, xmm0); xmm5 = crate::simd_ops::vsubps(xmm3, xmm1); xmm0 = crate::simd_ops::vcvttps2dq(xmm4); xmm0 = crate::simd_ops::vpaddd(xmm0, crate::simd_ops::ru128(P8)); xmm4 = crate::simd_ops::vpslld(xmm0, 0x17u64); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P9)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P10)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm5); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P11)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm5); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P12)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm5); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P13)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm5); xmm2 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P14)); xmm0 = crate::simd_ops::vmulps(xmm5, xmm5); xmm1 = crate::simd_ops::vmulps(xmm2, xmm0); rax = ((rax).wrapping_add(rax)) as u64; xmm2 = crate::simd_ops::vaddps(xmm1, xmm5); xmm3 = crate::simd_ops::vaddps(xmm2, crate::simd_ops::ru128(P5)); xmm0 = crate::simd_ops::vmulps(xmm3, xmm4); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm0) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x18u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ca440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x142980d80; const P2: u64 = 0x1429804c0; const P3: u64 = 0x142980fa0; const P4: u64 = 0x142980ec0; const P5: u64 = 0x142980ed0; const P6: u64 = 0x142980e80; const P7: u64 = 0x142980f80; const P8: u64 = 0x142980a80; const P9: u64 = 0x142980630; const P10: u64 = 0x142980610; const P11: u64 = 0x142980c60; const P12: u64 = 0x142980b00; const P13: u64 = 0x142980670; const P14: u64 = 0x142980fe0; const P15: u64 = 0x142980dc0; const P16: u64 = 0x142980d40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); crate::simd_ops::wu128(rsp, (xmm7) as u128); xmm7 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; xmm4 = (xmm7) & (crate::simd_ops::ru128(P1)); xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P2)); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm2 = crate::simd_ops::vcvttps2dq(xmm0); xmm1 = crate::simd_ops::vpaddd(xmm2, crate::simd_ops::ru128(P3)); xmm3 = (xmm1) & (crate::simd_ops::ru128(P4)); xmm0 = (crate::simd_ops::ru128(P5)) as u128; xmm2 = (xmm0) & (xmm3); xmm1 = 0u128; xmm6 = crate::simd_ops::vpcmpeqd(xmm2, xmm1); xmm5 = crate::simd_ops::vcvtdq2ps(xmm3); xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P6)); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P7)); xmm2 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P8)); xmm3 = crate::simd_ops::vaddps(xmm1, xmm4); xmm4 = crate::simd_ops::vaddps(xmm3, xmm0); xmm5 = crate::simd_ops::vaddps(xmm4, xmm2); xmm3 = crate::simd_ops::vmulps(xmm5, xmm5); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P9)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P10)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P11)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P12)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P13)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P14)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm2 = crate::simd_ops::vmulps(xmm1, xmm5); xmm1 = (crate::simd_ops::ru128(P15)) as u128; xmm4 = crate::simd_ops::vaddps(xmm2, xmm5); xmm0 = (xmm4) ^ (crate::simd_ops::ru128(P16)); xmm2 = crate::simd_ops::vdivps(xmm1, xmm4); xmm1 = (xmm7) & (crate::simd_ops::ru128(P16)); xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rax = ((rax).wrapping_add(rax)) as u64; xmm3 = (!(xmm6)) & (xmm0); xmm0 = (xmm2) & (xmm6); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm3 = (xmm3) | (xmm0); xmm2 = (xmm3) ^ (xmm1); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ca560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x142980d80; const P2: u64 = 0x1429804c0; const P3: u64 = 0x142980fa0; const P4: u64 = 0x142980ec0; const P5: u64 = 0x142980ed0; const P6: u64 = 0x142980e80; const P7: u64 = 0x142980f80; const P8: u64 = 0x142980a80; const P9: u64 = 0x142980630; const P10: u64 = 0x142980610; const P11: u64 = 0x142980c60; const P12: u64 = 0x142980b00; const P13: u64 = 0x142980670; const P14: u64 = 0x142980fe0; const P15: u64 = 0x142980dc0; const P16: u64 = 0x142980d40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); crate::simd_ops::wu128(rsp, (xmm7) as u128); xmm7 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; xmm4 = (xmm7) & (crate::simd_ops::ru128(P1)); xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P2)); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm2 = crate::simd_ops::vcvttps2dq(xmm0); xmm1 = crate::simd_ops::vpaddd(xmm2, crate::simd_ops::ru128(P3)); xmm3 = (xmm1) & (crate::simd_ops::ru128(P4)); xmm0 = (crate::simd_ops::ru128(P5)) as u128; xmm2 = (xmm0) & (xmm3); xmm1 = 0u128; xmm6 = crate::simd_ops::vpcmpeqd(xmm2, xmm1); xmm5 = crate::simd_ops::vcvtdq2ps(xmm3); xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P6)); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P7)); xmm2 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P8)); xmm3 = crate::simd_ops::vaddps(xmm1, xmm4); xmm4 = crate::simd_ops::vaddps(xmm3, xmm0); xmm5 = crate::simd_ops::vaddps(xmm4, xmm2); xmm3 = crate::simd_ops::vmulps(xmm5, xmm5); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P9)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P10)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P11)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P12)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P13)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P14)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm0 = (crate::simd_ops::ru128(P15)) as u128; xmm2 = crate::simd_ops::vmulps(xmm1, xmm5); xmm3 = crate::simd_ops::vaddps(xmm2, xmm5); xmm1 = crate::simd_ops::vdivps(xmm0, xmm3); xmm1 = (xmm1) ^ (crate::simd_ops::ru128(P16)); xmm2 = (!(xmm6)) & (xmm1); xmm1 = (xmm7) & (crate::simd_ops::ru128(P16)); xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rax = ((rax).wrapping_add(rax)) as u64; xmm0 = (xmm6) & (xmm3); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm3 = (xmm2) | (xmm0); xmm5 = (xmm3) ^ (xmm1); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm5) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ca680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142980ed0; const P1: u64 = 0x2; const P2: u64 = 0x142980d80; const P3: u64 = 0x1429804c0; const P4: u64 = 0x142980fa0; const P5: u64 = 0x142980ec0; const P6: u64 = 0x142980f00; const P7: u64 = 0x142980a80; const P8: u64 = 0x142980e80; const P9: u64 = 0x142980f80; const P10: u64 = 0x142980e00; const P11: u64 = 0x142980470; const P12: u64 = 0x142980dd0; const P13: u64 = 0x142980d60; const P14: u64 = 0x142980dc0; const P15: u64 = 0x142980e60; const P16: u64 = 0x142980600; const P17: u64 = 0x142980a90;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; xmm3 = (crate::simd_ops::ru128(P0)) as u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); crate::simd_ops::wu128(rsp, (xmm7) as u128); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; xmm4 = (xmm0) & (crate::simd_ops::ru128(P2)); xmm1 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P3)); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm2 = crate::simd_ops::vcvttps2dq(xmm1); xmm0 = crate::simd_ops::vpaddd(xmm2, crate::simd_ops::ru128(P4)); xmm1 = (xmm0) & (crate::simd_ops::ru128(P5)); xmm2 = crate::simd_ops::vpsubd(xmm1, xmm3); xmm0 = (!(xmm2)) & (crate::simd_ops::ru128(P6)); xmm7 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm5 = crate::simd_ops::vcvtdq2ps(xmm1); xmm0 = 0u128; xmm1 = (xmm3) & (xmm2); xmm2 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P7)); xmm6 = crate::simd_ops::vpcmpeqd(xmm1, xmm0); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P8)); xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P9)); xmm3 = crate::simd_ops::vaddps(xmm0, xmm4); xmm4 = crate::simd_ops::vaddps(xmm3, xmm1); xmm5 = crate::simd_ops::vaddps(xmm4, xmm2); xmm3 = crate::simd_ops::vmulps(xmm5, xmm5); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P10)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P11)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P12)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P13)); xmm2 = crate::simd_ops::vmulps(xmm0, xmm3); xmm0 = crate::simd_ops::vsubps(xmm2, xmm1); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P14)); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P15)); xmm4 = (!(xmm6)) & (xmm0); xmm0 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P16)); xmm2 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm2, crate::simd_ops::ru128(P17)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm2 = crate::simd_ops::vmulps(xmm0, xmm5); xmm3 = crate::simd_ops::vaddps(xmm2, xmm5); xmm1 = (xmm3) & (xmm6); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; rax = ((rax).wrapping_add(rax)) as u64; xmm0 = crate::simd_ops::vaddps(xmm4, xmm1); xmm2 = (xmm0) ^ (xmm7); xmm7 = (crate::simd_ops::ru128(rsp)) as u128; crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm2) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1420ca7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2; const P1: u64 = 0x142980d80; const P2: u64 = 0x1429804c0; const P3: u64 = 0x142980fa0; const P4: u64 = 0x142980ec0; const P5: u64 = 0x142980f00; const P6: u64 = 0x142980ed0; const P7: u64 = 0x142980e80; const P8: u64 = 0x142980f80; const P9: u64 = 0x142980a80; const P10: u64 = 0x142980e00; const P11: u64 = 0x142980470; const P12: u64 = 0x142980dd0; const P13: u64 = 0x142980d60; const P14: u64 = 0x142980e60; const P15: u64 = 0x142980dc0; const P16: u64 = 0x142980600; const P17: u64 = 0x142980a90; const P18: u64 = 0x142980d40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rax = ((rax).wrapping_add(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm7) as u128); crate::simd_ops::wu128(rsp, (xmm8) as u128); xmm8 = (crate::simd_ops::ru128(r8.wrapping_add(rax.wrapping_mul(8)))) as u128; xmm4 = (xmm8) & (crate::simd_ops::ru128(P1)); xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P2)); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; xmm2 = crate::simd_ops::vcvttps2dq(xmm0); xmm1 = crate::simd_ops::vpaddd(xmm2, crate::simd_ops::ru128(P3)); xmm3 = (xmm1) & (crate::simd_ops::ru128(P4)); xmm0 = (crate::simd_ops::ru128(P5)) as u128; xmm1 = (xmm0) & (xmm3); xmm0 = (crate::simd_ops::ru128(P6)) as u128; xmm2 = (xmm0) & (xmm3); xmm7 = crate::simd_ops::vpslld(xmm1, 0x1du64); xmm1 = 0u128; xmm6 = crate::simd_ops::vpcmpeqd(xmm2, xmm1); xmm5 = crate::simd_ops::vcvtdq2ps(xmm3); xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P7)); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P8)); xmm2 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P9)); xmm3 = crate::simd_ops::vaddps(xmm1, xmm4); xmm4 = crate::simd_ops::vaddps(xmm3, xmm0); xmm5 = crate::simd_ops::vaddps(xmm4, xmm2); xmm3 = crate::simd_ops::vmulps(xmm5, xmm5); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P10)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P11)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P12)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P13)); xmm2 = crate::simd_ops::vmulps(xmm0, xmm3); xmm0 = crate::simd_ops::vsubps(xmm2, xmm1); xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P14)); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P15)); xmm4 = (!(xmm6)) & (xmm0); xmm0 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P16)); xmm2 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm2, crate::simd_ops::ru128(P17)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); xmm2 = crate::simd_ops::vmulps(xmm0, xmm5); xmm3 = crate::simd_ops::vaddps(xmm2, xmm5); xmm2 = (xmm8) & (crate::simd_ops::ru128(P18)); xmm8 = (crate::simd_ops::ru128(rsp)) as u128; rax = ((rax).wrapping_add(rax)) as u64; xmm1 = (xmm3) & (xmm6); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm4 = crate::simd_ops::vaddps(xmm4, xmm1); xmm1 = (xmm2) ^ (xmm7); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm3 = (xmm4) ^ (xmm1); crate::simd_ops::wu128(r8.wrapping_add(rax.wrapping_mul(8)), (xmm3) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}
