#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e63450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0; const P1: u64 = 0x7001e5cb70; const P2: u64 = 0x7001e6bc10; const P3: u64 = 0x7001e6bc28; const P4: u64 = 0x7001e6bc40; const P5: u64 = 0x7001e1c690; const P6: u64 = 0x7001e1887c; const P7: u64 = 0x7001e76d68; const P8: u64 = 0x7001e66444; const P9: u64 = 0x7001e6365c;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[485], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rsi = ((r8 as u32)) as u32 as u64; r14 = (rdx) as u64; rbx = (rcx) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xe8u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; r10 = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; r9 = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(rcx).wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rcx = (!((rcx as u32))) as u32 as u64; rcx = ((((rcx as u32)) & (0x4u32))) as u32 as u64; r9 = ((((r9 as u32)) & (0xfffffffeu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((r9 as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(rbx).wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = ((((rcx as u32)) & (crate::simd_ops::ru32(r10.wrapping_add(rbx).wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e634c9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e634d5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbx).wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); r9 = ((rsi as u32)) as u32 as u64; r8 = (r14) as u64; rdx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6350b); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e6350b, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e63532 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 5; continue; } _ => l.unknown(0x7001e63450) } }; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x48u64)))) as u64; r8 = (0x2u32) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; rdi = (if (((fl_l as u64)) == ((fl_r as u64))) { (r8 as u32) } else { (rdi as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xe8u64), ((rdi as u32)) as u32); bb = 7; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0xe8u64))) as u32 as u64; bb = 6; continue; }
            6 => { r8 = (0x2u32) as u32 as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rdi = ((((rdi as u32)) | (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rdx = (!((rdx as u32))) as u32 as u64; rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rdi as u32)))) as u32 as u64; rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = ((((rdx as u32)) & (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6358b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            11 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rbx = ((P2) as u64) as u64; bb = 14; continue; }
            13 => { r8 = (0x2u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 14; continue; }
            14 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e635dc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e635ec); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e635fd); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 15; continue; }
            15 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rbx = ((P2) as u64) as u64; bb = 18; continue; }
            17 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 18; continue; }
            18 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63630); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63643); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63657); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e63660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0; const P1: u64 = 0x7001e5cdf0; const P2: u64 = 0x7001e6bc10; const P3: u64 = 0x7001e6bc28; const P4: u64 = 0x7001e6bc40; const P5: u64 = 0x7001e1c690; const P6: u64 = 0x7001e1887c; const P7: u64 = 0x7001e76d68; const P8: u64 = 0x7001e66444; const P9: u64 = 0x7001e6386a;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[486], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rsi = (rdx) as u64; rbx = (rcx) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x100u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; r8 = (crate::simd_ops::ru32(r9.wrapping_add(rcx).wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(rcx).wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rcx = (!((rcx as u32))) as u32 as u64; rcx = ((((rcx as u32)) & (0x4u32))) as u32 as u64; r8 = ((((r8 as u32)) & (0xfffffffeu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((r8 as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(rbx).wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = ((((rcx as u32)) & (crate::simd_ops::ru32(r9.wrapping_add(rbx).wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e636d0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e636dc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbx).wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); r9 = (0x1u32) as u32 as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63722); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001e63722, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e63749 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 5; continue; } _ => l.unknown(0x7001e63660) } }; rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x68u64)))) as u64; r8 = (0x2u32) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; rdi = (if (((fl_l as u64)) == ((fl_r as u64))) { (r8 as u32) } else { (rdi as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x100u64), ((rdi as u32)) as u32); bb = 7; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x100u64))) as u32 as u64; bb = 6; continue; }
            6 => { r8 = (0x2u32) as u32 as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rdi = ((((rdi as u32)) | (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rdx = (!((rdx as u32))) as u32 as u64; rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rdi as u32)))) as u32 as u64; rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = ((((rdx as u32)) & (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e637a2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0xd0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rbx = ((P2) as u64) as u64; bb = 14; continue; }
            13 => { r8 = (0x2u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 14; continue; }
            14 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e637ea); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e637fa); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6380b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            15 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rbx = ((P2) as u64) as u64; bb = 18; continue; }
            17 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 18; continue; }
            18 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6383e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0xa0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63851); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0xa0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63865); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e63870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0; const P1: u64 = 0x7001e5cdf0; const P2: u64 = 0x7001e6bc10; const P3: u64 = 0x7001e6bc28; const P4: u64 = 0x7001e6bc40; const P5: u64 = 0x7001e1c690; const P6: u64 = 0x7001e1887c; const P7: u64 = 0x7001e76d68; const P8: u64 = 0x7001e66444; const P9: u64 = 0x7001e63a7c;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[487], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rsi = ((r8 as u32)) as u32 as u64; r14 = (rdx) as u64; rbx = (rcx) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xe8u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; r10 = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; r9 = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(rcx).wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rcx = (!((rcx as u32))) as u32 as u64; rcx = ((((rcx as u32)) & (0x4u32))) as u32 as u64; r9 = ((((r9 as u32)) & (0xfffffffeu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((r9 as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(rbx).wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = ((((rcx as u32)) & (crate::simd_ops::ru32(r10.wrapping_add(rbx).wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e638e9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e638f5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbx).wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); r9 = ((rsi as u32)) as u32 as u64; r8 = (r14) as u64; rdx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6392b); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e6392b, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e63952 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 5; continue; } _ => l.unknown(0x7001e63870) } }; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x48u64)))) as u64; r8 = (0x2u32) as u32 as u64; fl_l = (rdx) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; rdi = (if (((fl_l as u64)) == ((fl_r as u64))) { (r8 as u32) } else { (rdi as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xe8u64), ((rdi as u32)) as u32); bb = 7; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0xe8u64))) as u32 as u64; bb = 6; continue; }
            6 => { r8 = (0x2u32) as u32 as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rdi = ((((rdi as u32)) | (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rdx = (!((rdx as u32))) as u32 as u64; rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rdi as u32)))) as u32 as u64; rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = ((((rdx as u32)) & (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e639ab); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 10; continue; }
            10 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            11 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rbx = ((P2) as u64) as u64; bb = 14; continue; }
            13 => { r8 = (0x2u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 14; continue; }
            14 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e639fc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63a0c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63a1d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 15; continue; }
            15 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rbx = ((P2) as u64) as u64; bb = 18; continue; }
            17 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 18; continue; }
            18 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63a50); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63a63); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63a77); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e63a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1f08c; const P1: u64 = 0x7001e696a0; const P2: u64 = 0x7001e26f90; const P3: u64 = 0x7001e1f210; const P4: u64 = 0x7001e6bc10; const P5: u64 = 0x7001e6bc28; const P6: u64 = 0x7001e6bc40; const P7: u64 = 0x7001e1c690; const P8: u64 = 0x7001e1887c; const P9: u64 = 0x7001e76d68; const P10: u64 = 0x7001e66444; const P11: u64 = 0x7001e63bf4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[488], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; rdx = (rcx) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63aa0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbx).wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u32(rsp.wrapping_add(0xc0u64), (0x0u32) as u32); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); rdi = (0x2u32) as u32 as u64; r9 = ((rdi as u32)) as u32 as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63af1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001e63af1, ({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e63b16 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x7001e63a80) } }; rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x68u64)))) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rdx) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; r8 = (if (((fl_l as u64)) == ((fl_r as u64))) { (rdi as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((r8 as u32)) as u32); bb = 3; continue; }
            2 => { rdi = (0x2u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; r8 = ((((r8 as u32)) | (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rdx = (!((rdx as u32))) as u32 as u64; rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((r8 as u32)))) as u32 as u64; rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = ((((rdx as u32)) & (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63b55); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001e63b55, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e63b16 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x7001e63a80) } }; rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63b63); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001e63b63, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e63b16 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x7001e63a80) } }; rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63b86); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rbx = ((P4) as u64) as u64; bb = 12; continue; }
            11 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rbx = ((P5) as u64) as u64; rax = ((P6) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 12; continue; }
            12 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63bce); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63bde); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P9) as u64) as u64; rcx = ((rsp.wrapping_add(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63bef); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e63c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1f08c; const P1: u64 = 0x7001e696a0; const P2: u64 = 0x7001e26f90; const P3: u64 = 0x7001e1f210; const P4: u64 = 0x7001e6bc10; const P5: u64 = 0x7001e6bc28; const P6: u64 = 0x7001e6bc40; const P7: u64 = 0x7001e1c690; const P8: u64 = 0x7001e1887c; const P9: u64 = 0x7001e76d68; const P10: u64 = 0x7001e66444; const P11: u64 = 0x7001e63d6c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[489], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rsi = ((r8 as u32)) as u32 as u64; r14 = (rdx) as u64; rbx = (rcx) as u64; rdx = (rcx) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63c26); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbx).wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u32(rsp.wrapping_add(0xc8u64), (0x0u32) as u32); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = (0x2u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); r9 = ((rsi as u32)) as u32 as u64; r8 = (r14) as u64; rdx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63c66); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e63c66, ({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e63c8b => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x7001e63c00) } }; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x48u64)))) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rdx) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; r8 = (if (((fl_l as u64)) == ((fl_r as u64))) { (rdi as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((r8 as u32)) as u32); bb = 3; continue; }
            2 => { rdi = (0x2u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; r8 = ((((r8 as u32)) | (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rdx = (!((rdx as u32))) as u32 as u64; rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((r8 as u32)))) as u32 as u64; rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = ((((rdx as u32)) & (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63cca); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e63cca, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e63c8b => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x7001e63c00) } }; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63cd8); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e63cd8, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e63c8b => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 2; continue; } _ => l.unknown(0x7001e63c00) } }; bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63cfb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            9 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rbx = ((P4) as u64) as u64; bb = 12; continue; }
            11 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rbx = ((P5) as u64) as u64; rax = ((P6) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 12; continue; }
            12 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63d46); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63d56); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P9) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e63d67); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e63dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x7001e6bc10; const P4: u64 = 0x7001e6bc28; const P5: u64 = 0x7001e6bc40; const P6: u64 = 0x7001e1c690; const P7: u64 = 0x7001e1887c; const P8: u64 = 0x7001e76d68; const P9: u64 = 0x7001e66444; const P10: u64 = 0x7001e63e47;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; r8 = (!((r8 as u32))) as u32 as u64; r8 = ((((r8 as u32)) & (0x4u32))) as u32 as u64; r8 = ((((r8 as u32)) | ((rdx as u32)))) as u32 as u64; r8 = ((((r8 as u32)) | (crate::simd_ops::ru32(rcx.wrapping_add(P1))))) as u32 as u64; r8 = ((((r8 as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32); fl_l = ((((r8 as u32)) & (crate::simd_ops::ru32(rcx.wrapping_add(P2))))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((P3) as u64) as u64; bb = 5; continue; }
            4 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P4) as u64) as u64; rax = ((P5) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 5; continue; }
            5 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P8) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P10, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e63e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x50; const P2: u64 = 0x1; const P3: u64 = 0x7001e696a0; const P4: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = ((((rbx as u32)) | (0xffffffffu32))) as u32 as u64; rdi = (rcx) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = ((((rbx as u32)) | ((rbx as u32)))) as u32 as u64; crate::simd_ops::lk_add_u32(rax, ((rbx as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::lk_add_u64(rax, (1) as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rbx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; bb = 11; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(rax, ((rbx as u32)) as u32)) as u32 as u64; fl_r = fl_l; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rdx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rbx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e63f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x50; const P2: u64 = 0x2; const P3: u64 = 0x7001e696a0; const P4: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (rcx) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::lk_sub_u32(rax, (1) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::lk_add_u64(rax, (0x2u64) as u64)) as u64; fl_r = fl_l; rax = (crate::simd_ops::ru64(rax)) as u64; rax = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; bb = 11; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_sub_u32(rax, (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rdx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rcx = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (((rax as u16)) as u32) as u32 as u64; bb = 6; continue; }
            6 => { rax = (0xffffu32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e63fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x50; const P4: u64 = 0x20; const P5: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (((rdx as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (r8) as u64 as u64;
    if (((fl_l as u64)) >= ((fl_r as u64))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (((rdx as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(r8.wrapping_add(P2))) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; crate::simd_ops::lk_add_u32(rax, (1) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::lk_sub_u64(rax, (1) as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (((rdx as u8)) as u32) as u32 as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e63fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x18; const P2: u64 = 0xfffffffffffffffe; const P3: u64 = 0x50; const P4: u64 = 0x20; const P5: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (r8) as u64 as u64;
    if (((fl_l as u64)) >= ((fl_r as u64))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    fl_l = ((rdx as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(r8.wrapping_add(P2))) as u16 as u64;
    if !((((fl_l as u16)) != ((fl_r as u16)))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; crate::simd_ops::lk_add_u32(rax, (1) as u32); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = ((rax).wrapping_add(0xfffffffffffffffeu64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x50; const P2: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0x0u32) as u32 as u64;
    if !(((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32))) { crate::simd_ops::lk_sub_u32(rax, (1) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::lk_add_u64(rax, (1) as u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x50; const P2: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0x0u32) as u32 as u64;
    if !(((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32))) { crate::simd_ops::lk_sub_u32(rax, (1) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::lk_add_u64(rax, (0x2u64) as u64)) as u64; fl_r = fl_l; return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x18; const P2: u64 = 0x50; const P3: u64 = 0x20; const P4: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = ((((rdx as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; crate::simd_ops::lk_add_u32(rax, (1) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::lk_sub_u64(rax, (1) as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = ((((rdx as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l;
    rdx = ((fl_l as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e640d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x18; const P2: u64 = 0x50; const P3: u64 = 0x20; const P4: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0xffffu32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; crate::simd_ops::lk_add_u32(rax, (1) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::lk_add_u64(rax, (0xfffffffffffffffeu64) as u64)) as u64; fl_r = fl_l; rax = (crate::simd_ops::ru64(rax)) as u64; rax = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (0xffffu32) as u32 as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64;
    crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1ced0; const P1: u64 = 0x58; const P2: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (rdx) as u64;
    rbx = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    r8 = (r8 & !0xffu64) | (((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64));
    rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rdi.wrapping_add(P1))) as u8 as u64));
    crate::simd_ops::wu8(rbx.wrapping_add(P1), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P1), ((r8 as u8)) as u8);
    r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (rax) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P2), (r8) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1ced0; const P1: u64 = 0x58; const P2: u64 = 0x50;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (rdx) as u64;
    rbx = (rcx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    r8 = ((crate::simd_ops::ru16(rbx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = ((crate::simd_ops::ru16(rdi.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu16(rbx.wrapping_add(P1), ((rax as u16)) as u16);
    rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    crate::simd_ops::wu16(rdi.wrapping_add(P1), ((r8 as u16)) as u16);
    r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (rax) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P2), (r8) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e641a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x7001e1ced0; const P2: u64 = 0x58; const P3: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; rcx = (rdi) as u64; rbx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdx)) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rdi.wrapping_add(P2))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P3), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P3), (rcx) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x7001e1ced0; const P2: u64 = 0x58; const P3: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; rcx = (rdi) as u64; rbx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdx)) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P2))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(P2))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(P2), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu16(rbx.wrapping_add(P2), ((rcx as u16)) as u16); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P3), (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P3), (rcx) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x7001e1ced0; const P2: u64 = 0x58; const P3: u64 = 0x50; const P4: u64 = 0x8;
    let (mut rax, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r14 = (rdx) as u64; rsi = (rcx) as u64; rdi = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; rcx = (rdi) as u64; rbx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdx)) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((crate::simd_ops::ru8(rdi.wrapping_add(P2))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P3), (rax) as u64); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P3), (rcx) as u64); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(P4))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P4), (rax) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(P4), (rcx) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e642e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x7001e1ced0; const P2: u64 = 0x58; const P3: u64 = 0x50; const P4: u64 = 0x8;
    let (mut rax, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r14 = (rdx) as u64; rsi = (rcx) as u64; rdi = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; rcx = (rdi) as u64; rbx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rdx)) as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P2))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(P2))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(P2), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu16(rbx.wrapping_add(P2), ((rcx as u16)) as u16); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P3), (rax) as u64); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P3), (rcx) as u64); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(P4))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(P4), (rax) as u64); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(P4), (rcx) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 2>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; r14 = (rdx) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; r15 = (rcx) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; r11 = (crate::simd_ops::ru64(r9)) as u64; r10 = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; r10 = (((r10 as u32)).wrapping_sub((rbx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; rdi = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; rdi = ((rdi).wrapping_add(rbp)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x58u64))) as u64; rdx = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; rdx = (((rdx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(r9, (rcx) as u64); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rax, (r8) as u64); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, (r11) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rax, (rbx) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu32(rax, ((r10 as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rdx = (((crate::simd_ops::ru32(rax)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; rdx = (((rdx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax, (r8) as u64); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rax, (rsi) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax, (rbp) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x60u64), (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x60u64), (rcx) as u64); r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 2; continue; }
            2 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 2>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; r14 = (rdx) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; r15 = (rcx) as u64; r11 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(r9)) as u64; r10 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x58u64))) as u64; r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(r9, (rcx) as u64); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rax, (rdx) as u64); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, (rbx) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rax, (r11) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rax, ((r10 as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax, (rdx) as u64); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rax, (rbp) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, (rsi) as u64); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x60u64), (rax) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x60u64), (rcx) as u64); r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); bb = 2; continue; }
            2 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0; const P1: u64 = 0x7001e5c910; const P2: u64 = 0x7001e6bc10; const P3: u64 = 0x7001e6bc28; const P4: u64 = 0x7001e6bc40; const P5: u64 = 0x7001e1c690; const P6: u64 = 0x7001e1887c; const P7: u64 = 0x7001e76d68; const P8: u64 = 0x7001e66444; const P9: u64 = 0x7001e646a4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[490], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6459a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e645a6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { bb = 13; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e645cc); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e645cc, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64621 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 10; continue; } _ => l.unknown(0x7001e64570) } }; rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(0x10u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0x13u32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(rbx).wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = ((((rcx as u32)) & (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; bb = 11; continue; }
            7 => { rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64616); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 14; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(0x48u64))) as u64; bb = 11; continue; }
            11 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64641); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 13; continue; }
            13 => { fl_l = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            15 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rbx = ((P2) as u64) as u64; bb = 18; continue; }
            17 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 18; continue; }
            18 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6467e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6468e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6469f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e646b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0; const P1: u64 = 0x7001e5cb70; const P2: u64 = 0x7001e6bc10; const P3: u64 = 0x7001e6bc28; const P4: u64 = 0x7001e6bc40; const P5: u64 = 0x7001e1c690; const P6: u64 = 0x7001e1887c; const P7: u64 = 0x7001e76d68; const P8: u64 = 0x7001e66444; const P9: u64 = 0x7001e647e4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[491], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e646da); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e646e6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { bb = 13; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6470c); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e6470c, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64761 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 10; continue; } _ => l.unknown(0x7001e646b0) } }; rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(0x10u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0x13u32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(rbx).wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = ((((rcx as u32)) & (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; bb = 11; continue; }
            7 => { rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64756); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 14; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(0x48u64))) as u64; bb = 11; continue; }
            11 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64781); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 13; continue; }
            13 => { fl_l = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            15 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rbx = ((P2) as u64) as u64; bb = 18; continue; }
            17 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 18; continue; }
            18 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e647be); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e647ce); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e647df); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e647f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0; const P1: u64 = 0x7001e5cdf0; const P2: u64 = 0x7001e6bc10; const P3: u64 = 0x7001e6bc28; const P4: u64 = 0x7001e6bc40; const P5: u64 = 0x7001e1c690; const P6: u64 = 0x7001e1887c; const P7: u64 = 0x7001e76d68; const P8: u64 = 0x7001e66444; const P9: u64 = 0x7001e64924;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[492], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6481a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64826); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { bb = 13; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6484c); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e6484c, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e648a1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 10; continue; } _ => l.unknown(0x7001e647f0) } }; rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(0x10u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0x13u32))) as u32 as u64; rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(rbx).wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = ((((rcx as u32)) & (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; bb = 11; continue; }
            7 => { rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64896); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 14; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(0x48u64))) as u64; bb = 11; continue; }
            11 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e648c1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 13; continue; }
            13 => { fl_l = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            15 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rbx = ((P2) as u64) as u64; bb = 18; continue; }
            17 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 18; continue; }
            18 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e648fe); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6490e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6491f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0; const P1: u64 = 0x7001e5c910;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[493], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), (rcx) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(rcx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64966); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64972); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbx).wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); r9 = (0x1u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (rdi) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e649a9); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e649a9, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e649d4 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } _ => l.unknown(0x7001e64930) } }; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e649c8); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e649c8, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e649d4 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } _ => l.unknown(0x7001e64930) } }; bb = 5; continue; }
            5 => { rax = (rdi) as u64; bb = 10; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 7; continue; }
            7 => { crate::simd_ops::lk_or_u64(rdi, (0xffffffffffffffffu64) as u64); crate::simd_ops::lk_and_u64(rdi.wrapping_add(0x8u64), (0x0u64) as u64); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64a05); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rax = (rdi) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0; const P1: u64 = 0x7001e5cb70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[494], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), (rcx) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(rcx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64a56); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64a62); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbx).wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); r9 = (0x1u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (rdi) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64a99); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e64a99, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64ac4 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } _ => l.unknown(0x7001e64a20) } }; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64ab8); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e64ab8, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64ac4 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } _ => l.unknown(0x7001e64a20) } }; bb = 5; continue; }
            5 => { rax = (rdi) as u64; bb = 10; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 7; continue; }
            7 => { crate::simd_ops::lk_or_u64(rdi, (0xffffffffffffffffu64) as u64); crate::simd_ops::lk_and_u64(rdi.wrapping_add(0x8u64), (0x0u64) as u64); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64af5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rax = (rdi) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0; const P1: u64 = 0x7001e5cdf0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[495], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), (rcx) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(rcx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64b46); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64b52); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbx).wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); r9 = (0x1u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (rdi) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64b89); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e64b89, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64bb4 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } _ => l.unknown(0x7001e64b10) } }; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64ba8); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e64ba8, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64bb4 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } _ => l.unknown(0x7001e64b10) } }; bb = 5; continue; }
            5 => { rax = (rdi) as u64; bb = 10; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 7; continue; }
            7 => { crate::simd_ops::lk_or_u64(rdi, (0xffffffffffffffffu64) as u64); crate::simd_ops::lk_and_u64(rdi.wrapping_add(0x8u64), (0x0u64) as u64); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64be5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rax = (rdi) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1f08c; const P1: u64 = 0x7001e696a0; const P2: u64 = 0x7001e26f90; const P3: u64 = 0x7001e1f210;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[496], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; rdx = (rcx) as u64; rcx = ((rax.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64c21); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdi).wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdi).wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); r9 = (0x1u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64c55); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e64c55, ({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64c99 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } _ => l.unknown(0x7001e64c00) } }; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64c5c); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e64c5c, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64c99 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } _ => l.unknown(0x7001e64c00) } }; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64c6a); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e64c6a, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64c99 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } _ => l.unknown(0x7001e64c00) } }; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64c8d); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e64c8d, ({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64c99 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } _ => l.unknown(0x7001e64c00) } }; bb = 5; continue; }
            5 => { rax = (rbx) as u64; bb = 12; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 7; continue; }
            7 => { crate::simd_ops::lk_or_u64(rbx, (0xffffffffffffffffu64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64cad); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e64cad, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64c99 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } _ => l.unknown(0x7001e64c00) } }; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64cbb); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e64cbb, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64c99 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 6; continue; } _ => l.unknown(0x7001e64c00) } }; bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64cde); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rax = (rbx) as u64; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0; const P1: u64 = 0x7001e5c910; const P2: u64 = 0x7001e6bc10; const P3: u64 = 0x7001e6bc28; const P4: u64 = 0x7001e6bc40; const P5: u64 = 0x7001e1c690; const P6: u64 = 0x7001e1887c; const P7: u64 = 0x7001e76d68; const P8: u64 = 0x7001e66444; const P9: u64 = 0x7001e64ef0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[497], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbx = (rcx) as u64; crate::simd_ops::lk_and_u64(rcx.wrapping_add(0x8u64), (0x0u64) as u64); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rsi = (crate::simd_ops::ru32(rdx.wrapping_add(rcx).wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rcx = (!((rcx as u32))) as u32 as u64; rcx = ((((rcx as u32)) & (0x4u32))) as u32 as u64; rax = ((rsi as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffeu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rbx).wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = ((((rcx as u32)) & (crate::simd_ops::ru32(rdx.wrapping_add(rbx).wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64d6f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64d7b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; crate::simd_ops::lk_add_u32(rax, (1) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; fl_l = (crate::simd_ops::lk_sub_u64(rax, (1) as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; bb = 8; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((((rdx as u32)) | (0xffffffffu32))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64dc8); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e64dc8, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64de1 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 11; continue; } _ => l.unknown(0x7001e64d00) } }; bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdi = ((rsi as u32)) as u32 as u64; fl_l = ((((rdi as u32)) | (0x4u32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((rdi as u32)) as u32); bb = 10; continue; }
            10 => { bb = 12; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rdi = ((((rdi as u32)) | (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rdx = (!((rdx as u32))) as u32 as u64; rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rdi as u32)))) as u32 as u64; rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = ((((rdx as u32)) & (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64e34); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 15; continue; }
            15 => { rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            16 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rbx = ((P2) as u64) as u64; bb = 19; continue; }
            18 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 19; continue; }
            19 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64e76); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64e86); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64e97); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 20; continue; }
            20 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rbx = ((P2) as u64) as u64; bb = 23; continue; }
            22 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 23; continue; }
            23 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64eca); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64eda); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64eeb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e64f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0; const P1: u64 = 0x7001e5cb70; const P2: u64 = 0x7001e6bc10; const P3: u64 = 0x7001e6bc28; const P4: u64 = 0x7001e6bc40; const P5: u64 = 0x7001e1c690; const P6: u64 = 0x7001e1887c; const P7: u64 = 0x7001e76d68; const P8: u64 = 0x7001e66444; const P9: u64 = 0x7001e6510c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[498], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbx = (rcx) as u64; crate::simd_ops::lk_and_u64(rcx.wrapping_add(0x8u64), (0x0u64) as u64); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; r14 = (crate::simd_ops::ru32(rdx.wrapping_add(rcx).wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rcx = (!((rcx as u32))) as u32 as u64; rcx = ((((rcx as u32)) & (0x4u32))) as u32 as u64; rax = ((r14 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffeu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rbx).wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = ((((rcx as u32)) & (crate::simd_ops::ru32(rdx.wrapping_add(rbx).wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64f77); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64f83); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; crate::simd_ops::lk_add_u32(rax, (1) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(rax, (0xfffffffffffffffeu64) as u64)) as u64; fl_r = fl_l; rax = (crate::simd_ops::ru64(rax)) as u64; rax = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rsi = (0xffffu32) as u32 as u64; bb = 8; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = (0xffffu32) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e64fda); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e64fda, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e64ff4 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 11; continue; } _ => l.unknown(0x7001e64f00) } }; bb = 8; continue; }
            8 => { fl_l = ((rsi as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdi = ((r14 as u32)) as u32 as u64; fl_l = ((((rdi as u32)) | (0x4u32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((rdi as u32)) as u32); bb = 10; continue; }
            10 => { bb = 12; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rdi = ((((rdi as u32)) | (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rdx = (!((rdx as u32))) as u32 as u64; rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rdi as u32)))) as u32 as u64; rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = ((((rdx as u32)) & (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65047); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 15; continue; }
            15 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            16 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rbx = ((P2) as u64) as u64; bb = 19; continue; }
            18 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 19; continue; }
            19 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65092); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e650a2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e650b3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 20; continue; }
            20 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rbx = ((P2) as u64) as u64; bb = 23; continue; }
            22 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 23; continue; }
            23 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e650e6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e650f6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65107); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e65110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0; const P1: u64 = 0x7001e5cdf0; const P2: u64 = 0x7001e6bc10; const P3: u64 = 0x7001e6bc28; const P4: u64 = 0x7001e6bc40; const P5: u64 = 0x7001e1c690; const P6: u64 = 0x7001e1887c; const P7: u64 = 0x7001e76d68; const P8: u64 = 0x7001e66444; const P9: u64 = 0x7001e6531c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[499], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbx = (rcx) as u64; crate::simd_ops::lk_and_u64(rcx.wrapping_add(0x8u64), (0x0u64) as u64); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; r14 = (crate::simd_ops::ru32(rdx.wrapping_add(rcx).wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rcx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rcx = (!((rcx as u32))) as u32 as u64; rcx = ((((rcx as u32)) & (0x4u32))) as u32 as u64; rax = ((r14 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffeu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rcx = ((((rcx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rbx).wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = ((((rcx as u32)) & (crate::simd_ops::ru32(rdx.wrapping_add(rbx).wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65187); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65193); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; crate::simd_ops::lk_add_u32(rax, (1) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(rax, (0xfffffffffffffffeu64) as u64)) as u64; fl_r = fl_l; rax = (crate::simd_ops::ru64(rax)) as u64; rax = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rsi = (0xffffu32) as u32 as u64; bb = 8; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = (0xffffu32) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e651ea); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e651ea, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65204 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 11; continue; } _ => l.unknown(0x7001e65110) } }; bb = 8; continue; }
            8 => { fl_l = ((rsi as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdi = ((r14 as u32)) as u32 as u64; fl_l = ((((rdi as u32)) | (0x4u32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((rdi as u32)) as u32); bb = 10; continue; }
            10 => { bb = 12; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; rdi = ((((rdi as u32)) | (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rdx = (!((rdx as u32))) as u32 as u64; rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rdi as u32)))) as u32 as u64; rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = ((((rdx as u32)) & (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rbx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65257); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 15; continue; }
            15 => { rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            16 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rbx = ((P2) as u64) as u64; bb = 19; continue; }
            18 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 19; continue; }
            19 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e652a2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e652b2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e652c3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 20; continue; }
            20 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rbx = ((P2) as u64) as u64; bb = 23; continue; }
            22 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P3) as u64) as u64; rax = ((P4) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 23; continue; }
            23 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e652f6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65306); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P7) as u64) as u64; rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65317); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e65320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1f08c; const P1: u64 = 0x7001e696a0; const P2: u64 = 0x7001e26f90; const P3: u64 = 0x7001e1f210; const P4: u64 = 0x7001e6bc10; const P5: u64 = 0x7001e6bc28; const P6: u64 = 0x7001e6bc40; const P7: u64 = 0x7001e1c690; const P8: u64 = 0x7001e1887c; const P9: u64 = 0x7001e76d68; const P10: u64 = 0x7001e66444; const P11: u64 = 0x7001e65479;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[500], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rsi = (r8) as u64; r14 = (rdx) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), ((rbx as u32)) as u32); rdx = (rcx) as u64; rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6534c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rbx.wrapping_add(0x4u64)) as u32) as u32 as u64; rbx = ((rdx as u32)) as u32 as u64; bb = 6; continue; }
            2 => { fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdi).wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (rsi) as u64; rdx = (r14) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6537d); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e6537d, ({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65397 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 4; continue; } _ => l.unknown(0x7001e65320) } }; rdx = (0x4u32) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (rsi) as u64 as u64; rbx = (if (((fl_l as u64)) != ((fl_r as u64))) { (rdx as u32) } else { (rbx as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), ((rbx as u32)) as u32); bb = 6; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; bb = 5; continue; }
            5 => { rdx = (0x4u32) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rbx = ((((rbx as u32)) | (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_l = 0; fl_r = (rax) as u64 as u64; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; r8 = (!((r8 as u32))) as u32 as u64; r8 = ((((r8 as u32)) & ((rdx as u32)))) as u32 as u64; r8 = ((((r8 as u32)) | ((rbx as u32)))) as u32 as u64; r8 = ((((r8 as u32)) & (0x17u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r8 as u32)) as u32); fl_l = ((((r8 as u32)) & (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e653d9); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e653d9, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65397 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 4; continue; } _ => l.unknown(0x7001e65320) } }; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e653e7); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e653e7, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65397 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 4; continue; } _ => l.unknown(0x7001e65320) } }; bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6540a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            11 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            12 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rbx = ((P4) as u64) as u64; bb = 15; continue; }
            14 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((P5) as u64) as u64; rax = ((P6) as u64) as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; bb = 15; continue; }
            15 => { rdx = (0x1u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65453); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r8 = (rax) as u64; rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65463); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((P9) as u64) as u64; rcx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65474); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P11, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e65484(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e65fd4; const P1: u64 = 0x8; const P2: u64 = 0x7001e9a210;
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
pub unsafe extern "win64" fn fn_7001e654c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e65fd4; const P1: u64 = 0x7001e65e28; const P2: u64 = 0x7001e1b0e0; const P3: u64 = 0x7001e664d8; const P4: u64 = 0x7001e664de;
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
pub unsafe extern "win64" fn fn_7001e654f4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6562c;
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
pub unsafe extern "win64" fn fn_7001e6550c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1b0a0;
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
pub unsafe extern "win64" fn fn_7001e65534(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1b0a0;
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
pub unsafe extern "win64" fn fn_7001e6554c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e65fd4; const P1: u64 = 0x7001e696a0; const P2: u64 = 0x7001e664d2;
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
    if !((((fl_l as u32)) != ((fl_r as u32)))) { r8 = (rsi) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbp) as u64; rax = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
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
pub unsafe extern "win64" fn fn_7001e655ac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e65fd4; const P1: u64 = 0x7001e9a220; const P2: u64 = 0x7001e664f0; const P3: u64 = 0x7001e163a0; const P4: u64 = 0x7001e664fc;
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
pub unsafe extern "win64" fn fn_7001e655dc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1b0a0;
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
pub unsafe extern "win64" fn fn_7001e655f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e9a218; const P1: u64 = 0x7001e65e28; const P2: u64 = 0x7001e1b0a0;
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
pub unsafe extern "win64" fn fn_7001e6562c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e9a219; const P1: u64 = 0x7001e65fd4; const P2: u64 = 0x7001e9a220; const P3: u64 = 0x7001e664e4; const P4: u64 = 0x7001e9a238; const P5: u64 = 0x7001e6ef10; const P6: u64 = 0x7001e9a230; const P7: u64 = 0x7001e9a248; const P8: u64 = 0x7001e65fe8; const P9: u64 = 0x7001e656b6;
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
pub unsafe extern "win64" fn fn_7001e656b8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e14000; const P1: u64 = 0x7001e1403c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[501], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x18u64)) as u64; r8 = (rcx) as u64; rax = (0x5a4du32) as u32 as u64; fl_l = (crate::simd_ops::ru16(P0)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 14; continue; } bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru32(P1)) as i32 as i64) as u64) as u64; rdx = ((P0) as u64) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = ((match eh.ru32(0x7001e656de, rcx) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65749 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x7001e656b8) } })) as u32 as u64; fl_r = (0x4550u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rax = (0x20bu32) as u32 as u64; fl_l = ((match eh.ru16(0x7001e656eb, rcx.wrapping_add(0x18u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65749 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x7001e656b8) } })) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 14; continue; } bb = 3; continue; }
            3 => { r8 = ((r8).wrapping_sub(rdx)) as u64; rdx = (((match eh.ru16(0x7001e656f4, rcx.wrapping_add(0x14u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65749 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x7001e656b8) } })) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rax = (((match eh.ru16(0x7001e656ff, rcx.wrapping_add(0x6u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65749 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x7001e656b8) } })) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r9 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp, (rdx) as u64); fl_l = (rdx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = ((match eh.ru32(0x7001e65714, rdx.wrapping_add(0xcu64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65749 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x7001e656b8) } })) as u32 as u64; fl_l = (r8) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((match eh.ru32(0x7001e6571c, rdx.wrapping_add(0x8u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65749 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x7001e656b8) } })) as u32 as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx).wrapping_add(0x28u64)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 4; continue; }
            8 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 16; continue; }
            11 => { fl_l = ((match eh.ru32(0x7001e65737, rdx.wrapping_add(0x24u64)) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65749 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 15; continue; } _ => l.unknown(0x7001e656b8) } })) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 12; continue; }
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
pub unsafe extern "win64" fn fn_7001e65750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e65fd4; const P1: u64 = 0x7001e9a210;
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
pub unsafe extern "win64" fn fn_7001e65774(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e9a218; const P1: u64 = 0x7001e1b0a0;
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
pub unsafe extern "win64" fn fn_7001e657a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e9a220; const P1: u64 = 0x7001e664f6; const P2: u64 = 0x7001e664ea;
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
pub unsafe extern "win64" fn fn_7001e657dc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e657a0;
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
pub unsafe extern "win64" fn fn_7001e657f4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x7001e65814;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r8 = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64;
    rcx = (rdx) as u64;
    rdx = (r9) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (0x1u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e65814(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x10; const P3: u64 = 0x3; const P4: u64 = 0x7001e658e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    r11 = (crate::simd_ops::ru32(r8)) as u32 as u64;
    rbx = (rdx) as u64;
    r11 = ((((r11 as u32)) & (0xfffffff8u32))) as u32 as u64;
    r9 = (rcx) as u64;
    fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    r10 = (rcx) as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; r10 = (((crate::simd_ops::ru32(r8.wrapping_add(P1))) as i32 as i64) as u64) as u64; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; r10 = ((r10).wrapping_add(rcx)) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = (((r10) & (rcx))) as u64 as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; }
    rax = ((((r11 as u32)) as i32 as i64) as u64) as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(r10))) as u64;
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax).wrapping_add(P3))) as u8 as u64; fl_r = (0xfu8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rax).wrapping_add(P3))) as u32) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff0u32))) as u32 as u64; fl_l = ((r9).wrapping_add(rax)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; }
    fl_l = (((r9) ^ (rdx))) as u64 as u64; fl_r = fl_l;
    r9 = ((fl_l as u64)) as u64;
    rcx = (r9) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e65880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
pub unsafe extern "win64" fn fn_7001e658e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e66170;
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
pub unsafe extern "win64" fn fn_7001e65900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6ef28; const P1: u64 = 0x7001e6649c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((P0) as u64) as u64;
    rbx = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rax) as u64);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (rbx) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e6592c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0xffffffffffffffc8; const P4: u64 = 0x8; const P5: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[502], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (r9) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (r8) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r14 = (r9) as u64; rbx = (r8) as u64; rsi = (rdx) as u64; crate::simd_ops::wu8(rax.wrapping_add(P3), (0x0u8) as u8); rdi = (rdx) as u64; rdi = (((rdi) as i64).wrapping_mul((r8) as i64) as u64) as u64; fl_l = ((rdi).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P4), (rdi) as u64); bb = 1; continue; }
            1 => { rax = (rbx) as u64; rbx = ((rbx).wrapping_sub(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rbx) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = ((rdi).wrapping_sub(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rdi) as u64); rcx = (rdi) as u64; rax = (r14) as u64; rdx = (crate::simd_ops::ru64(P5)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65984); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65984, (rdx) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e6592c) }; bb = 1; continue; }
            3 => { crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x38u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e65998(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e696a0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[503], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = (r9) as u64; rsi = (r8) as u64; r15 = (rdx) as u64; rdi = (rcx) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = ((rdi).wrapping_sub(r15)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rdi) as u64); rcx = (rdi) as u64; rax = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e659d6); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e659d6, ({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e659e3 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 4; continue; } _ => l.unknown(0x7001e65998) } }; fl_l = ((rbx).wrapping_add(1)) as u64 as u64; rbx = ((fl_l as u64)) as u64; bb = 1; continue; }
            3 => { bb = 4; continue; }
            4 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e659fc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e66508; const P1: u64 = 0x7001e664a2; const P2: u64 = 0x7001e4d480; const P3: u64 = 0x7001e65a31; const P4: u64 = 0x7001e1a9d8; const P5: u64 = 0x7001e65a37;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (rcx) as u64; bb = 3; continue; }
            1 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (rbx) as u64; bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = (rbx) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P3, rcx, rsp); return rax; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e65a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x38; const P5: u64 = 0x4; const P6: u64 = 0x7001e65814; const P7: u64 = 0x7001e6644a;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64; rsi = (rdx) as u64; r14 = (r8) as u64; rbp = (rcx) as u64; rdx = (r9) as u64; rcx = (rsi) as u64; rdi = (r9) as u64; r8 = ((rbx.wrapping_add(P5)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x66u8))) as u8 as u64)); fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rax = (0x1u32) as u32 as u64; r8 = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; r8 = (0u32.wrapping_sub((r8 as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = (rdi) as u64; r8 = (r14) as u64; rdx = (rsi) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e65ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e65534; const P1: u64 = 0x7001e6550c; const P2: u64 = 0x7001e65b10; const P3: u64 = 0x7001e65c28;
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
pub unsafe extern "win64" fn fn_7001e65b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e655f0; const P1: u64 = 0x7001e65484; const P2: u64 = 0x7001e9a208; const P3: u64 = 0x7001e654f4; const P4: u64 = 0x7001e663b4; const P5: u64 = 0x7001e6636c; const P6: u64 = 0x7001e66390; const P7: u64 = 0x7001e698a8; const P8: u64 = 0x7001e69898; const P9: u64 = 0x7001e66514; const P10: u64 = 0x7001e654c0; const P11: u64 = 0x7001e69890; const P12: u64 = 0x7001e696c8; const P13: u64 = 0x7001e6650e; const P14: u64 = 0x7001e65750; const P15: u64 = 0x7001e663ac; const P16: u64 = 0x7001e656b8; const P17: u64 = 0x7001e696a0; const P18: u64 = 0x7001e9a250; const P19: u64 = 0x7001e65fe8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[504], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rdx) as u64; r14 = (rcx) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65b2d); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65b2d, (P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65b3a); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65b3a, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; rbx = (rbx & !0xffu64) | ((((rax as u8)) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rax as u8)) as u8); rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(P2, (0x1u32) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65b5f); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65b5f, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65b68); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65b68, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65b6d); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65b6d, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65b72); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65b72, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; rdx = ((P7) as u64) as u64; rcx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65b85); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65b85, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65b8e); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65b8e, (P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((P11) as u64) as u64; rcx = ((P12) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65ba5); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65ba5, (P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; crate::simd_ops::wu32(P2, (0x2u32) as u32); rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = (rcx & !0xffu64) | ((((rbx as u8)) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65bb9); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65bb9, (P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65bc3); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65bc3, (P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; rbx = (rax) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65bd4); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65bd4, (P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = (rsi) as u64; rdx = (0x2u32) as u32 as u64; rcx = (r14) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = (crate::simd_ops::ru64(P17)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65bf2); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65bf2, (r9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(P18, (1) as u32)) as u32 as u64; rax = (0x1u32) as u32 as u64; bb = 12; continue; }
            11 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rcx = (0x7u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65c1f); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65c1f, (P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65b10) }; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e65c28(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e9a250; const P1: u64 = 0x7001e65484; const P2: u64 = 0x7001e9a208; const P3: u64 = 0x7001e655ac; const P4: u64 = 0x7001e6637c; const P5: u64 = 0x7001e663f0; const P6: u64 = 0x7001e65750; const P7: u64 = 0x7001e65774; const P8: u64 = 0x7001e655dc; const P9: u64 = 0x7001e65fe8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[505], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (rdi & !0xffu64) | ((((rcx as u8)) as u8 as u64)); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(P0, ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65c54); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65c54, (P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65c28) }; rbx = (rbx & !0xffu64) | ((((rax as u8)) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65c68); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65c68, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65c28) }; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65c6d); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65c6d, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65c28) }; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65c72); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65c72, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65c28) }; crate::simd_ops::lk_and_u32(P2, (0x0u32) as u32); rcx = (rcx & !0xffu64) | ((((rbx as u8)) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65c80); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65c80, (P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65c28) }; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rcx & !0xffu64) | ((((rdi as u8)) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65c8a); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65c8a, (P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65c28) }; rbx = (((rax as u8)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65c92); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65c92, (P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65c28) }; rax = ((rbx as u32)) as u32 as u64; bb = 2; continue; }
            5 => { rcx = (0x7u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65ca0); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65ca0, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x7001e65c28) }; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e65ca8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e9a250; const P1: u64 = 0x7001e6ef30; const P2: u64 = 0x7001e696a0; const P3: u64 = 0x7001e65ac0; const P4: u64 = 0x7001e1b0e0; const P5: u64 = 0x7001e65c28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[506], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r8) as u64; rdi = ((rdx as u32)) as u32 as u64; r14 = (rcx) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 21; continue; }
            3 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(P1)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); bb = 7; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65cfb); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65cfb, ({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65dc3 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x7001e65ca8) } }; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { r8 = (rsi) as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65d17); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65d17, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65dc3 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x7001e65ca8) } }; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 8; continue; }
            8 => { r8 = (rsi) as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65d32); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65d32, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65dc3 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x7001e65ca8) } }; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = (rsi) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65d4e); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65d4e, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65dc3 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x7001e65ca8) } }; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65d59); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65d59, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65dc3 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x7001e65ca8) } }; rax = (crate::simd_ops::ru64(P1)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = (rsi) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65d72); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65d72, ({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65dc3 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x7001e65ca8) } }; bb = 12; continue; }
            12 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 14; continue; }
            14 => { r8 = (rsi) as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65d89); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65d89, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65dc3 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x7001e65ca8) } }; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(P1)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rbx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); bb = 18; continue; }
            17 => { r8 = (rsi) as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e65db5); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x7001e65db5, ({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e65dc3 => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 19; continue; } _ => l.unknown(0x7001e65ca8) } }; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); bb = 18; continue; }
            18 => { bb = 20; continue; }
            19 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); bb = 20; continue; }
            20 => { rax = ((rbx as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e65de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e662c0; const P1: u64 = 0x7001e65ca8;
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
pub unsafe extern "win64" fn fn_7001e65e28(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e973e0; const P1: u64 = 0x7001e973d8; const P2: u64 = 0x7001e9a254; const P3: u64 = 0x7001e973d0; const P4: u64 = 0x7001e973d4;
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
pub unsafe extern "win64" fn fn_7001e65fd4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x7001e973f0; const P1: u64 = 0x7001e9a258;
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
pub unsafe extern "win64" fn fn_7001e66130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e690a8; const P1: u64 = 0x7001e690b0; const P2: u64 = 0x7001e690a0; const P3: u64 = 0x7001e69098;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (rcx) as u64;
    rcx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rcx = (rax) as u64;
    rdx = (0xc0000409u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e691c8; const P1: u64 = 0x7001e6618d; const P2: u64 = 0x7001e9a300; const P3: u64 = 0x7001e66244; const P4: u64 = 0x7001e9a3f8; const P5: u64 = 0x7001e9a398; const P6: u64 = 0x7001e9a270; const P7: u64 = 0x7001e9a380; const P8: u64 = 0x7001e9a260; const P9: u64 = 0x7001e9a264; const P10: u64 = 0x7001e9a278; const P11: u64 = 0x7001e9a280; const P12: u64 = 0x7001e97380; const P13: u64 = 0x7001e973c0; const P14: u64 = 0x7001e6ef38; const P15: u64 = 0x7001e66130;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rcx = (0x17u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (0x2u32) as u32 as u64; crate::lift_ops::trap(0xc0000409, P1, rcx, rsp); return rax; bb = 2; continue; }
            2 => { rcx = ((P2) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(P4, (rax) as u64); rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(P5, (rax) as u64); rax = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu64(P6, (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(P7, (rax) as u64); crate::simd_ops::wu32(P8, (0xc0000409u32) as u32); crate::simd_ops::wu32(P9, (0x1u32) as u32); crate::simd_ops::wu32(P10, (0x1u32) as u32); rax = (0x8u32) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x0u64) as i64) as u64) as u64; rcx = ((P11) as u64) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rax), (0x2u64) as u64); rax = (0x8u32) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x0u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(P12)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(rax).wrapping_add(0x20u64), (rcx) as u64); rax = (0x8u32) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x1u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(P13)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(rax).wrapping_add(0x20u64), (rcx) as u64); rcx = ((P14) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e662c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e97380; const P1: u64 = 0x7001e69130; const P2: u64 = 0x7001e691f0; const P3: u64 = 0x7001e69090; const P4: u64 = 0x7001e69198; const P5: u64 = 0x7001e973c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rbx = (0x2b992ddfa232u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbp.wrapping_add(0x10u64), (0x0u64) as u64); rcx = ((rbp.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rax as u32)) as u32 as u64; crate::simd_ops::lk_xor_u64(rbp.wrapping_sub(0x10u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rax as u32)) as u32 as u64; rcx = ((rbp.wrapping_add(0x18u64)) as u64) as u64; crate::simd_ops::lk_xor_u64(rbp.wrapping_sub(0x10u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rax = ((rax).wrapping_shl(32u32)) as u64; rax = (((rax) ^ (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))))) as u64; rax = (((rax) ^ (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))))) as u64; rax = (((rax) ^ (rcx))) as u64; rcx = (0xffffffffffffu64) as u64; rax = (((rax) & (rcx))) as u64; rcx = (0x2b992ddfa233u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::wu64(P0, (rax) as u64); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (!(rax)) as u64; crate::simd_ops::wu64(P5, (rax) as u64); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e255ec; const P1: u64 = 0x7001e66388;
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
pub unsafe extern "win64" fn fn_7001e66530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
pub unsafe extern "win64" fn fn_7001e665bd(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e15b9c;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x20u64), (0xfffffffeu32) as u32); rcx = ((rbp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e665e3(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e15b9c;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x2u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x20u64), (0xfffffffdu32) as u32); rcx = ((rbp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66609(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e15b9c;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x4u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x20u64), (0xfffffffbu32) as u32); rcx = ((rbp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e6662f(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e15b9c;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x8u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x20u64), (0xfffffff7u32) as u32); rcx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66655(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e693a8; const P1: u64 = 0x7001e97008; const P2: u64 = 0x7001e97010; const P3: u64 = 0x7001e97000; const P4: u64 = 0x7001e15c30; const P5: u64 = 0x8; const P6: u64 = 0x7001e696a0; const P7: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[507], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rbp = (rdx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e66667); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x28u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), (0x8u32) as u32); crate::simd_ops::lk_add_u32(P1, (1) as u32); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (rax) as u64); rdx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e666a5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0x8u32) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff7u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P5), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e666d8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P7), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e666f6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { rax = (0x0u64) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e6670f(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e66438; const P1: u64 = 0x7001e693a8; const P2: u64 = 0x7001e693c8; const P3: u64 = 0x7001e970c8; const P4: u64 = 0x7001e970d0; const P5: u64 = 0x7001e970c0; const P6: u64 = 0x7001e15c30; const P7: u64 = 0x7001e696a0; const P8: u64 = 0x7001e160d1; const P9: u64 = 0x7001e16254; const P10: u64 = 0x7001e6b3f8; const P11: u64 = 0x7001e66450; const P12: u64 = 0x7001e66456; const P13: u64 = 0x7001e160d3; const P14: u64 = 0x7001e160d5; const P15: u64 = 0x7001e160dc; const P16: u64 = 0x7001e160d7; const P17: u64 = 0x7001e97008; const P18: u64 = 0x7001e97010; const P19: u64 = 0x7001e97000; const P20: u64 = 0x7001e160c1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[508], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e66721); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = (0xe0434f4du32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = (0xe0434352u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; fl_r = (0x19930520u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; fl_r = (0x19930521u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; fl_r = (0x19930522u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; fl_r = (0x1994000u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x38u64)))) as u64; rbx = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x38u64)))) as u64; rdi = (((crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as i32 as i64) as u64) as u64; rdi = ((rdi).wrapping_add(0xb0u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e667ac); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e667b5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x38u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), (0x4u32) as u32); crate::simd_ops::lk_add_u32(P3, (1) as u32); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), (rax) as u64); rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (rax) as u64); rdx = ((rbp.wrapping_add(0x38u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e66802); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((r14.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffbu32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e66834); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e66852); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 14; continue; }
            14 => { rax = ((P8) as u64) as u64; bb = 24; continue; }
            15 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 16; continue; }
            16 => { rcx = ((r14.wrapping_add(0xb0u64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; r8 = (rbx) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6687b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x7001e6687b, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => match l.cont_va { 0x7001e6694c => { rax = crate::dll_native::lift::eh::landed_rax(); bb = 17; continue; } _ => l.unknown(0x7001e6670f) } }; rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(r14, (xmm0) as u128); crate::simd_ops::wu32(r14.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), (0x1u32) as u32); rax = ((P10) as u64) as u64; crate::simd_ops::wu64(r14, (rax) as u64); crate::simd_ops::lk_and_u64(r14.wrapping_add(0xa8u64), (0x0u64) as u64); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x14u64), ((rax as u32)) as u32); crate::simd_ops::lk_and_u64(r14.wrapping_add(0x18u64), (0x0u64) as u64); crate::simd_ops::lk_and_u64(r14.wrapping_add(0x20u64), (0x0u64) as u64); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xc0u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(r14.wrapping_add(0x28u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xb8u64), (0xfu32) as u32); rax = ((rbp.wrapping_add(0xb8u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0xc0u64)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0xfu32) as u32 as u64; rax = (if (((fl_l as u32)) <= ((fl_r as u32))) { rdx } else { rax }) as u64; rbx = (crate::simd_ops::ru32(rax)) as u32 as u64; r8 = ((rbx.wrapping_mul(8)) as u64) as u64; rdx = ((rsi.wrapping_add(0x20u64)) as u64) as u64; rcx = ((r14.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e66902); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (0xfu32) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((rbx as u32))) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rcx = ((rbx.wrapping_add(0x6u64)) as u64) as u64; rcx = ((r14.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6691e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((r14.wrapping_add(0xb0u64)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x38u64), (rax) as u64); rax = ((r14.wrapping_add(0x10u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (r14) as u64); rax = ((P13) as u64) as u64; bb = 24; continue; }
            17 => { rax = ((P14) as u64) as u64; bb = 24; continue; }
            18 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdx = (rsi) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e66966); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P16) as u64) as u64; bb = 24; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e6697c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), (0x2u32) as u32); crate::simd_ops::lk_add_u32(P17, (1) as u32); rax = ((P18) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), (rax) as u64); rax = ((P19) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), (rax) as u64); rdx = ((rbp.wrapping_add(0x60u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e669ba); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0x2u32) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffdu32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e669ed); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e66a0b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 23; continue; }
            23 => { rax = ((P20) as u64) as u64; bb = 24; continue; }
            24 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66a25(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e15b9c;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x20u64), (0xfffffffeu32) as u32); rcx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66a63(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x48u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66b35(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x10u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66b55(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x40u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66b75(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x18u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66b95(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x30u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66bb5(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x60u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66be1(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18e9c;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x20u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66c1f(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x48u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66c6d(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[509], rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66c97(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x48; const P3: u64 = 0x14; const P4: u64 = 0x7001e66444;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    r8 = ((r8).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rdx = (!((rdx as u32))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64;
    rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rdx as u32)) as u32);
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66d3c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18dc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xb0u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0xb0u64), (0xfffffffeu32) as u32); rcx = ((rbp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66dbc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x30u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66e0c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18e9c;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x20u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66e32(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x38u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66e8b(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x20u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66ea8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18bfc;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x20u64), (0xfffffffeu32) as u32); rcx = ((rbp.wrapping_add(0xa0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66ed1(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x78u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66ef1(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x58u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66f1d(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x18u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66f55(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18bfc;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x34u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66f8a(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1de14;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x2u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x34u64), (0xfffffffdu32) as u32); rcx = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e66fdc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1de14;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x8u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x34u64), (0xfffffff7u32) as u32); rcx = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e6709f(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x30u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e670bc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x78u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e670d9(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e6649c;
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rdx = (0x58u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e6717a(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e1de14;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x30u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e671c4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18bfc;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x2u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x34u64), (0xfffffffdu32) as u32); rcx = ((rbp.wrapping_add(0x98u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67216(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18bfc;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x34u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x8u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x34u64), (0xfffffff7u32) as u32); rcx = ((rbp.wrapping_add(0x98u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e672e5(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18bfc;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x30u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e6733b(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x48; const P3: u64 = 0x14; const P4: u64 = 0x7001e66444;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xe0u64))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    r8 = ((r8).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rdx = (!((rdx as u32))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64;
    rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rdx as u32)) as u32);
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e673a4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x50u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_add(0x20u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e673ee(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x40u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rcx = ((rcx).wrapping_add(0x20u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67488(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x50u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e674b2(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x40u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e674dc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x50u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67506(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x40u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e6757c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e6759a(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x48; const P3: u64 = 0x14; const P4: u64 = 0x7001e66444;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    r8 = ((r8).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rdx = (!((rdx as u32))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64;
    rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rdx as u32)) as u32);
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67603(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x48; const P3: u64 = 0x14; const P4: u64 = 0x7001e66444;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    r8 = ((r8).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rdx = (!((rdx as u32))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64;
    rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rdx as u32)) as u32);
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e6766c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2; const P2: u64 = 0x7001e1ce20; const P3: u64 = 0x7001e66444;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rdx) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(P1)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e676a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x48; const P3: u64 = 0x14; const P4: u64 = 0x7001e66444;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    r8 = ((r8).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rdx = (!((rdx as u32))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64;
    rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rdx as u32)) as u32);
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e676fd(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x2; const P2: u64 = 0x7001e1ce20; const P3: u64 = 0x7001e66444;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (rdx) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(P1)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67761(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x48; const P3: u64 = 0x14; const P4: u64 = 0x7001e66444;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    r8 = ((r8).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rdx = (!((rdx as u32))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64;
    rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rdx as u32)) as u32);
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e677d6(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e18e9c;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::lk_and_u32(rbp.wrapping_add(0x20u64), (0xfffffffeu32) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e677fc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x48; const P3: u64 = 0x14; const P4: u64 = 0x7001e66444;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    r8 = ((r8).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rdx = (!((rdx as u32))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64;
    rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rdx as u32)) as u32);
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67859(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x48; const P3: u64 = 0x14; const P4: u64 = 0x7001e66444;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    r8 = ((r8).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rdx = (!((rdx as u32))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64;
    rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rdx as u32)) as u32);
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e678c2(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x48; const P3: u64 = 0x14; const P4: u64 = 0x7001e66444;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    r8 = ((r8).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rdx = (!((rdx as u32))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64;
    rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rdx as u32)) as u32);
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e6792b(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x48; const P3: u64 = 0x14; const P4: u64 = 0x7001e66444;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbp = (rdx) as u64;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (((crate::simd_ops::ru32(rax.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    r8 = ((r8).wrapping_add(rcx)) as u64;
    rcx = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    rcx = ((((rcx as u32)) | (0x4u32))) as u32 as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    fl_l = 0; fl_r = (rax) as u64 as u64;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rdx = (!((rdx as u32))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x4u32))) as u32 as u64;
    rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x17u32))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P1), ((rdx as u32)) as u32);
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = (0x0u64) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67985(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
pub unsafe extern "win64" fn fn_7001e6799d(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e65998;
    let (mut rax, mut fl_l, mut fl_r, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbp = (rdx) as u64;
    fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e679c9(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e66438; const P1: u64 = 0x8; const P2: u64 = 0x7001e66490; const P3: u64 = 0x7001e66502;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rbp = (rdx) as u64;
    crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (rcx) as u64);
    crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (rcx) as u64);
    rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rcx) as u64);
    rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64;
    fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = (0xe06d7363u32) as u32 as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), (0x0u32) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; } else { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rax, (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    rbp = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67a28(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e65750;
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
pub unsafe extern "win64" fn fn_7001e67a3f(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e65750;
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
pub unsafe extern "win64" fn fn_7001e67a58(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e655dc;
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
pub unsafe extern "win64" fn fn_7001e67a6c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e65ac0; const P1: u64 = 0x7001e6554c;
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
pub unsafe extern "win64" fn fn_7001e67ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e98bb0; const P1: u64 = 0x4; const P2: u64 = 0x7001e6bc90; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x7001e6bc08; const P6: u64 = 0x7001e98bc0; const P7: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[510], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((P0) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8), (rax) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P4), ((rdx as u32)) as u32);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(P6, (rax) as u64);
    rcx = ((P6) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e67b19); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e98d90; const P1: u64 = 0x4; const P2: u64 = 0x7001e6be18; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x7001e6bc08; const P6: u64 = 0x7001e98da8; const P7: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[511], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((P0) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8), (rax) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P4), ((rdx as u32)) as u32);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(P6, (rax) as u64);
    rcx = ((P6) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e67b99); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e98ec0; const P1: u64 = 0x4; const P2: u64 = 0x7001e6bc90; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x7001e6bc08; const P6: u64 = 0x7001e98ed0; const P7: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[512], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((P0) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8), (rax) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P4), ((rdx as u32)) as u32);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(P6, (rax) as u64);
    rcx = ((P6) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e67c19); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e98f40; const P1: u64 = 0x4; const P2: u64 = 0x7001e6bc90; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x7001e6bc08; const P6: u64 = 0x7001e98f50; const P7: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[513], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((P0) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8), (rax) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P4), ((rdx as u32)) as u32);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(P6, (rax) as u64);
    rcx = ((P6) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e67c99); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67cd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e99110; const P1: u64 = 0x7001e97238; const P2: u64 = 0x7001e69238; const P3: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[514], rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; bb = 3; continue; }
            1 => { rcx = ((P0) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P1, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e67ceb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x7001e67cf6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(P1)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0xau64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x7001e991e0; const P2: u64 = 0x10; const P3: u64 = 0x7001e696a0; const P4: u64 = 0x7001e6649c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; bb = 4; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(P1, (rax) as u64); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x7001e66530 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rdx = (0x10u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(P1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e991d0; const P1: u64 = 0x10; const P2: u64 = 0x7001e696a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
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
pub unsafe extern "win64" fn fn_7001e67de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e99570; const P1: u64 = 0x4; const P2: u64 = 0x7001e6d3c8; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x7001e6bc08; const P6: u64 = 0x7001e99580; const P7: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[515], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((P0) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8), (rax) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P4), ((rdx as u32)) as u32);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(P6, (rax) as u64);
    rcx = ((P6) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e67e29); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e99740; const P1: u64 = 0x4; const P2: u64 = 0x7001e6d3e0; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x7001e6bc08; const P6: u64 = 0x7001e99758; const P7: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[516], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((P0) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8), (rax) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P4), ((rdx as u32)) as u32);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(P6, (rax) as u64);
    rcx = ((P6) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e67ea9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e997d0; const P1: u64 = 0x4; const P2: u64 = 0x7001e6d3c8; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x7001e6bc08; const P6: u64 = 0x7001e997e0; const P7: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[517], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((P0) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8), (rax) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P4), ((rdx as u32)) as u32);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(P6, (rax) as u64);
    rcx = ((P6) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e67f29); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e998f0; const P1: u64 = 0x4; const P2: u64 = 0x7001e6d3c8; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x7001e6bc08; const P6: u64 = 0x7001e99900; const P7: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[518], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((P0) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8), (rax) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P4), ((rdx as u32)) as u32);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(P6, (rax) as u64);
    rcx = ((P6) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e67fa9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e67fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e99a30; const P1: u64 = 0x4; const P2: u64 = 0x7001e6d508; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x7001e6bc08; const P6: u64 = 0x7001e99a40; const P7: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[519], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((P0) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8), (rax) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P4), ((rdx as u32)) as u32);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(P6, (rax) as u64);
    rcx = ((P6) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e68029); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e68060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e99c00; const P1: u64 = 0x4; const P2: u64 = 0x7001e6d520; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x7001e6bc08; const P6: u64 = 0x7001e99c18; const P7: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[520], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((P0) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8), (rax) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P4), ((rdx as u32)) as u32);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(P6, (rax) as u64);
    rcx = ((P6) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e680a9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e680e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e99c90; const P1: u64 = 0x4; const P2: u64 = 0x7001e6d508; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x7001e6bc08; const P6: u64 = 0x7001e99ca0; const P7: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[521], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((P0) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8), (rax) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P4), ((rdx as u32)) as u32);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(P6, (rax) as u64);
    rcx = ((P6) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e68129); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_7001e68160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7001e99db0; const P1: u64 = 0x4; const P2: u64 = 0x7001e6d508; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0xfffffffffffffffc; const P5: u64 = 0x7001e6bc08; const P6: u64 = 0x7001e99dc0; const P7: u64 = 0x7001e1efd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&super::EH_META[522], rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    r8 = ((P0) as u64) as u64;
    rax = ((P2) as u64) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(r8), (rax) as u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (((crate::simd_ops::ru32(rax.wrapping_add(P1))) as i32 as i64) as u64) as u64;
    rdx = ((rcx.wrapping_add(P3)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(P4), ((rdx as u32)) as u32);
    rax = ((P5) as u64) as u64;
    crate::simd_ops::wu64(P6, (rax) as u64);
    rcx = ((P6) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x7001e681a9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    return rax;
}
