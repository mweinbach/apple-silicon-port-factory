#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_14169d070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x48; const P2: u64 = 0x141834d30; const P3: u64 = 0x50; const P4: u64 = 0x142646540; const P5: u64 = 0x10; const P6: u64 = 0x18; const P7: u64 = 0x20; const P8: u64 = 0x28; const P9: u64 = 0x30; const P10: u64 = 0x38; const P11: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rdi);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdi);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rdi); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P11), rax);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169d0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14169a060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x34u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r8); r13 = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rcx)) as u64; r12 = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_r = r12; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rax = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r13 as u32)) as u32); r9 = rsp.wrapping_add(0x48); r8 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 18; continue; } bb = 6; continue; }
            6 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (((rax) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r14 = ((r14).wrapping_add(rax)) as u64; fl_l = ((r15).wrapping_add(rdi)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { r12 = ((r12).wrapping_add(1)) as u64; rax = rbp; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rbp } else { rax }) as u64; rbp = rax; rax = rsi; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rsi } else { rax }) as u64; rsi = rax; rax = rdi; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rdi } else { rax }) as u64; rdi = rax; fl_l = r12; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 9; continue; }
            9 => { bb = 17; continue; }
            10 => { r12 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_r = r12; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_r = rbp; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rax = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r13 as u32)) as u32); r9 = rsp.wrapping_add(0x48); r8 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x40u64)))) as u64; r15 = ((r15).wrapping_add(1)) as u64; rbp = ((rbp).wrapping_add(1)) as u64; rax = rsi; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rsi } else { rax }) as u64; rsi = rax; rax = rdi; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rdi } else { rax }) as u64; rdi = rax; fl_l = rbp; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 16; continue; }
            16 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = r12; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax, r15); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax, r14); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169d2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14169a060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); r14 = r8; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r9 as u32)) as u32); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x34u64))) as u32 as u64; r13 = ((rcx.wrapping_add(rdx)) as u64) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 29; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(r14)) as u64; r15 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 7; continue; }
            7 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x30u64))) as u32 as u64; rax = rsp.wrapping_add(0x48); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r9 as u32)) as u32); r8 = r15; r9 = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 29; continue; } bb = 9; continue; }
            9 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { fl_l = rsi; fl_r = r12; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rbp); rsi = ((rsi).wrapping_add(1)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), r15); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); rdi = ((rdi).wrapping_add(r8)) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r8); rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rdi; fl_r = r13; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 29; continue; } bb = 13; continue; }
            13 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = r15; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { r15 } else { rax }) as u64; r15 = rax; rax = rbp; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rbp } else { rax }) as u64; rbp = rax; rax = rbx; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rbx } else { rax }) as u64; rbx = rax; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 15; continue; }
            15 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            16 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 18; continue; }
            18 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { rbp = (crate::simd_ops::ru64(r14)) as u64; r12 = 0; fl_l = 0; fl_r = 0; r15 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 21; continue; }
            21 => { rax = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rbx = r8.wrapping_add(0x10); fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { fl_l = rsi; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x30u64))) as u32 as u64; rax = rsp.wrapping_add(0x48); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r9 as u32)) as u32); r8 = r15; r9 = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 29; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), rbp); rsi = ((rsi).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), r15); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); rdi = ((rdi).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rdi; fl_r = r13; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 29; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; r12 = ((r12).wrapping_add(1)) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = r15; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { r15 } else { rax }) as u64; r15 = rax; rax = rbp; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rbp } else { rax }) as u64; rbp = rax; fl_l = r12; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 27; continue; }
            27 => { rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); fl_l = rdx; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 28; continue; }
            28 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169d560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x14183acc2; const P2: u64 = 0x141699fb0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; r10 = (rdx) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 76; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xbcu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 76; continue; } bb = 2; continue; }
            2 => { r11 = ((P0) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(r11.wrapping_add(rax).wrapping_add(0x169d950u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(r11.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x169d930u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(r11)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = (rdx) as u64; bb = 0xffff_fffe; continue; }
            3 => { fl_l = (r9) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 78; continue; } bb = 4; continue; }
            4 => { fl_l = (r10) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 78; continue; } bb = 5; continue; }
            5 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r11 = (0x3f800000u32) as u32 as u64; bb = 8; continue; }
            7 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; rax = (0x7fffffffu32) as u32 as u64; r11 = (0xffffffffu32) as u32 as u64; r11 = (if (((fl_l as u32)) == ((fl_r as u32))) { (rax as u32) } else { (r11 as u32) }) as u32 as u64; bb = 8; continue; }
            8 => { fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rax = ((r10.wrapping_sub(0xfu64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 10; continue; }
            10 => { rax = ((rax).wrapping_sub(1)) as u64; rax = ((rax).wrapping_shr(4u32)) as u64; fl_l = ((rax).wrapping_add(1)) as u64 as u64; rax = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((r11 as u32)) as u32); rcx = ((rcx.wrapping_add(0x10u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            13 => { fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { r10 }) as u64; rdx = ((r10.wrapping_sub(0xfu64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_sub(1)) as u64; rdx = ((rdx).wrapping_shr(4u32)) as u64; fl_l = ((rdx).wrapping_add(1)) as u64 as u64; rdx = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(r8)) as u32 as u64; r8 = ((r8.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = ((rcx.wrapping_add(0x10u64)) as u64) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_sub(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_sub(0x4u64), ((r11 as u32)) as u32); fl_l = (rdx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            17 => { fl_l = (r9) as u64 as u64; fl_r = (0x8u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 78; continue; } bb = 18; continue; }
            18 => { fl_l = (r10) as u64 as u64; fl_r = (0x8u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 78; continue; } bb = 19; continue; }
            19 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0xau32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { r11 = (0x3c00u32) as u32 as u64; bb = 22; continue; }
            21 => { r11 = (0xffffu32) as u32 as u64; rdx = ((rbx.wrapping_sub(0xdu64)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = (0x7fffu32) as u32 as u64; rax = (rax & !0xffffu64) | (((if (((fl_l as u32)) > ((fl_r as u32))) { (r11 as u16) } else { (rax as u16) }) as u16 as u64)); r11 = (((rax as u16)) as u32) as u32 as u64; bb = 22; continue; }
            22 => { fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 23; continue; }
            23 => { rax = ((r10.wrapping_sub(0x7u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 24; continue; }
            24 => { rax = ((rax).wrapping_sub(1)) as u64; rax = ((rax).wrapping_shr(3u32)) as u64; fl_l = ((rax).wrapping_add(1)) as u64 as u64; rax = ((fl_l as u64)) as u64; bb = 25; continue; }
            25 => { crate::simd_ops::wu16(rcx.wrapping_add(0x6u64), ((r11 as u16)) as u16); rcx = ((rcx.wrapping_add(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 26; continue; }
            26 => { rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            27 => { fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; r9 = (if (((fl_l as u64)) >= ((fl_r as u64))) { r10 } else { r9 }) as u64; rdx = ((r9.wrapping_sub(0x7u64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 28; continue; }
            28 => { rdx = ((rdx).wrapping_sub(1)) as u64; rdx = ((rdx).wrapping_shr(3u32)) as u64; fl_l = ((rdx).wrapping_add(1)) as u64 as u64; rdx = ((fl_l as u64)) as u64; bb = 29; continue; }
            29 => { rax = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; r8 = ((r8.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu16(rcx, ((rax as u16)) as u16); rcx = ((rcx.wrapping_add(0x8u64)) as u64) as u64; rax = ((crate::simd_ops::ru16(r8.wrapping_sub(0x6u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_sub(0x6u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(r8.wrapping_sub(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_sub(0x4u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rcx.wrapping_sub(0x2u64), ((r11 as u16)) as u16); fl_l = (rdx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 30; continue; }
            30 => { rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            31 => { fl_l = (r9) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 78; continue; } bb = 32; continue; }
            32 => { fl_l = (r10) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 78; continue; } bb = 33; continue; }
            33 => { fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 34; continue; }
            34 => { rax = ((r10.wrapping_sub(0x3u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 35; continue; }
            35 => { rax = ((rax).wrapping_sub(1)) as u64; rax = ((rax).wrapping_shr(2u32)) as u64; fl_l = ((rax).wrapping_add(1)) as u64 as u64; rax = ((fl_l as u64)) as u64; bb = 36; continue; }
            36 => { crate::simd_ops::lk_or_u32(rcx, (0xc0000000u32) as u32); rcx = ((rcx.wrapping_add(0x4u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 37; continue; }
            37 => { rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            38 => { fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; r9 = (if (((fl_l as u64)) >= ((fl_r as u64))) { r10 } else { r9 }) as u64; fl_l = ((r9).wrapping_add(0xfffffffffffffffdu64)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 78; continue; } bb = 39; continue; }
            39 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(rdx))) as u32 as u64; rax = ((((rax as u32)) | (0xc0000000u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 40; continue; } bb = 41; continue; }
            41 => { rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            42 => { fl_l = (r9) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 78; continue; } bb = 43; continue; }
            43 => { fl_l = (r10) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 78; continue; } bb = 44; continue; }
            44 => { rax = ((rbx.wrapping_sub(0x1fu64)) as u32) as u32 as u64; rdx = (0xff000000u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; r11 = (0x7f000000u32) as u32 as u64; r11 = (if (((fl_l as u32)) > ((fl_r as u32))) { (rdx as u32) } else { (r11 as u32) }) as u32 as u64; fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 49; continue; } bb = 45; continue; }
            45 => { rax = ((r10.wrapping_sub(0x3u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 46; continue; }
            46 => { rax = ((rax).wrapping_sub(1)) as u64; rax = ((rax).wrapping_shr(2u32)) as u64; fl_l = ((rax).wrapping_add(1)) as u64 as u64; rax = ((fl_l as u64)) as u64; bb = 47; continue; }
            47 => { crate::simd_ops::lk_and_u32(rcx, (0xffffffu32) as u32); crate::simd_ops::lk_or_u32(rcx, ((r11 as u32)) as u32); rcx = ((rcx.wrapping_add(0x4u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 47; continue; } bb = 48; continue; }
            48 => { rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            49 => { fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; r10 = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { r10 }) as u64; fl_l = ((r10).wrapping_add(0xfffffffffffffffdu64)) as u64 as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 78; continue; } bb = 50; continue; }
            50 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = ((((rax as u32)) | ((r11 as u32)))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 51; continue; } bb = 52; continue; }
            52 => { rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            53 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 78; continue; } bb = 54; continue; }
            54 => { fl_l = (r10) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 78; continue; } bb = 55; continue; }
            55 => { fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 60; continue; } bb = 56; continue; }
            56 => { rax = ((r10.wrapping_sub(0x1u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 57; continue; }
            57 => { rax = ((rax).wrapping_sub(1)) as u64; r9 = (0x8000u32) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = ((rax).wrapping_add(1)) as u64 as u64; rax = ((fl_l as u64)) as u64; bb = 58; continue; }
            58 => { crate::simd_ops::lk_or_u16(rcx, ((r9 as u16)) as u16); rcx = ((rcx.wrapping_add(0x2u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 58; continue; } bb = 59; continue; }
            59 => { rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            60 => { fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; r9 = (if (((fl_l as u64)) >= ((fl_r as u64))) { r10 } else { r9 }) as u64; r10 = ((r9.wrapping_sub(0x1u64)) as u64) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 61; continue; }
            61 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r9 = (0x8000u32) as u32 as u64; bb = 62; continue; }
            62 => { rax = ((crate::simd_ops::ru16(rdx.wrapping_add(r8))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) | ((r9 as u16)))) as u16 as u64)); crate::simd_ops::wu16(rdx.wrapping_add(rcx), ((rax as u16)) as u16); rdx = ((rdx).wrapping_add(0x2u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 62; continue; } bb = 63; continue; }
            63 => { rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            64 => { r8 = (r10) as u64; rdx = (0xffu32) as u32 as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            65 => { fl_l = (r9) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 78; continue; } bb = 66; continue; }
            66 => { fl_l = (r10) as u64 as u64; fl_r = (0x2u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 78; continue; } bb = 67; continue; }
            67 => { fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 72; continue; } bb = 68; continue; }
            68 => { rax = ((r10.wrapping_sub(0x1u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 69; continue; }
            69 => { rax = ((rax).wrapping_sub(1)) as u64; r9 = (0xf000u32) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = ((rax).wrapping_add(1)) as u64 as u64; rax = ((fl_l as u64)) as u64; bb = 70; continue; }
            70 => { crate::simd_ops::lk_or_u16(rcx, ((r9 as u16)) as u16); rcx = ((rcx.wrapping_add(0x2u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 70; continue; } bb = 71; continue; }
            71 => { rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            72 => { fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; r9 = (if (((fl_l as u64)) >= ((fl_r as u64))) { r10 } else { r9 }) as u64; r10 = ((r9.wrapping_sub(0x1u64)) as u64) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 73; continue; }
            73 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r9 = (0xf000u32) as u32 as u64; bb = 74; continue; }
            74 => { rax = ((crate::simd_ops::ru16(rdx.wrapping_add(r8))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) | ((r9 as u16)))) as u16 as u64)); crate::simd_ops::wu16(rdx.wrapping_add(rcx), ((rax as u16)) as u16); rdx = ((rdx).wrapping_add(0x2u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 74; continue; } bb = 75; continue; }
            75 => { rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            76 => { fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 78; continue; } bb = 77; continue; }
            77 => { fl_l = (r9) as u64 as u64; fl_r = (r10) as u64 as u64; rdx = (r10) as u64; r9 = (if (((fl_l as u64)) >= ((fl_r as u64))) { r10 } else { r9 }) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            78 => { rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14169d560u64) { 0x10 => { bb = 1; continue; } 0x22 => { bb = 2; continue; } 0x41 => { bb = 3; continue; } 0x4b => { bb = 4; continue; } 0x55 => { bb = 5; continue; } 0x5a => { bb = 6; continue; } 0x62 => { bb = 7; continue; } 0x74 => { bb = 8; continue; } 0x79 => { bb = 9; continue; } 0x86 => { bb = 10; continue; } 0x90 => { bb = 11; continue; } 0x9e => { bb = 12; continue; } 0xa0 => { bb = 13; continue; } 0xb4 => { bb = 14; continue; } 0xc0 => { bb = 15; continue; } 0xe5 => { bb = 16; continue; } 0xe7 => { bb = 17; continue; } 0xf1 => { bb = 18; continue; } 0xfb => { bb = 19; continue; } 0x100 => { bb = 20; continue; } 0x108 => { bb = 21; continue; } 0x122 => { bb = 22; continue; } 0x127 => { bb = 23; continue; } 0x134 => { bb = 24; continue; } 0x140 => { bb = 25; continue; } 0x14f => { bb = 26; continue; } 0x151 => { bb = 27; continue; } 0x165 => { bb = 28; continue; } 0x170 => { bb = 29; continue; } 0x19c => { bb = 30; continue; } 0x19e => { bb = 31; continue; } 0x1a8 => { bb = 32; continue; } 0x1b2 => { bb = 33; continue; } 0x1b7 => { bb = 34; continue; } 0x1c4 => { bb = 35; continue; } 0x1d0 => { bb = 36; continue; } 0x1e0 => { bb = 37; continue; } 0x1e2 => { bb = 38; continue; } 0x1f3 => { bb = 39; continue; } 0x200 => { bb = 40; continue; } 0x215 => { bb = 41; continue; } 0x217 => { bb = 42; continue; } 0x221 => { bb = 43; continue; } 0x22b => { bb = 44; continue; } 0x245 => { bb = 45; continue; } 0x252 => { bb = 46; continue; } 0x260 => { bb = 47; continue; } 0x273 => { bb = 48; continue; } 0x275 => { bb = 49; continue; } 0x286 => { bb = 50; continue; } 0x290 => { bb = 51; continue; } 0x2a8 => { bb = 52; continue; } 0x2aa => { bb = 53; continue; } 0x2b4 => { bb = 54; continue; } 0x2be => { bb = 55; continue; } 0x2c3 => { bb = 56; continue; } 0x2d0 => { bb = 57; continue; } 0x2e0 => { bb = 58; continue; } 0x2ee => { bb = 59; continue; } 0x2f0 => { bb = 60; continue; } 0x304 => { bb = 61; continue; } 0x310 => { bb = 62; continue; } 0x326 => { bb = 63; continue; } 0x328 => { bb = 64; continue; } 0x336 => { bb = 65; continue; } 0x340 => { bb = 66; continue; } 0x34a => { bb = 67; continue; } 0x34f => { bb = 68; continue; } 0x358 => { bb = 69; continue; } 0x370 => { bb = 70; continue; } 0x37e => { bb = 71; continue; } 0x380 => { bb = 72; continue; } 0x390 => { bb = 73; continue; } 0x3a0 => { bb = 74; continue; } 0x3b6 => { bb = 75; continue; } 0x3b8 => { bb = 76; continue; } 0x3bd => { bb = 77; continue; } 0x3cd => { bb = 78; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169da10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); r10 = rcx; rdi = r9; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; r11 = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x55; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1d; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 3; continue; }
            3 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rax; fl_r = (0x2u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 33; continue; } bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 33; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rsi = rax.wrapping_sub(0x1); r8 = r10; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rbp = r11.wrapping_sub(0x3); bb = 8; continue; }
            8 => { rax = r8; rax = ((rax).wrapping_sub(r10)) as u64; fl_l = rax; fl_r = rbp; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 32; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; rdi = ((rdi).wrapping_add(0x2u64)) as u64; r9 = ((rax as u32)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(4u32)) as u32 as u64; r9 = ((((r9 as u32)) & (0xf0u32))) as u32 as u64; r9 = ((((r9 as u32)) | ((rax as u32)))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xf0u32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(4u32)) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; r11 = (((r11 as u32)).wrapping_shl(4u32)) as u32 as u64; r11 = ((((r11 as u32)) | ((rax as u32)))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; r11 = (((r11 as u32)).wrapping_shl(4u32)) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_shl(4u32)) as u32 as u64; rbx = ((((rbx as u32)) | ((rax as u32)))) as u32 as u64; rbx = (((rbx as u32)).wrapping_shl(16u32)) as u32 as u64; fl_l = ((r14 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = 0xff000000; bb = 12; continue; }
            11 => { rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xf000u32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(16u32)) as u32 as u64; rax = ((((rax as u32)) & (0xf0000000u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(12u32)) as u32 as u64; fl_l = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rax = ((((rax as u32)) | ((rbx as u32)))) as u32 as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; rax = ((((rax as u32)) | ((r11 as u32)))) as u32 as u64; rax = ((((rax as u32)) | ((r9 as u32)))) as u32 as u64; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); r8 = ((r8).wrapping_add(0x4u64)) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 13; continue; }
            13 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 34; continue; }
            14 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 15; continue; }
            15 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rsi; fl_r = (0x2u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 33; continue; } bb = 16; continue; }
            16 => { fl_l = r11; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 33; continue; } bb = 17; continue; }
            17 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = r10; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 18; continue; }
            18 => { r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rbp = r11.wrapping_sub(0x3); bb = 19; continue; }
            19 => { rax = rcx; rax = ((rax).wrapping_sub(r10)) as u64; fl_l = rax; fl_r = rbp; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 32; continue; } bb = 20; continue; }
            20 => { rbx = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; rdi = ((rdi).wrapping_add(0x2u64)) as u64; r8 = ((rbx as u32)) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x7c00u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(5u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x380u32))) as u32 as u64; r9 = ((rbx as u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; r9 = ((((r9 as u32)) & (0x3e0u32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_shl(5u32)) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x380u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(7u32)) as u32 as u64; r9 = ((((r9 as u32)) | ((rax as u32)))) as u32 as u64; r11 = ((rbx as u32)) as u32 as u64; r11 = ((((r11 as u32)) & (0x1fu32))) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; r11 = (((r11 as u32)).wrapping_shl(5u32)) as u32 as u64; rax = ((((rax as u32)) & (0x1cu32))) as u32 as u64; r11 = ((((r11 as u32)) | ((rax as u32)))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((r9 as u32))) as u32 as u64; r11 = (((r11 as u32)).wrapping_shl(14u32)) as u32 as u64; fl_l = ((r14 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = 0xff000000; bb = 23; continue; }
            22 => { rax = ((((rbx as u16)) as i16 as i32) as u32) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; fl_l = ((((rax as u32)) & (0xff000000u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rax = ((((rax as u32)) | ((r11 as u32)))) as u32 as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; rax = ((((rax as u32)) | ((r9 as u32)))) as u32 as u64; rax = ((((rax as u32)) | ((r8 as u32)))) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 24; continue; }
            24 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 34; continue; }
            25 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rax; fl_r = (0x2u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 33; continue; } bb = 27; continue; }
            27 => { fl_l = r11; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 33; continue; } bb = 28; continue; }
            28 => { rsi = rax.wrapping_sub(0x1); rbx = rdi; r9 = r10; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 29; continue; }
            29 => { fl_l = ((r11).wrapping_add(0xfffffffffffffffdu64)) as u64; fl_r = fl_l; r11 = ((fl_l as u64)) as u64; bb = 30; continue; }
            30 => { rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; fl_l = rax; fl_r = r11; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(0x2u64)) as u64; rdx = ((rax as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x1cu32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(5u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(9u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x7e0u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(5u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xf800u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(5u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rdx = ((((rdx as u32)) & (0x600u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(3u32)) as u32 as u64; rax = rbx; rcx = ((((rcx as u32)) | ((rdx as u32)))) as u32 as u64; rax = ((rax).wrapping_sub(rdi)) as u64; rcx = (((rcx as u32)).wrapping_shr(5u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rcx as u32)))) as u32 as u64; r8 = ((((r8 as u32)) | (0xff000000u32))) as u32 as u64; crate::simd_ops::wu32(r9, ((r8 as u32)) as u32); r9 = ((r9).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 30; continue; } bb = 32; continue; }
            32 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 34; continue; }
            33 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169dcd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x141699fb0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; r11 = (r8) as u64; rax = (((rax as u32)).wrapping_add(0xffffffe9u32)) as u32 as u64; r10 = (rcx) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xa6u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 38; continue; } bb = 1; continue; }
            1 => { r8 = ((P0) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(0x169e000u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x169dff0u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            2 => { fl_l = (r9) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 38; continue; } bb = 3; continue; }
            3 => { fl_l = (rdx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 38; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 5; continue; }
            5 => { fl_l = (r10) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { r9 = ((rdx.wrapping_sub(0x3u64)) as u64) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 7; continue; }
            7 => { r8 = ((((r8 as u32)) & (0x1u32))) as u32 as u64; r9 = ((r9).wrapping_sub(1)) as u64; r9 = ((r9).wrapping_shr(2u32)) as u64; fl_l = ((r9).wrapping_add(1)) as u64 as u64; r9 = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(r10)) as u32 as u64; r10 = ((r10.wrapping_add(0x4u64)) as u64) as u64; rax = ((rdx as u32)) as u32 as u64; rcx = (0xc0000000u32) as u32 as u64; rax = ((((rax as u32)) & (0xc0000000u32))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(20u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rdx = ((((rdx as u32)) & (0xffc00u32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(20u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rcx = ((((rcx as u32)) | ((rdx as u32)))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_sub(0x4u64), ((rcx as u32)) as u32); fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            10 => { fl_l = (r9) as u64 as u64; fl_r = (rdx) as u64 as u64; r9 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rdx } else { r9 }) as u64; rbx = ((r9.wrapping_sub(0x3u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 11; continue; }
            11 => { r8 = ((((r8 as u32)) & (0x1u32))) as u32 as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru32(r9.wrapping_add(r11))) as u32 as u64; rcx = (0xc0000000u32) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xc0000000u32))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(20u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x3ffu32))) as u32 as u64; rdx = ((((rdx as u32)) & (0xffc00u32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(20u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rcx = ((((rcx as u32)) | ((rdx as u32)))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(r10), ((rcx as u32)) as u32); r9 = ((r9).wrapping_add(0x4u64)) as u64; fl_l = (r9) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            14 => { fl_l = (r9) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 38; continue; } bb = 15; continue; }
            15 => { fl_l = (rdx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 38; continue; } bb = 16; continue; }
            16 => { fl_l = (r10) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 17; continue; }
            17 => { r9 = ((rdx.wrapping_sub(0x3u64)) as u64) as u64; fl_l = (r9) as u64 as u64; fl_r = (r9) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 18; continue; }
            18 => { r11 = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; r11 = ((((r11 as u32)) & (0x1u32))) as u32 as u64; r9 = ((r9).wrapping_sub(1)) as u64; r9 = ((r9).wrapping_shr(2u32)) as u64; fl_l = ((r9).wrapping_add(1)) as u64 as u64; r9 = ((fl_l as u64)) as u64; bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru32(r10)) as u32 as u64; r10 = ((r10.wrapping_add(0x4u64)) as u64) as u64; rax = ((r8 as u32)) as u32 as u64; rdx = (0xff000000u32) as u32 as u64; rax = ((((rax as u32)) & (0xff000000u32))) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(16u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0xff00u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rdx = ((((rdx as u32)) | ((r8 as u32)))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_sub(0x4u64), ((rdx as u32)) as u32); fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            21 => { fl_l = (r9) as u64 as u64; fl_r = (rdx) as u64 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rdi) as u64); r9 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rdx } else { r9 }) as u64; rdi = ((r9.wrapping_sub(0x3u64)) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; rbx = ((((rbx as u32)) & (0x1u32))) as u32 as u64; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { r8 = (crate::simd_ops::ru32(r9.wrapping_add(r11))) as u32 as u64; rdx = (0xff000000u32) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xff000000u32))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((((rdx as u32)) | ((rcx as u32)))) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(16u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0xff00u32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rdx = ((((rdx as u32)) | ((r8 as u32)))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(r10), ((rdx as u32)) as u32); r9 = ((r9).wrapping_add(0x4u64)) as u64; fl_l = (r9) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 24; continue; }
            24 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 25; continue; }
            25 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            26 => { fl_l = (r9) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 38; continue; } bb = 27; continue; }
            27 => { fl_l = (rdx) as u64 as u64; fl_r = (0x4u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 38; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 29; continue; }
            29 => { fl_l = (r10) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 30; continue; }
            30 => { r8 = ((rdx.wrapping_sub(0x3u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 31; continue; }
            31 => { r8 = ((r8).wrapping_sub(1)) as u64; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(1)) as u64 as u64; r8 = ((fl_l as u64)) as u64; bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru32(r10)) as u32 as u64; r10 = ((r10.wrapping_add(0x4u64)) as u64) as u64; rdx = ((rcx as u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(8u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0xff00ffu32))) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_sub(0x4u64), ((rdx as u32)) as u32); fl_l = (r8) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 33; continue; }
            33 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            34 => { fl_l = (r9) as u64 as u64; fl_r = (rdx) as u64 as u64; r9 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rdx } else { r9 }) as u64; r8 = ((r9.wrapping_sub(0x3u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 35; continue; }
            35 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru32(r9.wrapping_add(r11))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(8u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0xff00ffu32))) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(r10), ((rdx as u32)) as u32); r9 = ((r9).wrapping_add(0x4u64)) as u64; fl_l = (r9) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 36; continue; } bb = 37; continue; }
            37 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            38 => { fl_l = (r10) as u64 as u64; fl_r = (r11) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 39; continue; }
            39 => { fl_l = (r9) as u64 as u64; fl_r = (rdx) as u64 as u64; r8 = (r11) as u64; rcx = (r10) as u64; r9 = (if (((fl_l as u64)) >= ((fl_r as u64))) { rdx } else { r9 }) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14169dcd0u64) { 0x1e => { bb = 1; continue; } 0x3d => { bb = 2; continue; } 0x47 => { bb = 3; continue; } 0x51 => { bb = 4; continue; } 0x60 => { bb = 5; continue; } 0x65 => { bb = 6; continue; } 0x72 => { bb = 7; continue; } 0x80 => { bb = 8; continue; } 0xc3 => { bb = 9; continue; } 0xc9 => { bb = 10; continue; } 0xdd => { bb = 11; continue; } 0xf0 => { bb = 12; continue; } 0x133 => { bb = 13; continue; } 0x139 => { bb = 14; continue; } 0x143 => { bb = 15; continue; } 0x14d => { bb = 16; continue; } 0x152 => { bb = 17; continue; } 0x15f => { bb = 18; continue; } 0x180 => { bb = 19; continue; } 0x1c2 => { bb = 20; continue; } 0x1c8 => { bb = 21; continue; } 0x1dd => { bb = 22; continue; } 0x1f0 => { bb = 23; continue; } 0x231 => { bb = 24; continue; } 0x236 => { bb = 25; continue; } 0x23c => { bb = 26; continue; } 0x246 => { bb = 27; continue; } 0x250 => { bb = 28; continue; } 0x25b => { bb = 29; continue; } 0x260 => { bb = 30; continue; } 0x269 => { bb = 31; continue; } 0x280 => { bb = 32; continue; } 0x2a8 => { bb = 33; continue; } 0x2ae => { bb = 34; continue; } 0x2c2 => { bb = 35; continue; } 0x2d0 => { bb = 36; continue; } 0x2f8 => { bb = 37; continue; } 0x2fe => { bb = 38; continue; } 0x307 => { bb = 39; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = 0x1; bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = r8; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = ((r9).wrapping_add(1)) as u64; rax = rdx; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rdx } else { rax }) as u64; rdx = rax; rax = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rcx } else { rax }) as u64; rcx = rax; rax = r8; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { r8 } else { rax }) as u64; r8 = rax; bb = 1; continue; }
            5 => { rax = r9; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169e0b0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r11 = (crate::simd_ops::ru64(r9)) as u64; r10 = r9; fl_l = r11; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { r9 = 0x1; bb = 2; continue; }
            2 => { fl_l = rdx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = r8; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r9 = ((r9).wrapping_add(1)) as u64; rax = rdx; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rdx } else { rax }) as u64; rdx = rax; rax = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rcx } else { rax }) as u64; rcx = rax; rax = r8; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { r8 } else { rax }) as u64; r8 = rax; bb = 2; continue; }
            6 => { fl_l = r11; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            8 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r10, rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            10 => { r9 = 0x1; crate::simd_ops::wu64(r10, r9); bb = 11; continue; }
            11 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e1a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (crate::simd_ops::ru64(r8)) as u64; r9 = r8; r8 = 0x1; fl_l = r10; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = rdx; r8 = ((r8).wrapping_add(1)) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rdx } else { rax }) as u64; rdx = rax; rax = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rcx } else { rax }) as u64; rcx = rax; bb = 1; continue; }
            4 => { fl_l = r10; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            6 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = rdx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = rcx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = rdx; r8 = ((r8).wrapping_add(1)) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rdx } else { rax }) as u64; rdx = rax; rax = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rax = (if (((fl_l as u64)) <= ((fl_r as u64))) { rcx } else { rax }) as u64; rcx = rax; bb = 7; continue; }
            10 => { crate::simd_ops::wu64(r9, r8); bb = 11; continue; }
            11 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa448; const P1: u64 = 0x18; const P2: u64 = 0x28; const P3: u64 = 0x30; const P4: u64 = 0x20; const P5: u64 = 0x38; const P6: u64 = 0x60; const P7: u64 = 0x68; const P8: u64 = 0x70; const P9: u64 = 0x40; const P10: u64 = 0x48; const P11: u64 = 0x50; const P12: u64 = 0x141833c33;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = P0; crate::simd_ops::wu64(rcx, rax); rbx = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx); crate::simd_ops::wu64(rcx.wrapping_add(P2), rdx); crate::simd_ops::wu64(rcx.wrapping_add(P3), rdx); crate::simd_ops::wu32(rcx.wrapping_add(P4), (0x1u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rdx as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P6), rdx); crate::simd_ops::wu64(rcx.wrapping_add(P7), rdx); crate::simd_ops::wu64(rcx.wrapping_add(P8), rdx); crate::simd_ops::wu64(rcx.wrapping_add(P9), (0x10u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P10), (0x2000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P11), (0x2000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), (0xffffffffffffffffu64) as u64); fl_l = ((rcx).wrapping_add(0x88u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = rcx.wrapping_sub(0x8); rax = (if (((fl_l as u64)) == 0) { rdx } else { rax }) as u64; rdx = 0x102; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rcx = rbx.wrapping_add(0x98); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; crate::simd_ops::wu16(rbx.wrapping_add(0xe8u64), (0x101u16) as u16); crate::simd_ops::wu128(rbx.wrapping_add(0x38u64), (xmm0) as u128); rax = rbx; crate::simd_ops::wu128(rbx.wrapping_add(0x48u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x58u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x68u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x78u64), (xmm0) as u128); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa448; const P1: u64 = 0x141833c27; const P2: u64 = 0x141834d30;
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
    rcx = ((rcx).wrapping_add(0x98u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xf0; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x141833c2d; const P2: u64 = 0xf8; const P3: u64 = 0x141833c49; const P4: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (((rax as u8)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (((rdi as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x14169e392, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e3b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143932190; const P2: u64 = 0x78; const P3: u64 = 0xc; const P4: u64 = 0x14169ea80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r8 = (crate::simd_ops::ru64(rdx)) as u64; rdi = rcx; rbx = r8; rcx = 0; fl_l = 0; fl_r = 0; rbx = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_l = ((((r8 as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; bb = 5; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; rax = (0x7fffffffffffffffu64) as u64; r8 = r9; rcx = (0x8000000000000000u64) as u64; r8 = (((r8) & (rax))) as u64; r9 = (((r9) & (rcx))) as u64; rax = ((rbx.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8); rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { crate::simd_ops::wu64(rdi, rbx); rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x14169eb00; const P2: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = r9; rax = r8; r15 = rcx; rdi = (0xffffffffffffffffu64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rsi = rdi; bb = 2; continue; }
            2 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 3; continue; }
            3 => { bb = 5; continue; }
            4 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rbp = rdi; bb = 7; continue; }
            7 => { rbp = ((rbp).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rbp))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 8; continue; }
            8 => { bb = 10; continue; }
            9 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 13; continue; }
            13 => { bb = 15; continue; }
            14 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); r8 = ((rdi.wrapping_add(rbp)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = ((r8).wrapping_add(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(0xcu64)) as u64; r8 = rdi; rdx = r14; rbx = rax; rcx = ((rax.wrapping_add(rbp)) as u64) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = r15; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(r15, rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x10; const P2: u64 = 0x48; const P3: u64 = 0xc; const P4: u64 = 0x8; const P5: u64 = 0x14169ed00; const P6: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; r14 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rbx = (0xffffffffffffffffu64) as u64; bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P0)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbx.wrapping_add(P1)) as u64) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = rax; crate::simd_ops::wu8(rbx.wrapping_add(rax).wrapping_add(P3), (0x0u8) as u8); rax = 0x1; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rsi.wrapping_add(P4), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; crate::simd_ops::wu64(rsi, rbx); bb = 7; continue; }
            5 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; bb = 7; continue; }
            7 => { rcx = ((rsi.wrapping_add(P3)) as u64) as u64; r8 = rbx; rdx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = r14; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(r14, rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e5d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169ed00; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx, rax);
    rdx = 0x1;
    let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rax.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rdx = (xa_old) as u32 as u64;
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x14169ea80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r8 = (crate::simd_ops::ru32(rcx)) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; r15 = rdx; rbx = rcx; rbp = ((rdi as u32)) as u32 as u64; fl_l = ((((r8 as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; bb = 7; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = rbx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = rax; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P0)) as u64; bb = 7; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rbp = rcx; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(r15)) as u64; r14 = (crate::simd_ops::ru64(rbx)) as u64; rsi = rdx; rsi = (((rsi) & (0xfffffffffffffffcu64))) as u64; r14 = (((r14) & (0xfffffffffffffffcu64))) as u64; fl_l = ((((rdx as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; bb = 13; continue; }
            11 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r15; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; bb = 13; continue; }
            12 => { rdi = rcx; bb = 13; continue; }
            13 => { fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rax = (((rax) | (rsi))) as u64; crate::simd_ops::wu64(rbx, rax); rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; bb = 16; continue; }
            15 => { r9 = (crate::simd_ops::ru64(rsi)) as u64; rax = (0x7fffffffffffffffu64) as u64; r8 = r9; rcx = (0x8000000000000000u64) as u64; r8 = (((r8) & (rax))) as u64; r9 = (((r9) & (rcx))) as u64; rax = rsi.wrapping_add(0xc); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8); rdx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbx, rcx); bb = 16; continue; }
            16 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(r14.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = r14; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x14169ed00; const P2: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = 0; fl_l = 0; fl_r = 0; rsi = rdx; r14 = rcx; r15 = r9.wrapping_sub(0x1); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = r15; bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rbx))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 3; continue; }
            3 => { bb = 5; continue; }
            4 => { rbx = r9; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rbp = rcx; rbp = (((rbp) & (0xfffffffffffffffcu64))) as u64; fl_l = ((((rcx as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; bb = 11; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = r14; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = rax; bb = 11; continue; }
            10 => { r9 = (crate::simd_ops::ru64(P0)) as u64; bb = 11; continue; }
            11 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; bb = 14; continue; }
            13 => { rax = (crate::simd_ops::ru64(r9)) as u64; rdx = rbx.wrapping_add(0x10); r8 = 0; fl_l = 0; fl_r = 0; rcx = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; crate::simd_ops::wu8(rbx.wrapping_add(rax).wrapping_add(0xcu64), (0x0u8) as u8); rax = 0x1; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; crate::simd_ops::wu64(rdi, rbx); bb = 14; continue; }
            14 => { rcx = rdi.wrapping_add(0xc); r8 = rbx; rdx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rax = ((((rax as u32)) & (0x3u32))) as u32 as u64; rax = (((rax) | (rdi))) as u64; crate::simd_ops::wu64(r14, rax); let xa_src: u32 = (r15 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbp.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r15 = (xa_old) as u32 as u64; rax = ((r15.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = rbp; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169e3b0; const P1: u64 = 0x1426cdcb0; const P2: u64 = 0x143932190; const P3: u64 = 0x14169eb00;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = r8; rdi = rdx; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rbp as u32)) as u32); rdx = rcx; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x1u32) as u32); rsi = P1; fl_l = rbx; fl_r = rbx; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { r15 = (0xffffffffffffffffu64) as u64; rbx = r15; bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru64(rdi)) as u64; r14 = (((r14) & (0xfffffffffffffffcu64))) as u64; r12 = (crate::simd_ops::ru64(r14)) as u64; rax = (0x7fffffffffffffffu64) as u64; r12 = (((r12) & (rax))) as u64; rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((((rcx as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; bb = 9; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = rax; bb = 9; continue; }
            8 => { rbp = (crate::simd_ops::ru64(P2)) as u64; bb = 9; continue; }
            9 => { rax = r14.wrapping_add(0xc); r8 = ((r12.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0; fl_l = 0; fl_r = 0; rdx = rbp; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3u32))) as u32 as u64; rcx = (((rcx) | (rax))) as u64; crate::simd_ops::wu64(rdi, rcx); let xa_src: u32 = (r15 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(r14.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r15 = (xa_old) as u32 as u64; rax = ((r15.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rax = rdi; r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169e980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x14169eb00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; r15 = (crate::simd_ops::ru64(rdx)) as u64; rdi = r8; rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; r15 = (((r15) & (0xfffffffffffffffcu64))) as u64; rcx = (0x7fffffffffffffffu64) as u64; rax = (0x8000000000000000u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(r15)) as u64; rbp = (crate::simd_ops::ru64(rdi)) as u64; r14 = rsi; rsi = (((rsi) & (crate::simd_ops::ru64(rdi)))) as u64; rbp = (((rbp) & (rcx))) as u64; rsi = (((rsi) & (rax))) as u64; r14 = (((r14) & (rcx))) as u64; fl_l = ((((r8 as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 6; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = rbx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rax; bb = 6; continue; }
            5 => { rdx = (crate::simd_ops::ru64(P0)) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rax = r15.wrapping_add(0xc); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rcx = rdi.wrapping_add(0xc); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); r8 = ((r14.wrapping_add(rbp)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r9 = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3u32))) as u32 as u64; rcx = (((rcx) | (rax))) as u64; rax = 0xffffffff; crate::simd_ops::wu64(rbx, rcx); let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = rdi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169ea80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169ed00; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x48; const P4: u64 = 0xc; const P5: u64 = 0x14183acbc;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r9; rdi = r8; r10 = rdx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = r10; rdx = ((r8.wrapping_add(P2)) as u64) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; crate::simd_ops::wu8(rax.wrapping_add(rdi).wrapping_add(P4), (0x0u8) as u8); rax = 0x1; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rbx.wrapping_add(P1), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; fl_l = (((rdi) | (rsi))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbx, rdi); bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169eb00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169ed00; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x48; const P4: u64 = 0xc; const P5: u64 = 0x14183acbc;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r9; rbx = r8; r10 = rdx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = r10; rdx = ((r8.wrapping_add(P2)) as u64) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; crate::simd_ops::wu8(rax.wrapping_add(rbx).wrapping_add(P4), (0x0u8) as u8); rax = 0x1; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(rdi.wrapping_add(P1), ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; fl_l = (((rbx) | (rsi))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rdi, rbx); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169eba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x14169eb00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = r8; rdi = rdx; r14 = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); rbp = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r15); fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = rbp; bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = (0x7fffffffffffffffu64) as u64; rsi = rcx; rdx = 0; fl_l = 0; fl_r = 0; rsi = (((rsi) & (0xfffffffffffffffcu64))) as u64; r15 = (crate::simd_ops::ru64(rsi)) as u64; r15 = (((r15) & (rax))) as u64; fl_l = ((((rcx as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; bb = 11; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = r14; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rax; bb = 11; continue; }
            10 => { rdx = (crate::simd_ops::ru64(P0)) as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rax = rsi.wrapping_add(0xc); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); r8 = ((r15.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); r9 = 0; fl_l = 0; fl_r = 0; rcx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(r14)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3u32))) as u32 as u64; rcx = (((rcx) | (rax))) as u64; crate::simd_ops::wu64(r14, rcx); let xa_src: u32 = (rbp as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbp = (xa_old) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rbp.wrapping_sub(0x1u64)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 15; continue; }
            15 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169eca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x14171ee90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = (0x7fffffffffffffffu64) as u64;
    rbx = (((rbx) & (0xfffffffffffffffcu64))) as u64;
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    rdi = rcx;
    rdi = (((rdi) & (rax))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !((((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0)) { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rbx.wrapping_add(P0)) as u64) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rdi;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rcx = (0x8000000000000000u64) as u64; fl_l = (crate::simd_ops::lk_or_u64(rbx, (rcx) as u64)) as u64; fl_r = fl_l; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169ed00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397d1e0; const P1: u64 = 0x143932198; const P2: u64 = 0x1438214f8; const P3: u64 = 0x141834a78; const P4: u64 = 0x143821500; const P5: u64 = 0x143821504; const P6: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rcx = ((crate::exe::tls_pointer() as u64)) as u64; rdx = 0x74; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = P2; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            2 => { rcx = P1; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = 0x1; rcx = P1; let tmp_xchg: u32 = crate::simd_ops::atomic_xchg_u32(P4, ((rax as u32)) as u32); rax = (tmp_xchg) as u32 as u64; crate::simd_ops::wu8(P5, (0x0u8) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = P2; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169eda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = 0x1;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169edb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0xffffffff;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { return rax; }
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169edd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x1;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32);
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169edf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x30; const P4: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), (0xffffffffffffffffu64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169ee10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143932190; const P2: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx).wrapping_add(rdx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_sub(0x30u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169eef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa580; const P1: u64 = 0x14171f650; const P2: u64 = 0x30; const P3: u64 = 0x14169edd0; const P4: u64 = 0x18; const P5: u64 = 0x143932190; const P6: u64 = 0x60; const P7: u64 = 0x14169ed80; const P8: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rdx = 0x40; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169ef90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa560; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = 0x10;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169efe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa560; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = 0x8;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169f030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x1426aa580; const P2: u64 = 0x14169edb0; const P3: u64 = 0x1416a3130; const P4: u64 = 0x1433e14a8; const P5: u64 = 0x14171f7b0; const P6: u64 = 0x1416a30e0; const P7: u64 = 0x14171f4e0; const P8: u64 = 0x14169edd0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = r8; r14 = rdx; rsi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_add(0x40u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); rax = P1; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rbp); crate::simd_ops::wu8(rbx.wrapping_add(0x38u64), (0x1u8) as u8); bb = 4; continue; }
            3 => { rbx = rbp; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rbx); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x30u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P4; crate::simd_ops::wu64(rbx, rax); bb = 16; continue; }
            9 => { rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rdx = 0x12; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu8(r14, ((rbp as u8)) as u8); bb = 21; continue; }
            15 => { rbx = rbp; bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rbp as u32)) as u32); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x24u32) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbp); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rdx = rbx; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (((rax as u32)).wrapping_add(0x24u32)) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = rbp; bb = 19; continue; }
            18 => { rax = ((rax).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x4u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0xcu64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x14u64), rbp); fl_l = (crate::simd_ops::lk_add_u32(rcx, (0x24u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbp); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(r14, (0x1u8) as u8); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rax = r14; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169f1f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169eda0; const P1: u64 = 0x140000000; const P2: u64 = 0x1416bcbb0; const P3: u64 = 0x14169edb0; const P4: u64 = 0x14169fc30; const P5: u64 = 0x1416a0000; const P6: u64 = 0x1416bd860;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); rax = (((crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 11; continue; } bb = 3; continue; }
            3 => { rdx = ((P1) as u64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x169f31cu64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            4 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rdi, (0x0u8) as u8); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rcx = ((rcx).wrapping_add(0xd8u64)) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), (0x1u32) as u32); bb = 8; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rcx = ((rcx).wrapping_add(0x120u64)) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), (0x3u32) as u32); bb = 8; continue; }
            8 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rdi, (0x1u8) as u8); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rdi, (0x0u8) as u8); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            12 => { crate::simd_ops::wu8(rdi, (0x0u8) as u8); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14169f1f0u64) { 0x1b => { bb = 1; continue; } 0x26 => { bb = 2; continue; } 0x3d => { bb = 3; continue; } 0x50 => { bb = 4; continue; } 0x60 => { bb = 5; continue; } 0x7a => { bb = 6; continue; } 0x98 => { bb = 7; continue; } 0xc9 => { bb = 8; continue; } 0xe6 => { bb = 9; continue; } 0xf7 => { bb = 10; continue; } 0x111 => { bb = 11; continue; } 0x11a => { bb = 12; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169f340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x9c; const P2: u64 = 0xa0; const P3: u64 = 0xa4; const P4: u64 = 0xac; const P5: u64 = 0x70; const P6: u64 = 0x68; const P7: u64 = 0x14; const P8: u64 = 0x1c; const P9: u64 = 0x18; const P10: u64 = 0x20; const P11: u64 = 0xa8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x80000000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P3), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P4), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp, rbx); r11 = 0; fl_l = 0; fl_r = 0; rbx = ((rax as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; r11 = r11.wrapping_add(0x8); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r9 = (crate::simd_ops::ru64(r11.wrapping_add(rax).wrapping_sub(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(P7))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r9.wrapping_add(P7)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; r10 = (crate::simd_ops::ru32(r9.wrapping_add(P8))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r10 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rax as u32) } else { (r10 as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((r10 as u32)) as u32); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(P9))) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r9.wrapping_add(P9)) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; rdx = (crate::simd_ops::ru32(r9.wrapping_add(P10))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rdx as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(P0)))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rdx as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P11), ((r10 as u32)) as u32); fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp)) as u64; bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169f4d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x48; const P2: u64 = 0x14171f7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; rbx = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = 0x2; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x0u32) as u32); bb = 3; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169f520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x14171f7b0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    rdx = 0x12;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169f560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14171f7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rbx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rbx = ((fl_l as u32)) as u32 as u64; bb = 1; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rbx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; bb = 8; continue; }
            8 => { rdx = 0x12; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; rdx = 0x12; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(0x1cu64)) as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169f640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x14171f7b0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    rdx = 0x2;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169f670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x14171f7b0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rdx;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    rdx = 0x1;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169f6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169eda0; const P1: u64 = 0x140000000; const P2: u64 = 0x1416a0000; const P3: u64 = 0x14169edb0;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rdi = rdx; rbx = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (((crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { r8 = P1; rcx = (crate::simd_ops::ru32(r8.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x169f784u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r8)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            7 => { rcx = rdx.wrapping_add(0xd8); rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), (0x0u32) as u32); crate::simd_ops::wu8(rdi, (0x1u8) as u8); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { crate::simd_ops::wu8(rdi, (0x0u8) as u8); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { crate::simd_ops::wu8(rdi, (0x0u8) as u8); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14169f6b0u64) { 0x1d => { bb = 1; continue; } 0x22 => { bb = 2; continue; } 0x2c => { bb = 3; continue; } 0x34 => { bb = 4; continue; } 0x3d => { bb = 5; continue; } 0x46 => { bb = 6; continue; } 0x5a => { bb = 7; continue; } 0x6b => { bb = 8; continue; } 0x95 => { bb = 9; continue; } 0xb4 => { bb = 10; continue; } _ => { return crate::simd_ops::tail_stk(j_tgt, rax, rcx, rdx, r8, r9, rsp); } } }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169f7a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14171f7b0; const P1: u64 = 0x14171fdb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; rbx = rdx; rdx = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; bb = 3; continue; }
            2 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rdx = 0x12; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r15 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((r15 as u32)) as u32); bb = 6; continue; }
            5 => { crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), (0x0u32) as u32); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rdx = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r13 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; bb = 9; continue; }
            8 => { r13 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rdx = 0x12; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r12 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; bb = 12; continue; }
            11 => { r12 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rdx as u32)) as u32); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 13; continue; }
            13 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r8 = ((r8).wrapping_add(0x24u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r8); fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r15))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = r8; rdx = ((rdx).wrapping_add(0x24u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 17; continue; }
            17 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; r15 = ((r15).wrapping_add(0x8u64)) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; bb = 19; continue; }
            19 => { r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(rax))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; r15 = ((r15).wrapping_add(0x8u64)) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 21; continue; } bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 14; continue; } bb = 25; continue; }
            25 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rdx as u32)) as u32); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 38; continue; } bb = 26; continue; }
            26 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rcx); bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax))) as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r14))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = r15; rdx = ((rdx).wrapping_add(0x24u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 30; continue; }
            30 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; r14 = ((r14).wrapping_add(0x8u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 29; continue; } bb = 31; continue; }
            31 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; bb = 32; continue; }
            32 => { rsi = 0; fl_l = 0; fl_r = 0; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 33; continue; }
            33 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(rax))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = r15; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 35; continue; }
            35 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; r14 = ((r14).wrapping_add(0x8u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 34; continue; } bb = 36; continue; }
            36 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rcx); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 27; continue; } bb = 38; continue; }
            38 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 39; continue; }
            39 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            40 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 39; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169fa50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x40; const P2: u64 = 0x48; const P3: u64 = 0x8; const P4: u64 = 0x1426aa580; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x30; const P9: u64 = 0x38; const P10: u64 = 0x14171f800; const P11: u64 = 0x1416a03d0; const P12: u64 = 0x14169edb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = r8; rdi = rdx; rsi = rcx; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rax.wrapping_add(P3), (0x1u32) as u32); rax = P4; crate::simd_ops::wu64(rbx, rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P5), rax); crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); crate::simd_ops::wu64(rbx.wrapping_add(P8), rax); crate::simd_ops::wu8(rbx.wrapping_add(P9), (0x1u8) as u8); bb = 3; continue; }
            2 => { rbx = rax; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r9 = 0x1; r8 = rbp; rdx = rsp.wrapping_add(0x58); rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rdi, (0x0u8) as u8); bb = 6; continue; }
            5 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi, (0x1u8) as u8); bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169fb10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x14169e530; const P2: u64 = 0x14171f090; const P3: u64 = 0x14169edb0; const P4: u64 = 0x14169eda0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = ((r9 as u32)) as u32 as u64; rbp = r8; rdi = rdx; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x18u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r14 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = rbp; rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = 0x2; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((rsi.wrapping_add(0x1fu64)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { r8 = 0; fl_l = 0; fl_r = 0; rax = 0x2e; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; r8 = (if (((fl_l as u32)) == ((fl_r as u32))) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; bb = 6; continue; }
            6 => { r9 = 0x1b6; rdx = rsp.wrapping_add(0x50); rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 8; continue; }
            7 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi, (0x0u64) as u64); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            13 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbx); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14169fcb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa5d8; const P1: u64 = 0x1416a0280; const P2: u64 = 0x1426aa5e8; const P3: u64 = 0x143932190; const P4: u64 = 0x1426aa610; const P5: u64 = 0x14171faf0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x37); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; r15 = r8; rsi = rdx; r14 = rcx; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), r12); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), r12); crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), r12); rax = P0; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r12); r8 = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x1); rcx = rbp.wrapping_sub(0x1); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), (0x1u64) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = P2; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); rax = r14.wrapping_add(0x8); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r12); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; rbx = rax.wrapping_add(0x1); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; rax = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rdx); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), r12); bb = 13; continue; }
            7 => { rdi = rbx.wrapping_add(0x3); rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rdx; rdx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); rdx = rax; crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rdi); bb = 13; continue; }
            9 => { crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), (0x2u32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rbp.wrapping_add(0x67); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); rdx = rax; crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rdi); bb = 13; continue; }
            10 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; rdx = rbp.wrapping_sub(0x1); rcx = rbp.wrapping_sub(0x1); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rbx); rax = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_sub(0x18u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), (0x1u32) as u32); rax = P4; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rax); rax = r14.wrapping_add(0x10); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; rbx = rax.wrapping_add(0x1); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; rax = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 25; continue; } bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 16; continue; }
            16 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r12; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rdx); bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), r12); bb = 26; continue; }
            20 => { rdi = rbx.wrapping_add(0x3); rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rdx; rdx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); rdx = rax; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rdi); bb = 26; continue; }
            22 => { crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), (0x2u32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rbp.wrapping_add(0x67); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); rdx = rax; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rdi); bb = 26; continue; }
            23 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; r8 = ((r8).wrapping_add(rbx)) as u64; rdx = rbp.wrapping_sub(0x19); rcx = rbp.wrapping_sub(0x19); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rbx); rax = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_sub(0x18u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm1 as u64)) as u64); r8 = r15; rdx = rsi; rcx = rbp.wrapping_sub(0x19); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x1fu64))) as u64; rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; rbx = ((rbx).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 27; continue; }
            27 => { fl_l = ((rbx).wrapping_add(rdx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rbx = ((rbx).wrapping_sub(0x30u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 31; continue; }
            31 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; bb = 32; continue; }
            32 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 34; continue; }
            34 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            36 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            38 => { rax = rsi; r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a00c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169eda0; const P1: u64 = 0x140000000; const P2: u64 = 0x1416a0000; const P3: u64 = 0x14169edb0; const P4: u64 = 0x14169fc30; const P5: u64 = 0x1416bd860;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = (((crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rdx = P1; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x16a01b8u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rcx = ((rcx).wrapping_add(0xd8u64)) as u64; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), (0x0u32) as u32); bb = 7; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi, (0x0u8) as u8); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rcx = ((rcx).wrapping_add(0x120u64)) as u64; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), (0x4u32) as u32); bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rdi, (0x1u8) as u8); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rdi, (0x0u8) as u8); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1416a00c0u64) { 0x1b => { bb = 1; continue; } 0x26 => { bb = 2; continue; } 0x3d => { bb = 3; continue; } 0x50 => { bb = 4; continue; } 0x6e => { bb = 5; continue; } 0x88 => { bb = 6; continue; } 0xb9 => { bb = 7; continue; } 0xdc => { bb = 8; continue; } 0xe5 => { bb = 9; continue; } _ => { return crate::simd_ops::tail_stk(j_tgt, rax, rcx, rdx, r8, r9, rsp); } } }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a01d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x3; const P4: u64 = 0x50; const P5: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdi = ((r8.wrapping_add(P3)) as u64) as u64; rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; r9 = ((rdi.wrapping_mul(8)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x81u32) as u32); r8 = rsp.wrapping_add(0x48); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a0280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x3; const P4: u64 = 0x50; const P5: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdi = ((r8.wrapping_add(P3)) as u64) as u64; rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; r9 = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_shl(3u32)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); r8 = rsp.wrapping_add(0x48); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a03d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169eda0; const P1: u64 = 0x14169edb0; const P2: u64 = 0x14171f7b0; const P3: u64 = 0x14169fcb0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 5; continue; }
            5 => { rdx = 0x1; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rbx.wrapping_add(0x98u64), (0x7fffffffu32) as u32); rdx = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x9cu64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xa0u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xa4u64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), (0xffffffffu32) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); r11 = ((rdi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = ((r11 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9)) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r9) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x98u64), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; r10 = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r10 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rcx as u32) } else { (r10 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xa0u64), ((r10 as u32)) as u32); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x9cu64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r9.wrapping_add(0x4u64)) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x9cu64), ((r8 as u32)) as u32); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa4u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(r9.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; r11 = (((r11 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xa4u64), ((rdx as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub(crate::simd_ops::ru32(rbx.wrapping_add(0x98u64)))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xa8u64), ((r10 as u32)) as u32); fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rdx = 0x12; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rbx.wrapping_add(0x98u64), (0x7fffffffu32) as u32); rdx = 0x12; crate::simd_ops::wu32(rbx.wrapping_add(0x9cu64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xa0u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xa4u64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), (0xffffffffu32) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { r11 = ((rcx as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdi = rdi.wrapping_add(0x8); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rdi).wrapping_sub(0x8u64))) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x28u64))) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r9.wrapping_add(0x28u64)) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x98u64), ((rdx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; r10 = (crate::simd_ops::ru32(r9.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r10 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rcx as u32) } else { (r10 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xa0u64), ((r10 as u32)) as u32); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x9cu64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x2cu64))) as u32 as u64; r8 = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { crate::simd_ops::ru32(r9.wrapping_add(0x2cu64)) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x9cu64), ((r8 as u32)) as u32); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa4u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(r9.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xa4u64), ((rdx as u32)) as u32); rdx = (((rdx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub(crate::simd_ops::ru32(rbx.wrapping_add(0x98u64)))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xa8u64), ((r10 as u32)) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rdx = 0xe; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x30); r8 = (crate::simd_ops::ru64(rcx)) as u64; rcx = rbx.wrapping_add(0xb0); r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a0670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411fb140;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rsi = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; r9 = rdx.wrapping_add(0xc); rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; rax = (((rax as u32)).swap_bytes()) as u32 as u64; rcx = (((rcx as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((rax as u32)) as u32); r8 = ((rbp as u32)) as u32 as u64; rax = (rax as u32 as i32 as i64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = ((r8 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; r9 = rdx.wrapping_add(0x14); rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; r8 = ((rbp as u32)) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; rcx = (((rcx as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (rax as u32 as i32 as i64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((r8 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as u32 as u64; rbx = rdx.wrapping_add(0x18); rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rsi = rbx.wrapping_add(0x4); rax = (((rax as u32)).swap_bytes()) as u32 as u64; rcx = (((rcx as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rax as u32)) as u32); rdi = ((rbp as u32)) as u32 as u64; rax = (rax as u32 as i32 as i64) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rax = ((rdi as u32)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = (((rax) as i64).wrapping_mul((0x1cu64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(0x14u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 9; continue; }
            9 => { bb = 19; continue; }
            10 => { r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rcx = rdx.wrapping_add(0x8); r9 = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); r9 = ((r9).wrapping_add(0xcu64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0x8u64))) as u32 as u64; rcx = rcx.wrapping_add(0x10); rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0x14u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0x10u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0xcu64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0xcu64), ((rax as u32)) as u32); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rcx = rdx.wrapping_add(0x10); r9 = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); r9 = ((r9).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = ((rcx).wrapping_add(r9)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0x4u64))) as u32 as u64; rcx = rcx.wrapping_add(0x10); rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0x10u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0xcu64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_sub(0x8u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_sub(0x8u64), ((rax as u32)) as u32); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; rbx = (((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = ((rbx).wrapping_add(rdx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rcx as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rbx.wrapping_sub(0x4u64))) as u32 as u64; rdx = rbx.wrapping_add(0x10); rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_sub(0x4u64), ((rax as u32)) as u32); rcx = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rbx.wrapping_add(0x1c); fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a08e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = ((r8.wrapping_mul(4)) as u32) as u32 as u64; rdi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(rcx, ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdx, (0x0u64) as u64); rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rbx = ((rbx).wrapping_add(rax)) as u64; crate::simd_ops::wu32(rdx, ((r8 as u32)) as u32); rcx = ((rbx as u32)) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(0x4u32)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rcx as u32)) as u32); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_add_u32(rdi, ((rsi as u32)) as u32); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a0960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1433e1380; const P2: u64 = 0x20; const P3: u64 = 0x14169edf0; const P4: u64 = 0x10; const P5: u64 = 0x1426aa6a8; const P6: u64 = 0x38; const P7: u64 = 0x3c; const P8: u64 = 0x40; const P9: u64 = 0x44; const P10: u64 = 0x48; const P11: u64 = 0x4c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x1u32) as u32); rax = P1; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P2), (0xffffffffffffffffu64) as u64); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P4), rcx); rax = P5; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(P6), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P7), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P9), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P10), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P11), (0xff7fffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rcx); rax = rbx.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rcx); crate::simd_ops::wu32(rax.wrapping_add(0x18u64), (0xa5ff00u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), (0x2u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), rcx); crate::simd_ops::wu8(rbx.wrapping_add(0xa8u64), ((rcx as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xb0u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xb4u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xb8u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xbcu64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xc0u64), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xc4u64), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xc8u64), (0xff7fffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xccu64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xd4u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xdcu64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xe4u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xecu64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0xf4u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xfcu64), rcx); crate::simd_ops::wu32(rbx.wrapping_add(0x104u64), ((rcx as u32)) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0x108u64), ((rcx as u16)) as u16); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a0b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa688; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = 0x38;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a0b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa6a8; const P1: u64 = 0x1416af290; const P2: u64 = 0x88; const P3: u64 = 0x14169edd0; const P4: u64 = 0x68; const P5: u64 = 0x143932190; const P6: u64 = 0x60; const P7: u64 = 0x50; const P8: u64 = 0x1433e1380; const P9: u64 = 0x14169ed80; const P10: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rdi = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rcx = ((rcx).wrapping_add(0x88u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = P8; crate::simd_ops::wu64(rdi, rax); rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rdx = 0x110; rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a0c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169eda0; const P1: u64 = 0x14169edb0; const P2: u64 = 0x1416a1920; const P3: u64 = 0x1416a18a0; const P4: u64 = 0x14171ff70; const P5: u64 = 0x1416a1c10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0xfcu64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x100u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x104u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rbp.wrapping_add(0x100u64), ((rcx as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x108u64), ((rcx as u8)) as u8); bb = 10; continue; }
            7 => { crate::simd_ops::wu32(rbp.wrapping_add(0x100u64), (0x3u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x104u64), (0x3u32) as u32); rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0xf8u64)) as u64; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0xe0u64)) as u64; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbp.wrapping_add(0x108u64), (0x1u8) as u8); bb = 10; continue; }
            8 => { crate::simd_ops::wu32(rbp.wrapping_add(0x100u64), (0x3u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x108u64), (0x1u8) as u8); bb = 10; continue; }
            9 => { rdx = rbp.wrapping_add(0x38); rcx = rbp.wrapping_add(0x88); r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x98u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rsi = ((rsi).wrapping_add(0xc8u64)) as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = rsi; rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), rbp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rdi as u32)) as u32); rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbp.wrapping_add(0xfcu64), (0x1u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x104u64), (0x1u32) as u32); bb = 10; continue; }
            10 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a0df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x109; const P1: u64 = 0x108;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8); bb = 6; continue; }
            6 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a0e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x9c; const P1: u64 = 0xfffffffffffffffe; const P2: u64 = 0xb4; const P3: u64 = 0x141720280; const P4: u64 = 0xa8; const P5: u64 = 0x141720300;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; xmm0 = (xmm2) as u128; rdx = rcx; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = ((rcx.wrapping_add(P1)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rdx.wrapping_add(P2)) as u64) as u64; xmm2 = (xmm3) as u128; xmm1 = (xmm0) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; crate::simd_ops::wu8(rbx, (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { crate::simd_ops::wu8(rbx, (0x0u8) as u8); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = ((rdx.wrapping_add(P4)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; crate::simd_ops::wu8(rbx, (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a0ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1417209f0;
    let (mut rax, mut r10, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    r10 = r9;
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    r11 = r8;
    rbx = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    r8 = r10;
    rdx = r11;
    rcx = rbx;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a0ed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169eda0; const P1: u64 = 0x14169edb0; const P2: u64 = 0x1416a0ed0; const P3: u64 = 0x1416a1920; const P4: u64 = 0x1416a18a0; const P5: u64 = 0x1411fa4d0; const P6: u64 = 0x1416a1d60; const P7: u64 = 0x141720180; const P8: u64 = 0x14169edd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; r14 = rdx; rbx = rcx; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xfcu64))) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x104u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rdx, (0x0u8) as u8); rax = rdx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xfcu64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = rdi; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x100u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = rdi; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x104u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rbx.wrapping_add(0x100u64), (0x2u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x108u64), (0x1u8) as u8); bb = 15; continue; }
            13 => { crate::simd_ops::wu32(rbx.wrapping_add(0x100u64), (0x2u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x104u64), (0x4u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0xe0u64)) as u64; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0xf8u64)) as u64; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(0x108u64), (0x1u8) as u8); bb = 15; continue; }
            14 => { crate::simd_ops::wu32(rbx.wrapping_add(0xfcu64), (0x2u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x104u64), (0x0u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0xc8u64)) as u64; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { r8 = rdi; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_add(0x68u64)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(4)), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 20; continue; }
            20 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x28u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x80u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x29u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x81u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x2au64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x82u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x2bu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x83u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x98u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x3cu64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x44u64), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), (0xff7fffffu32) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), ((rax as u32)) as u32); rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 21; continue; }
            21 => { rax = rdi.wrapping_add(0x8); rdi = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_add(0x58u64)) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rcx = rbx.wrapping_add(0x88); rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { crate::simd_ops::wu32(rbx.wrapping_add(0xfcu64), (0x2u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x100u64), (0x3u64) as u64); crate::simd_ops::wu16(rbx.wrapping_add(0x108u64), (0x101u16) as u16); crate::simd_ops::wu8(r14, (0x1u8) as u8); rax = r14; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a1200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426fd0; const P1: u64 = 0x14342668c; const P2: u64 = 0x1434264e4; const P3: u64 = 0x141720ed0; const P4: u64 = 0x1416a1cb0; const P5: u64 = 0x143932190;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); rdi = r9; rsi = r8; r14 = rdx; rbp = rcx; xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm8 = (xmm1) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { r11 = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rdx = r11.wrapping_add(0x10); rax = ((r8.wrapping_sub(0x4u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; r10 = ((rax.wrapping_mul(4)) as u32) as u32 as u64; rbx = ((rax.wrapping_mul(4)) as u64) as u64; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x10u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x70u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm8 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm8 as u32) }).to_bits() as u128); rax = (crate::simd_ops::ru64(rdx.wrapping_sub(0x8u64))) as u64; xmm8 = (xmm0) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((if f32::from_bits(xmm8 as u32) < f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(0x70u64))) { f32::from_bits(xmm8 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(0x70u64))) }).to_bits() as u128); rax = (crate::simd_ops::ru64(rdx)) as u64; xmm8 = (xmm8 & !0xffff_ffffu128) | ((if f32::from_bits(xmm8 as u32) < f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(0x70u64))) { f32::from_bits(xmm8 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(0x70u64))) }).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((if f32::from_bits(xmm8 as u32) < f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x70u64))) { f32::from_bits(xmm8 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x70u64))) }).to_bits() as u128); rdx = rdx.wrapping_add(0x20); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { bb = 10; continue; }
            6 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { r11 = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = ((r11.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0x70u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm8 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm8 as u32) }).to_bits() as u128); xmm8 = (xmm0) as u128; rcx = rcx.wrapping_add(0x8); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128); bb = 13; continue; }
            12 => { xmm8 = ((crate::simd_ops::ru32(P2)) as u32) as u128; bb = 13; continue; }
            13 => { xmm6 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32) as u128; xmm7 = (xmm1) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm4 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32) as u128; xmm5 = (xmm0) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(rsi)) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rdi)) as u32) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm4 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm6 as u32)) as u32); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm5 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm7 as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x0u64) as u64); rax = rbp.wrapping_add(0x4c); r9 = rbp.wrapping_add(0x48); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rsp.wrapping_add(0x48); rdx = rsp.wrapping_add(0x30); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a1400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169eda0; const P1: u64 = 0x14169edb0; const P2: u64 = 0x1416a1920; const P3: u64 = 0x1416a1c10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x100u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbp.wrapping_add(0x104u64), (0x4u32) as u32); bb = 5; continue; }
            4 => { crate::simd_ops::wu32(rbp.wrapping_add(0x104u64), (0x3u32) as u32); rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0xf8u64)) as u64; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rsi = ((rsi).wrapping_add(0xe0u64)) as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = rsi; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), rbp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rdi as u32)) as u32); rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a14f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169eda0; const P1: u64 = 0x14169edb0; const P2: u64 = 0x1416a1920; const P3: u64 = 0x1416a1c10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0xf8u64)) as u64; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x100u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rbp.wrapping_add(0x104u64), (0x0u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xfcu64), (0x2u32) as u32); bb = 7; continue; }
            6 => { crate::simd_ops::wu32(rbp.wrapping_add(0x104u64), (0x1u32) as u32); rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rsi = ((rsi).wrapping_add(0xc8u64)) as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = rsi; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), rbp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rdi as u32)) as u32); rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rbp.wrapping_add(0x100u64), (0x3u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x108u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x9cu64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rax.wrapping_add(0x28eu64), (0x1u8) as u8); bb = 10; continue; }
            10 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a1620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x104; const P1: u64 = 0x108;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x5u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a1640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169eda0; const P1: u64 = 0x100; const P2: u64 = 0x104; const P3: u64 = 0xfc; const P4: u64 = 0x14169edb0; const P5: u64 = 0x10; const P6: u64 = 0x1416a1920;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x0u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x2u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rcx = ((rcx).wrapping_add(0xc8u64)) as u64; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            4 => { crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x1u32) as u32); bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rbx.wrapping_add(P1), (0x3u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a16e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169eda0; const P1: u64 = 0x14169edb0; const P2: u64 = 0x1416a1920; const P3: u64 = 0x1416a1c10; const P4: u64 = 0x1416af7c0; const P5: u64 = 0x1417195d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0xc8u64)) as u64; rdx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x100u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rbp.wrapping_add(0x104u64), (0x4u32) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rsi = ((rsi).wrapping_add(0xf8u64)) as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = rsi; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), rbp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rdi as u32)) as u32); rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            6 => { crate::simd_ops::wu32(rbp.wrapping_add(0x104u64), (0x3u32) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rsi = ((rsi).wrapping_add(0xe0u64)) as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = rsi; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), rbp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rdi as u32)) as u32); rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::wu8(rbp.wrapping_add(0x108u64), (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x9cu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x128u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            13 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a1880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x104; const P1: u64 = 0x108;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x2u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a19e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14169eda0; const P1: u64 = 0x14169edb0; const P2: u64 = 0x1416a1920; const P3: u64 = 0x1416a18a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xfcu64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x100u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x104u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rbx.wrapping_add(0x100u64), (0x2u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x108u64), (0x1u8) as u8); bb = 10; continue; }
            8 => { crate::simd_ops::wu32(rbx.wrapping_add(0x100u64), (0x2u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x104u64), (0x4u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0xe0u64)) as u64; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0xf8u64)) as u64; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(0x108u64), (0x1u8) as u8); bb = 10; continue; }
            9 => { crate::simd_ops::wu32(rbx.wrapping_add(0xfcu64), (0x2u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x104u64), (0x0u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0xc8u64)) as u64; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a1b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x3; const P4: u64 = 0x50; const P5: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdi = ((r8.wrapping_add(P3)) as u64) as u64; rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; r9 = ((rdi.wrapping_mul(8)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x83u32) as u32); r8 = rsp.wrapping_add(0x48); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a1bf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x109; const P1: u64 = 0xa0; const P2: u64 = 0x9c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a1cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x1411fa550; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x4; const P4: u64 = 0x1; const P5: u64 = 0x8; const P6: u64 = 0xfffffffffffffff8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((r8 as u32)) as u32 as u64; rbx = rdx; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rsi.wrapping_add(P2)) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(8)))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 5; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P3))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdx = ((rsi as u32)) as u32 as u64; bb = 6; continue; }
            5 => { rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = ((rsi.wrapping_add(P4)) as u32) as u32 as u64; bb = 6; continue; }
            6 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0; fl_l = 0; fl_r = 0; rdx = rsi; bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(rbx))) as u64) as u128; rcx = ((rcx.wrapping_add(P5)) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx).wrapping_add(P6), ((xmm0 as u64)) as u64); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; xmm0 = ((crate::simd_ops::ru64(rcx)) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rsi.wrapping_mul(8)), ((xmm0 as u64)) as u64); bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 12; continue; }
            12 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a1d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x1416a1b40; const P2: u64 = 0x14170b740;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; rbp = rcx; rdi = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; r13 = rdx; rcx = rax; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = (0xfffffffffffffffeu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rsi); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rsi); rcx = rsi; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rsi); r12 = rdi; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = rdi; rdx = rbp.wrapping_add(0x50); rcx = rbp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(r13.wrapping_add(0x38u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 40; continue; } bb = 10; continue; }
            10 => { r15 = rsi; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rax = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 13; continue; }
            13 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rsi); bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rsi); bb = 23; continue; }
            18 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rsi = rdi.wrapping_add(0x3); rsi = (((rsi) & (0xfffffffffffffffcu64))) as u64; r9 = ((rsi.wrapping_mul(8)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            20 => { crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x83u32) as u32); r8 = rsp.wrapping_add(0x50); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            21 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { r8 = rdi; rdx = rbp.wrapping_add(0x50); r8 = ((r8).wrapping_shr(2u32)) as u64; rcx = rbp.wrapping_add(0x50); r8 = ((r8).wrapping_add(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rdi); xmm0 = ((crate::simd_ops::ru64(r15.wrapping_add(r14))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm0 as u64)) as u64); xmm2 = ((crate::simd_ops::ru32(r15.wrapping_add(r14))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(r15.wrapping_add(r14).wrapping_add(0x4u64))) as u32) as u128; r15 = ((r15).wrapping_add(0x8u64)) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32) as u128; xmm1 = (xmm3) as u128; xmm4 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm2 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); xmm0 = (xmm4) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x3cu64))) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x3cu64))) }).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x3cu64), ((xmm1 as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) }).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x44u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x44u64), ((xmm1 as u32)) as u32); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) }).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), ((xmm3 as u32)) as u32); xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) > f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x4cu64))) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x4cu64))) }).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x4cu64), ((xmm4 as u32)) as u32); fl_l = r12; fl_r = (0x1u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 24; continue; }
            24 => { bb = 40; continue; }
            25 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; r14 = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 40; continue; } bb = 26; continue; }
            26 => { bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; r15 = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rdi = rax.wrapping_add(0x1); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 37; continue; } bb = 28; continue; }
            28 => { rax = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 39; continue; } bb = 29; continue; }
            29 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 39; continue; } bb = 30; continue; }
            30 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 34; continue; } bb = 31; continue; }
            31 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rsi); bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rsi); bb = 39; continue; }
            34 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rsi = rdi.wrapping_add(0x3); rsi = (((rsi) & (0xfffffffffffffffcu64))) as u64; r9 = ((rsi.wrapping_mul(8)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; bb = 39; continue; }
            36 => { crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x83u32) as u32); r8 = rsp.wrapping_add(0x50); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; bb = 39; continue; }
            37 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { r8 = rdi; rdx = rbp.wrapping_add(0x50); r8 = ((r8).wrapping_shr(2u32)) as u64; rcx = rbp.wrapping_add(0x50); r8 = ((r8).wrapping_add(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rdi); xmm0 = ((crate::simd_ops::ru64(r15.wrapping_add(r14.wrapping_mul(8)))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), ((xmm0 as u64)) as u64); xmm2 = ((crate::simd_ops::ru32(r15.wrapping_add(r14.wrapping_mul(8)))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(r15.wrapping_add(r14.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u128; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32) as u128; xmm1 = (xmm3) as u128; xmm4 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm2 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); xmm0 = (xmm4) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x3cu64))) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x3cu64))) }).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x3cu64), ((xmm1 as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) }).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x44u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x44u64), ((xmm1 as u32)) as u32); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x48u64))) }).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), ((xmm3 as u32)) as u32); xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) > f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x4cu64))) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rbp.wrapping_add(0x4cu64))) }).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x4cu64), ((xmm4 as u32)) as u32); if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 27; continue; } bb = 40; continue; }
            40 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a20d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xa0; const P1: u64 = 0x214; const P2: u64 = 0x218; const P3: u64 = 0xac; const P4: u64 = 0xb0; const P5: u64 = 0xa8; const P6: u64 = 0x9c; const P7: u64 = 0x109;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32) as u128;
    xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((xmm1 as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x1u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), (0x3u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P7), (0x1u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a2110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa0; const P1: u64 = 0x64; const P2: u64 = 0x68; const P3: u64 = 0x70; const P4: u64 = 0x60; const P5: u64 = 0x74; const P6: u64 = 0x6c; const P7: u64 = 0xb4; const P8: u64 = 0xb8; const P9: u64 = 0xbc; const P10: u64 = 0xc0; const P11: u64 = 0xc4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx); xmm4 = ((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32) as u128; xmm5 = (xmm4) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P3)))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P3)))).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm7) as u128); xmm3 = (xmm2) as u128; xmm7 = ((crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P5)))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P5)))).to_bits() as u128); crate::simd_ops::wu128(rsp, (xmm8) as u128); xmm8 = (xmm7) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P6)))).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P6)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(P7), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P8), ((xmm4 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P9), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P10), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P11), ((xmm5 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xc8u64), ((xmm3 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x84u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp)) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0xccu64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x94u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0xdcu64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0xa4u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0xecu64), (xmm0) as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x9cu64), (0x1u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x109u64), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a21e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xa0; const P1: u64 = 0x38; const P2: u64 = 0x3c; const P3: u64 = 0xac; const P4: u64 = 0xb0; const P5: u64 = 0xa8; const P6: u64 = 0x9c; const P7: u64 = 0x109;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32) as u128;
    xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((xmm1 as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P5), (0x1u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), (0x2u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P7), (0x1u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a2220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x3; const P2: u64 = 0x4; const P3: u64 = 0x14183acbc; const P4: u64 = 0x14183acc2;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rsi = ((((rsi as u32)) & (0xfffffffcu32))) as u32 as u64; r10 = r8; rbp = ((rsi as u32)) as u32 as u64; rbx = rcx; rbp = (((rbp as u32)).wrapping_sub((r9 as u32))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(rcx, ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdx, rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru32(rcx)) as u32 as u64; rdi = ((rdi).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rax = ((rdi as u32)) as u32 as u64; r14 = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rdx, ((r9 as u32)) as u32); rax = (((rax as u32)).wrapping_sub(0x4u32)) as u32 as u64; r8 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32); rcx = rdi; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = r10; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = ((rbp as u32)) as u32 as u64; rcx = ((r14.wrapping_add(rdi)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u32(rbx, ((rsi as u32)) as u32); rax = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a22f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa6d8; const P1: u64 = 0x14169e5d0; const P2: u64 = 0x143932190; const P3: u64 = 0x1416a2ea0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r14); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r14); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), r14); crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), (0x2u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), r14); rax = P0; crate::simd_ops::wu64(rcx, rax); rcx = ((rcx).wrapping_add(0x38u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), r14); crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), r14); crate::simd_ops::wu64(rsi.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), r14); crate::simd_ops::wu64(rsi.wrapping_add(0x60u64), r14); crate::simd_ops::wu64(rsi.wrapping_add(0x68u64), r14); crate::simd_ops::wu64(rsi.wrapping_add(0x70u64), r14); crate::simd_ops::wu64(rsi.wrapping_add(0x78u64), r14); crate::simd_ops::wu64(rsi.wrapping_add(0x80u64), r14); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; rdi = rax.wrapping_add(0x1); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsi.wrapping_add(0x50u64), r14); bb = 8; continue; }
            5 => { rbp = rdi.wrapping_add(0x3); rbp = (((rbp) & (0xfffffffffffffffcu64))) as u64; rdx = ((rbp.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x50u64), rbp); bb = 8; continue; }
            6 => { fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; r8 = ((r8).wrapping_add(rdi)) as u64; rdx = rsi.wrapping_add(0x40); rcx = rsi.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rdi); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_sub(0x8u64), r14); rax = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a2400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x58; const P4: u64 = 0x1411fafc0; const P5: u64 = 0x40; const P6: u64 = 0x38; const P7: u64 = 0x8; const P8: u64 = 0x1433e13f0; const P9: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rbx = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P7), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = P8; crate::simd_ops::wu64(rbx, rax); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rdx = 0x88; rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a24c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416a2510;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((r9 as u32)) as u32 as u64;
    rbp = ((r8 as u32)) as u32 as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rsi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r9 = ((rdi as u32)) as u32 as u64;
    r8 = ((rbx as u32)) as u32 as u64;
    rdx = ((rbp as u32)) as u32 as u64;
    rcx = rsi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a2510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416a2f50; const P1: u64 = 0x1411fa5f0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; r15 = ((r9 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x78u64))) as u32 as u64; r12 = ((r8 as u32)) as u32 as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; rbp = rcx; r14 = ((rdx as u32)) as u32 as u64; rbx = rax.wrapping_add(0x1); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { r10 = ((r10).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = r10; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { r8 = rbx; bb = 5; continue; }
            3 => { fl_l = rbx; fl_r = r10; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rdx = rcx.wrapping_add(0x70); rcx = ((rcx).wrapping_add(0x70u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_sub(0xcu64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_sub(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_sub(0x4u64), ((r15 as u32)) as u32); rcx = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rbx = rax.wrapping_add(0x1); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { r8 = rbx; bb = 11; continue; }
            9 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { rdx = rdi; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rsi as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rbx.wrapping_mul(4)).wrapping_sub(0x4u64), ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a2610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x48; const P2: u64 = 0x1; const P3: u64 = 0x40; const P4: u64 = 0x50; const P5: u64 = 0x1416a2ea0; const P6: u64 = 0x8; const P7: u64 = 0xfffffffffffffff8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rsi = rdx; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0xffffffff; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rbx = ((rax.wrapping_add(P2)) as u64) as u64; rdi = ((rcx.wrapping_add(P3)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx).wrapping_shr(1u32)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { r8 = rbx; bb = 7; continue; }
            5 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { r8 = rbx; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rdx = rdi; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rbp as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P6), rbx); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(P7), rsi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a26a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x1416a3000; const P2: u64 = 0x1416a27c0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(r11.wrapping_sub(0x2cu64), ((r8 as u32)) as u32); crate::simd_ops::wu16(r11.wrapping_sub(0x28u64), ((r9 as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rbp as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), (0x0u64) as u64); rsi = rcx.wrapping_add(0x58); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r14 = rax.wrapping_add(0x1); fl_l = r14; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rbx = r14.wrapping_sub(0x2); rbx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = (0xffffffffffffffffu64) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_sub(0x30u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { r8 = r14; bb = 9; continue; }
            7 => { fl_l = r14; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { r8 = r14; r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(r14)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { rdx = rsi; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r14); rcx = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rax).wrapping_add(0xffffffffffffffd0u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rax = ((rbp as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a27c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x20; const P6: u64 = 0x1411fa5f0; const P7: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru16(rdx.wrapping_add(P2))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(P2), ((rax as u16)) as u16); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32); rsi = ((rcx.wrapping_add(P4)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); crate::simd_ops::wu64(rsi.wrapping_add(P2), rax); rdi = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r8 = rdi; r8 = ((r8).wrapping_shr(2u32)) as u64; r8 = ((r8).wrapping_add(rdi)) as u64; rdx = rsi; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(P0), rdi); rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); rbx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = ((rax.wrapping_add(P7)) as u64) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a2870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acbc; const P1: u64 = 0x14183acc2; const P2: u64 = 0x1426aa698; const P3: u64 = 0x1416a2e90; const P4: u64 = 0x1416c90f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rbx = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; bb = 2; continue; }
            2 => { r8 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = ((rcx).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 5; continue; }
            4 => { rcx = r8; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rdx).wrapping_add(0xcu64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rax = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { rax = rax.wrapping_add(0x1); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) != ((fl_r as u32))) { r8 } else { rdi }) as u64; rbp = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; r14 = ((rbp.wrapping_add(0x3u64)) as u32) as u32 as u64; r14 = ((((r14 as u32)) & (0xfffffffcu32))) as u32 as u64; r15 = ((r14 as u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_sub((rbp as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, ((r14 as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 17; continue; }
            9 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rcx, r8); bb = 17; continue; }
            11 => { rsi = (crate::simd_ops::ru32(rdi)) as u32 as u64; rsi = ((rsi).wrapping_add(rax)) as u64; crate::simd_ops::wu32(rcx, ((rbp as u32)) as u32); rax = ((rsi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x4u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); r8 = ((rbp as u32)) as u32 as u64; rcx = rsi; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { r8 = ((r15 as u32)) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { crate::simd_ops::lk_add_u32(rdi, ((r14 as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { r13 = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((r8 as u32)) as u32); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = ((rcx).wrapping_add(0x14u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 20; continue; }
            19 => { rcx = r8; bb = 20; continue; }
            20 => { r14 = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; rdi = (if (((fl_l as u32)) != ((fl_r as u32))) { r8 } else { rdi }) as u64; rbp = ((r14.wrapping_add(r14.wrapping_mul(2))) as u32) as u32 as u64; rbp = (((rbp as u32)).wrapping_shl(2u32)) as u32 as u64; r15 = ((rbp.wrapping_add(0x3u64)) as u32) as u32 as u64; r15 = ((((r15 as u32)) & (0xfffffffcu32))) as u32 as u64; r12 = ((r15 as u32)) as u32 as u64; r12 = (((r12 as u32)).wrapping_sub((rbp as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rsi = r8; bb = 29; continue; }
            22 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rcx, (0x0u64) as u64); rsi = r8; bb = 30; continue; }
            24 => { rsi = (crate::simd_ops::ru32(rdi)) as u32 as u64; rsi = ((rsi).wrapping_add(rax)) as u64; crate::simd_ops::wu32(rcx, ((r14 as u32)) as u32); rax = ((rsi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x4u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); r8 = ((rbp as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = rsi; rcx = r14; r8 = 0; fl_l = 0; fl_r = 0; bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rax, r8); rax = rax.wrapping_add(0xc); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 27; continue; }
            27 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { r8 = ((r12 as u32)) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, ((r15 as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 30; continue; }
            30 => { rbp = 0x1; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 39; continue; } bb = 31; continue; }
            31 => { r15 = P2; r13 = 0; fl_l = 0; fl_r = 0; bb = 32; continue; }
            32 => { rdi = ((rbp as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x2u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rcx); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r14 = ((rsi.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; bb = 35; continue; }
            34 => { r14 = r13; bb = 35; continue; }
            35 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; fl_l = (crate::simd_ops::lk_or_u8(rsi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8u64), (0x2u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 37; continue; }
            37 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 32; continue; } bb = 38; continue; }
            38 => { r13 = (crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32 as u64; bb = 39; continue; }
            39 => { r11 = 0; fl_l = 0; fl_r = 0; r10 = ((r11 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((r11 as u32)) as u32); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = ((rcx).wrapping_add(0xcu64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 42; continue; }
            41 => { rcx = r11; bb = 42; continue; }
            42 => { rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) != ((fl_r as u32))) { r11 } else { rdi }) as u64; r14 = ((rsi.wrapping_add(rsi.wrapping_mul(8))) as u32) as u32 as u64; r14 = (((r14 as u32)).wrapping_shl(2u32)) as u32 as u64; r15 = ((r14.wrapping_add(0x3u64)) as u32) as u32 as u64; r15 = ((((r15 as u32)) & (0xfffffffcu32))) as u32 as u64; r12 = ((r15 as u32)) as u32 as u64; r12 = (((r12 as u32)).wrapping_sub((r14 as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, ((r15 as u32)) as u32)) as u32 as u64; fl_r = fl_l; rbp = r11; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); bb = 52; continue; }
            44 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { crate::simd_ops::wu64(rcx, r11); rbp = r11; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); bb = 52; continue; }
            46 => { rbp = (crate::simd_ops::ru32(rdi)) as u32 as u64; rbp = ((rbp).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu32(rcx, ((rsi as u32)) as u32); rax = ((rbp as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x4u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((rax as u32)) as u32); r8 = ((r14 as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 49; continue; } bb = 47; continue; }
            47 => { rax = rbp.wrapping_add(0x20); rcx = rsi; rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x1; bb = 48; continue; }
            48 => { crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdx); crate::simd_ops::wu32(rax.wrapping_sub(0x18u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x14u64), (0xffffffffu32) as u32); crate::simd_ops::wu16(rax.wrapping_sub(0x10u64), ((r8 as u16)) as u16); crate::simd_ops::wu64(rax.wrapping_sub(0xcu64), rdx); crate::simd_ops::wu64(rax.wrapping_sub(0x4u64), rdx); rax = rax.wrapping_add(0x24); fl_l = rcx; fl_r = r8; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 48; continue; } bb = 49; continue; }
            49 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 51; continue; } bb = 50; continue; }
            50 => { r8 = ((r12 as u32)) as u32 as u64; rcx = ((r14 as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 51; continue; }
            51 => { crate::simd_ops::lk_add_u32(rdi, ((r15 as u32)) as u32); r10 = (crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; bb = 52; continue; }
            52 => { rax = ((r11 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xd0u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 82; continue; } bb = 53; continue; }
            53 => { bb = 54; continue; }
            54 => { rsi = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 58; continue; } bb = 55; continue; }
            55 => { xmm0 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(rsi.wrapping_mul(4)), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 58; continue; } bb = 56; continue; }
            56 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 58; continue; } bb = 57; continue; }
            57 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x10u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 58; continue; }
            58 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = ((rsi.wrapping_mul(4).wrapping_add(0x14u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r11 } else { rcx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); r15 = rbp.wrapping_add(0x14); r15 = ((r15.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r11 } else { r15 }) as u64; r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((r14 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; rcx = ((r14.wrapping_add(r14)) as u32) as u32 as u64; r12 = ((rcx.wrapping_add(0x3u64)) as u32) as u32 as u64; r12 = ((((r12 as u32)) & (0xfffffffcu32))) as u32 as u64; rax = ((r12 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xd8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 60; continue; } bb = 59; continue; }
            59 => { crate::simd_ops::lk_add_u32(rbp, ((r12 as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; r14 = ((r8 as u32)) as u32 as u64; bb = 65; continue; }
            60 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 62; continue; } bb = 61; continue; }
            61 => { r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r15, r8); r14 = ((r8 as u32)) as u32 as u64; bb = 65; continue; }
            62 => { r14 = (crate::simd_ops::ru32(rbp)) as u32 as u64; r14 = ((r14).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; crate::simd_ops::wu32(r15, ((rax as u32)) as u32); rax = ((r14 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x28u64)))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x4u32)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x4u64), ((rax as u32)) as u32); r8 = ((rcx as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xd8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 64; continue; } bb = 63; continue; }
            63 => { r8 = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rcx as u32))) as u32 as u64; rcx = ((rcx).wrapping_add(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 64; continue; }
            64 => { crate::simd_ops::lk_add_u32(rbp, ((r12 as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; bb = 65; continue; }
            65 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = rax.wrapping_add(0x1c); rdx = ((rdx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; fl_l = rcx; fl_r = rcx; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rdx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdx); r15 = rax.wrapping_add(0x1c); r15 = ((r15.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { r15 }) as u64; r12 = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; rdx = ((r12.wrapping_mul(4)) as u32) as u32 as u64; rax = ((rdx.wrapping_add(0x3u64)) as u32) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffcu32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xd8u64), ((rax as u32)) as u32); rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 67; continue; } bb = 66; continue; }
            66 => { crate::simd_ops::lk_add_u32(rbp, ((rax as u32)) as u32); r11 = 0; fl_l = 0; fl_r = 0; rsi = ((r11 as u32)) as u32 as u64; bb = 74; continue; }
            67 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 69; continue; } bb = 68; continue; }
            68 => { r11 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r15, r11); rsi = ((r11 as u32)) as u32 as u64; bb = 74; continue; }
            69 => { rsi = (crate::simd_ops::ru32(rbp)) as u32 as u64; rsi = ((rsi).wrapping_add(rcx)) as u64; crate::simd_ops::wu32(r15, ((r12 as u32)) as u32); rax = ((rsi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x28u64)))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x4u32)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x4u64), ((rax as u32)) as u32); r8 = ((rdx as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 71; continue; } bb = 70; continue; }
            70 => { r8 = r12; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = 0xffffffff; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 71; continue; }
            71 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 73; continue; } bb = 72; continue; }
            72 => { r8 = ((rax as u32)) as u32 as u64; rcx = ((r12.wrapping_mul(4)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 73; continue; }
            73 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xd8u64))) as u32 as u64; crate::simd_ops::lk_add_u32(rbp, ((rax as u32)) as u32); r11 = 0; fl_l = 0; fl_r = 0; bb = 74; continue; }
            74 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32 as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 81; continue; } bb = 75; continue; }
            75 => { rax = ((r10.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(0x10u64))) as u16 as u64; fl_r = (0x1u16) as u16 as u64; rax = (if (((fl_l as u16)) != ((fl_r as u16))) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((rax as u32)) as u32); rdx = ((r11 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 81; continue; } bb = 76; continue; }
            76 => { r8 = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; r9 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 79; continue; } bb = 77; continue; }
            77 => { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu16(r14.wrapping_add(r8.wrapping_mul(2)), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 79; continue; } bb = 78; continue; }
            78 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(r8.wrapping_mul(4)), ((rax as u32)) as u32); bb = 79; continue; }
            79 => { r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 76; continue; } bb = 80; continue; }
            80 => { crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((r10 as u32)) as u32); bb = 81; continue; }
            81 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xd0u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 54; continue; } bb = 82; continue; }
            82 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 85; continue; } bb = 83; continue; }
            83 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x1cu64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 85; continue; } bb = 84; continue; }
            84 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((r10 as u32)) as u32); bb = 85; continue; }
            85 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a2e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a2ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x3; const P4: u64 = 0x50; const P5: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdi = ((r8.wrapping_add(P3)) as u64) as u64; rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; r9 = ((rdi.wrapping_mul(8)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); r8 = rsp.wrapping_add(0x48); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a2f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x3; const P4: u64 = 0x50; const P5: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdi = ((r8.wrapping_add(P3)) as u64) as u64; rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; r9 = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_shl(2u32)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); r8 = rsp.wrapping_add(0x48); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a3000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x3; const P4: u64 = 0x50; const P5: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = r8; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdi = ((r8.wrapping_add(P3)) as u64) as u64; rdi = (((rdi) & (0xfffffffffffffffcu64))) as u64; r9 = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsp.wrapping_add(0x48); rdx = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a30b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(P1)))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(P3)))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_l = (((rdx as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(P1)))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a30e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416a3230; const P1: u64 = 0x10; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64;
    rbx = rdx;
    rdx = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    rdi = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    r8 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rdx as u32) } else { (r8 as u32) }) as u32 as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rdi, (0x0u32) as u32);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rax);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a3130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1426aa6f0; const P1: u64 = 0x8; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x10; const P5: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x1u32) as u32);
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a3160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa6f0; const P1: u64 = 0x20; const P2: u64 = 0x143932190; const P3: u64 = 0x60; const P4: u64 = 0x1433e1408; const P5: u64 = 0x14169ed90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rax = P4;
    crate::simd_ops::wu64(rbx, rax);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a31a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa6f0; const P1: u64 = 0x20; const P2: u64 = 0x143932190; const P3: u64 = 0x60; const P4: u64 = 0x1433e1408; const P5: u64 = 0x14169ed90; const P6: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = P4; crate::simd_ops::wu64(rbx, rax); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = 0x30; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a3230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x1c; const P2: u64 = 0x10; const P3: u64 = 0x28; const P4: u64 = 0x20; const P5: u64 = 0x143932190; const P6: u64 = 0x60; const P7: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rsi = r9; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdi as u32)) as u32); rbx = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rcx.wrapping_add(P2), (0x0u64) as u64); rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P5)) as u64; fl_l = rsi; fl_r = rsi; rdx = rdi; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rsi } else { rcx }) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a32e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x143932190; const P4: u64 = 0x60; const P5: u64 = 0x1c; const P6: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, r14);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r14 = 0;
    fl_l = 0; fl_r = 0;
    rbp = rdx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = ((r9 as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r14);
    rsi = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r14);
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp);
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rsi as u32)) as u32);
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rdi as u32)) as u32);
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), r14);
    crate::simd_ops::wu32(rbx.wrapping_add(P6), ((r14 as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    r14 = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a33d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x14169edb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rsi.wrapping_sub(0x1); rbx = ((rax.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rbx = ((rbx).wrapping_add(0xfffffffffffffff0u64)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = ((rbx).wrapping_sub(0x10u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbp); crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rbp as u32)) as u32); rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rbx = rsi.wrapping_sub(0x1); rbx = ((rbx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rbx = ((rax.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rbx = ((rbx).wrapping_sub(0x28u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rbp); crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rbp); crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((rbp as u32)) as u32); rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; rbx = ((rbx).wrapping_add(0xffffffffffffffe0u64)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rbx = ((rbx).wrapping_sub(0x20u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rbp); crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((rbp as u32)) as u32); rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; rbx = rsi.wrapping_sub(0x1); rbx = ((rax.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rbx = ((rbx).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), rbp); crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rbp as u32)) as u32); rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rbx = rsi.wrapping_sub(0x1); rbx = ((rax.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { rbx = ((rbx).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rbp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a35b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x14183acc2; const P2: u64 = 0x1416a3360; const P3: u64 = 0x1416a33d0; const P4: u64 = 0x14169eda0; const P5: u64 = 0x14169edb0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x1fu64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; r14 = (rcx) as u64; r13 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), (r13) as u64); r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; r12 = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x65u64), ((r12 as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x48u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), ((rcx as u32)) as u32); r10 = (crate::simd_ops::ru32(r14.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x77u64), ((r10 as u32)) as u32); rbx = (crate::simd_ops::ru32(r14.wrapping_add(0x90u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((rbx as u32)) as u32); rdi = (crate::simd_ops::ru32(r14.wrapping_add(0xa8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x69u64), ((rdi as u32)) as u32); rdx = (crate::simd_ops::ru64(r13)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 42; continue; } bb = 1; continue; }
            1 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 195; continue; } bb = 8; continue; }
            8 => { r9 = ((r12 as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shl(4u32)) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1cu32) as i32) as u32) as u32 as u64; rcx = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(5u32)) as u32 as u64; rax = ((rdi.wrapping_add(rbx)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((r15 as u32))) as u32 as u64; rbx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0x78u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((rdx as u32))) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((r8 as u32))) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((r9 as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbx as u32)) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x74u64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), (rax) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x70u64), ((rbx as u32)) as u32); rsi = ((rax.wrapping_add(0x78u64)) as u64) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r15 as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu64(rax, (rsi) as u64); r8 = (r15) as u64; rsi = ((rsi.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            10 => { crate::simd_ops::wu64(rax, (rbx) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 11; continue; }
            11 => { rcx = (0xffffu32) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((r12 as u32)) as u32); rdx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rsi) as u64); r8 = (r12) as u64; rax = (r12) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdx, (rbx) as u64); crate::simd_ops::wu16(rdx.wrapping_add(0x8u64), ((rcx as u16)) as u16); rdx = ((rdx.wrapping_add(0x10u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { bb = 17; continue; }
            16 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rbx as u32)) as u32); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); rdx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), (rsi) as u64); r8 = ((rax as u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rsi = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { rax = ((rdx.wrapping_add(0x18u64)) as u64) as u64; bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rdx, (rbx) as u64); crate::simd_ops::wu16(rax.wrapping_sub(0x10u64), ((rcx as u16)) as u16); crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax, (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); rdx = ((rdx.wrapping_add(0x28u64)) as u64) as u64; rax = ((rax.wrapping_add(0x28u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { bb = 23; continue; }
            22 => { crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rbx as u32)) as u32); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((rax as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), (rsi) as u64); rdx = ((rax as u32)) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x1cu64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { rax = ((rcx.wrapping_add(0x14u64)) as u64) as u64; bb = 26; continue; }
            26 => { crate::simd_ops::wu32(rcx, (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x10u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0xcu64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x8u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x4u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax, (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), (0xbf800000u32) as u32); rcx = ((rcx.wrapping_add(0x1cu64)) as u64) as u64; rax = ((rax.wrapping_add(0x1cu64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 27; continue; }
            27 => { bb = 29; continue; }
            28 => { crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((rbx as u32)) as u32); bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 34; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((rax as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), (rsi) as u64); r8 = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 31; continue; }
            31 => { rax = ((rcx.wrapping_add(0x18u64)) as u64) as u64; rdx = (0x3fffu32) as u32 as u64; bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rcx, (rbx) as u64); crate::simd_ops::wu16(rax.wrapping_sub(0x10u64), ((rdx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_sub(0x8u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x4u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax, (0x7f7fffffu32) as u32); rcx = ((rcx.wrapping_add(0x20u64)) as u64) as u64; rax = ((rax.wrapping_add(0x20u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 33; continue; }
            33 => { bb = 35; continue; }
            34 => { crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((rbx as u32)) as u32); bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rax as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), (rsi) as u64); r8 = ((rax as u32)) as u32 as u64; rsi = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 38; continue; }
            37 => { crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rbx as u32)) as u32); bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x69u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), (rsi) as u64); r8 = ((rax as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 41; continue; }
            40 => { crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rbx as u32)) as u32); bb = 41; continue; }
            41 => { rdx = ((rbp.wrapping_sub(0x61u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 77; continue; }
            42 => { r8 = ((r12 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(4u32)) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_shl(3u32)) as u32 as u64; r11 = ((((rcx as u32)) as i32).wrapping_mul((0x1cu32) as i32) as u32) as u32 as u64; r10 = (((r10 as u32)).wrapping_shl(5u32)) as u32 as u64; rcx = ((rbx.wrapping_add(0xfu64)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rdi as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r15 as u32))) as u32 as u64; rcx = ((r10.wrapping_add(rcx.wrapping_mul(8))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r11 as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r9 as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r8 as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x70u64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 80; continue; } bb = 43; continue; }
            43 => { rax = ((r15.wrapping_add(rdi)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; rbx = ((r10.wrapping_add(rax.wrapping_mul(8))) as u32) as u32 as u64; r11 = (((r11 as u32)).wrapping_add(0x78u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((r11 as u32))) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((r9 as u32))) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((r8 as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbx as u32)) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x74u64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), (rax) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x70u64), ((rbx as u32)) as u32); rsi = ((rax.wrapping_add(0x78u64)) as u64) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r15 as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu64(rax, (rsi) as u64); r8 = (r15) as u64; rsi = ((rsi.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 46; continue; }
            45 => { crate::simd_ops::wu64(rax, (rbx) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 46; continue; }
            46 => { rcx = (0xffffu32) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 51; continue; } bb = 47; continue; }
            47 => { crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((r12 as u32)) as u32); rdx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rsi) as u64); r8 = (r12) as u64; rax = (r12) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 52; continue; } bb = 48; continue; }
            48 => { bb = 49; continue; }
            49 => { crate::simd_ops::wu64(rdx, (rbx) as u64); crate::simd_ops::wu16(rdx.wrapping_add(0x8u64), ((rcx as u16)) as u16); rdx = ((rdx.wrapping_add(0x10u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 49; continue; } bb = 50; continue; }
            50 => { bb = 52; continue; }
            51 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rbx as u32)) as u32); bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 57; continue; } bb = 53; continue; }
            53 => { crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); rdx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), (rsi) as u64); r8 = ((rax as u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rsi = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 58; continue; } bb = 54; continue; }
            54 => { rax = ((rdx.wrapping_add(0x18u64)) as u64) as u64; bb = 55; continue; }
            55 => { crate::simd_ops::wu64(rdx, (rbx) as u64); crate::simd_ops::wu16(rax.wrapping_sub(0x10u64), ((rcx as u16)) as u16); crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax, (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); rdx = ((rdx.wrapping_add(0x28u64)) as u64) as u64; rax = ((rax.wrapping_add(0x28u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 55; continue; } bb = 56; continue; }
            56 => { bb = 58; continue; }
            57 => { crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rbx as u32)) as u32); bb = 58; continue; }
            58 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 63; continue; } bb = 59; continue; }
            59 => { crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((rax as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), (rsi) as u64); rdx = ((rax as u32)) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x1cu64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 60; continue; }
            60 => { rax = ((rcx.wrapping_add(0x14u64)) as u64) as u64; bb = 61; continue; }
            61 => { crate::simd_ops::wu32(rcx, (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x10u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0xcu64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x8u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x4u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax, (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), (0xbf800000u32) as u32); rcx = ((rcx.wrapping_add(0x1cu64)) as u64) as u64; rax = ((rax.wrapping_add(0x1cu64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 61; continue; } bb = 62; continue; }
            62 => { bb = 64; continue; }
            63 => { crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((rbx as u32)) as u32); bb = 64; continue; }
            64 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 69; continue; } bb = 65; continue; }
            65 => { crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((rax as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), (rsi) as u64); r8 = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 70; continue; } bb = 66; continue; }
            66 => { rax = ((rcx.wrapping_add(0x18u64)) as u64) as u64; rdx = (0x3fffu32) as u32 as u64; bb = 67; continue; }
            67 => { crate::simd_ops::wu64(rcx, (rbx) as u64); crate::simd_ops::wu16(rax.wrapping_sub(0x10u64), ((rdx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_sub(0x8u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x4u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax, (0x7f7fffffu32) as u32); rcx = ((rcx.wrapping_add(0x20u64)) as u64) as u64; rax = ((rax.wrapping_add(0x20u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 67; continue; } bb = 68; continue; }
            68 => { bb = 70; continue; }
            69 => { crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((rbx as u32)) as u32); bb = 70; continue; }
            70 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 72; continue; } bb = 71; continue; }
            71 => { crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rax as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), (rsi) as u64); r8 = ((rax as u32)) as u32 as u64; rsi = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 73; continue; }
            72 => { crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rbx as u32)) as u32); bb = 73; continue; }
            73 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x69u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 75; continue; } bb = 74; continue; }
            74 => { crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), (rsi) as u64); r8 = ((rax as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 76; continue; }
            75 => { crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rbx as u32)) as u32); bb = 76; continue; }
            76 => { rdx = ((rbp.wrapping_sub(0x61u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 77; continue; }
            77 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x74u64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x74u64), ((rax as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 79; continue; } bb = 78; continue; }
            78 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 79; continue; }
            79 => { bb = 112; continue; }
            80 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(r13)) as u64; rsi = ((rdi.wrapping_add(0x78u64)) as u64) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 82; continue; } bb = 81; continue; }
            81 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu64(rdi, (rsi) as u64); r8 = (r15) as u64; rsi = ((rsi.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 83; continue; }
            82 => { crate::simd_ops::wu64(rdi, (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 83; continue; }
            83 => { rcx = (0xffffu32) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 87; continue; } bb = 84; continue; }
            84 => { crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((r12 as u32)) as u32); rdx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rsi) as u64); r8 = (r12) as u64; rax = (r12) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 88; continue; } bb = 85; continue; }
            85 => { crate::simd_ops::wu64(rdx, (rbx) as u64); crate::simd_ops::wu16(rdx.wrapping_add(0x8u64), ((rcx as u16)) as u16); rdx = ((rdx.wrapping_add(0x10u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 85; continue; } bb = 86; continue; }
            86 => { bb = 88; continue; }
            87 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rbx as u32)) as u32); bb = 88; continue; }
            88 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 93; continue; } bb = 89; continue; }
            89 => { crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); rdx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), (rsi) as u64); r8 = ((rax as u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rsi = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 94; continue; } bb = 90; continue; }
            90 => { rax = ((rdx.wrapping_add(0x18u64)) as u64) as u64; bb = 91; continue; }
            91 => { crate::simd_ops::wu64(rdx, (rbx) as u64); crate::simd_ops::wu16(rax.wrapping_sub(0x10u64), ((rcx as u16)) as u16); crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax, (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); rdx = ((rdx.wrapping_add(0x28u64)) as u64) as u64; rax = ((rax.wrapping_add(0x28u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 91; continue; } bb = 92; continue; }
            92 => { bb = 94; continue; }
            93 => { crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rbx as u32)) as u32); bb = 94; continue; }
            94 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 99; continue; } bb = 95; continue; }
            95 => { crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((rax as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), (rsi) as u64); rdx = ((rax as u32)) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x1cu64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 100; continue; } bb = 96; continue; }
            96 => { rax = ((rcx.wrapping_add(0x14u64)) as u64) as u64; bb = 97; continue; }
            97 => { crate::simd_ops::wu32(rcx, (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x10u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0xcu64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x8u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x4u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax, (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), (0xbf800000u32) as u32); rcx = ((rcx.wrapping_add(0x1cu64)) as u64) as u64; rax = ((rax.wrapping_add(0x1cu64)) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 97; continue; } bb = 98; continue; }
            98 => { bb = 100; continue; }
            99 => { crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((rbx as u32)) as u32); bb = 100; continue; }
            100 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 105; continue; } bb = 101; continue; }
            101 => { crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((rax as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), (rsi) as u64); r8 = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 106; continue; } bb = 102; continue; }
            102 => { rax = ((rcx.wrapping_add(0x18u64)) as u64) as u64; rdx = (0x3fffu32) as u32 as u64; bb = 103; continue; }
            103 => { crate::simd_ops::wu64(rcx, (rbx) as u64); crate::simd_ops::wu16(rax.wrapping_sub(0x10u64), ((rdx as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_sub(0x8u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x4u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rax, (0x7f7fffffu32) as u32); rcx = ((rcx.wrapping_add(0x20u64)) as u64) as u64; rax = ((rax.wrapping_add(0x20u64)) as u64) as u64; fl_l = (r8) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 103; continue; } bb = 104; continue; }
            104 => { bb = 106; continue; }
            105 => { crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((rbx as u32)) as u32); bb = 106; continue; }
            106 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 108; continue; } bb = 107; continue; }
            107 => { crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rax as u32)) as u32); rcx = (rsi) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), (rsi) as u64); r8 = ((rax as u32)) as u32 as u64; rsi = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 109; continue; }
            108 => { crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rbx as u32)) as u32); bb = 109; continue; }
            109 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x69u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 111; continue; } bb = 110; continue; }
            110 => { crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), (rsi) as u64); r8 = ((rax as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 112; continue; }
            111 => { crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), (rbx) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rbx as u32)) as u32); bb = 112; continue; }
            112 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 126; continue; } bb = 113; continue; }
            113 => { rsi = (rbx) as u64; r12 = (r15) as u64; bb = 114; continue; }
            114 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rsi.wrapping_add(rcx))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 116; continue; } bb = 115; continue; }
            115 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; bb = 116; continue; }
            116 => { crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), (rdi) as u64); rax = (crate::simd_ops::ru64(r13)) as u64; r15 = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 118; continue; } bb = 117; continue; }
            117 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; bb = 118; continue; }
            118 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(r15))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 121; continue; } bb = 119; continue; }
            119 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 121; continue; } bb = 120; continue; }
            120 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 121; continue; }
            121 => { crate::simd_ops::wu64(rsi.wrapping_add(r15), (rdi) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 124; continue; } bb = 122; continue; }
            122 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 124; continue; } bb = 123; continue; }
            123 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 124; continue; }
            124 => { rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = (r12) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 114; continue; } bb = 125; continue; }
            125 => { r12 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x65u64))) as u32 as u64; bb = 126; continue; }
            126 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 139; continue; } bb = 127; continue; }
            127 => { rsi = (rbx) as u64; bb = 128; continue; }
            128 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rsi.wrapping_add(rdx))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 130; continue; } bb = 129; continue; }
            129 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; bb = 130; continue; }
            130 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (rdi) as u64); rax = ((crate::simd_ops::ru16(rsi.wrapping_add(rdx).wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x21u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(r13)) as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 132; continue; } bb = 131; continue; }
            131 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; bb = 132; continue; }
            132 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(r15))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 135; continue; } bb = 133; continue; }
            133 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 135; continue; } bb = 134; continue; }
            134 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 135; continue; }
            135 => { crate::simd_ops::wu64(rsi.wrapping_add(r15), (rdi) as u64); rax = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x21u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(r15).wrapping_add(0x8u64), ((rax as u16)) as u16); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 138; continue; } bb = 136; continue; }
            136 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 138; continue; } bb = 137; continue; }
            137 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 138; continue; }
            138 => { rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = (r12) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 128; continue; } bb = 139; continue; }
            139 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 152; continue; } bb = 140; continue; }
            140 => { rdi = (rbx) as u64; r12 = ((rax as u32)) as u32 as u64; bb = 141; continue; }
            141 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x40u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(rdi))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 143; continue; } bb = 142; continue; }
            142 => { fl_l = (crate::simd_ops::lk_add_u32(rsi, (1) as u32)) as u32 as u64; bb = 143; continue; }
            143 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), (rsi) as u64); rax = ((crate::simd_ops::ru16(rdx.wrapping_add(rdi).wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x49u64), ((rax as u16)) as u16); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(rdi).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(rdi).wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(rdi).wrapping_add(0x1cu64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x35u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(rdi).wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r13)) as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 145; continue; } bb = 144; continue; }
            144 => { fl_l = (crate::simd_ops::lk_add_u32(rsi, (1) as u32)) as u32 as u64; bb = 145; continue; }
            145 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(r15))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 148; continue; } bb = 146; continue; }
            146 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 148; continue; } bb = 147; continue; }
            147 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 148; continue; }
            148 => { crate::simd_ops::wu64(rdi.wrapping_add(r15), (rsi) as u64); rax = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x49u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(r15).wrapping_add(0x8u64), ((rax as u16)) as u16); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(r15).wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x39u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(r15).wrapping_add(0x18u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x35u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(r15).wrapping_add(0x1cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x2du64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(r15).wrapping_add(0x24u64), ((rax as u32)) as u32); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 151; continue; } bb = 149; continue; }
            149 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 151; continue; } bb = 150; continue; }
            150 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 151; continue; }
            151 => { rdi = ((rdi).wrapping_add(0x28u64)) as u64; fl_l = (r12) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 141; continue; } bb = 152; continue; }
            152 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 155; continue; } bb = 153; continue; }
            153 => { r8 = (rbx) as u64; r9 = ((rax as u32)) as u32 as u64; bb = 154; continue; }
            154 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x58u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(r13)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(r8))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(r8), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(r8).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(r8).wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r8).wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_add(0x18u64), ((rax as u32)) as u32); r8 = ((r8.wrapping_add(0x1cu64)) as u64) as u64; fl_l = (r9) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 154; continue; } bb = 155; continue; }
            155 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 173; continue; } bb = 156; continue; }
            156 => { rsi = (rbx) as u64; r13 = ((rax as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; bb = 157; continue; }
            157 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x70u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x68u64))) as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r15.wrapping_add(rsi))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 159; continue; } bb = 158; continue; }
            158 => { crate::simd_ops::lk_add_u32(rdi, (1) as u32); r12 = ((crate::simd_ops::ru16(r15.wrapping_add(rsi).wrapping_add(0x8u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; bb = 160; continue; }
            159 => { r12 = ((crate::simd_ops::ru16(r15.wrapping_add(rsi).wrapping_add(0x8u64))) as u32) as u32 as u64; bb = 160; continue; }
            160 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rdi) as u64); crate::simd_ops::wu16(rbp.wrapping_sub(0x11u64), ((r12 as u16)) as u16); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 163; continue; } bb = 161; continue; }
            161 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 163; continue; } bb = 162; continue; }
            162 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 163; continue; }
            163 => { xmm0 = ((crate::simd_ops::ru64(r15.wrapping_add(rsi).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r15.wrapping_add(rsi).wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 165; continue; } bb = 164; continue; }
            164 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; bb = 165; continue; }
            165 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(r15))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 168; continue; } bb = 166; continue; }
            166 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 168; continue; } bb = 167; continue; }
            167 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 168; continue; }
            168 => { crate::simd_ops::wu64(rsi.wrapping_add(r15), (rdi) as u64); crate::simd_ops::wu16(rsi.wrapping_add(r15).wrapping_add(0x8u64), ((r12 as u16)) as u16); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64) as u128; crate::simd_ops::wu64(rsi.wrapping_add(r15).wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x1u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(r15).wrapping_add(0x18u64), ((rax as u32)) as u32); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 171; continue; } bb = 169; continue; }
            169 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 171; continue; } bb = 170; continue; }
            170 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 171; continue; }
            171 => { rsi = ((rsi).wrapping_add(0x20u64)) as u64; fl_l = (r13) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r13 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 157; continue; } bb = 172; continue; }
            172 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; bb = 173; continue; }
            173 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 184; continue; } bb = 174; continue; }
            174 => { rsi = (rbx) as u64; r12 = ((rax as u32)) as u32 as u64; bb = 175; continue; }
            175 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x88u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x80u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rsi.wrapping_add(rcx))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 177; continue; } bb = 176; continue; }
            176 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 177; continue; }
            177 => { crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rdi) as u64); rax = (crate::simd_ops::ru64(r13)) as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 179; continue; } bb = 178; continue; }
            178 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 179; continue; }
            179 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(r15))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 181; continue; } bb = 180; continue; }
            180 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 181; continue; }
            181 => { crate::simd_ops::wu64(rsi.wrapping_add(r15), (rdi) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 183; continue; } bb = 182; continue; }
            182 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 183; continue; }
            183 => { rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = (r12) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 175; continue; } bb = 184; continue; }
            184 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x69u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 195; continue; } bb = 185; continue; }
            185 => { r15 = ((rax as u32)) as u32 as u64; bb = 186; continue; }
            186 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0xa0u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x98u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rbx.wrapping_add(rcx))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 188; continue; } bb = 187; continue; }
            187 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 188; continue; }
            188 => { crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rdi) as u64); rax = (crate::simd_ops::ru64(r13)) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 190; continue; } bb = 189; continue; }
            189 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 190; continue; }
            190 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(rsi))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 192; continue; } bb = 191; continue; }
            191 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 192; continue; }
            192 => { crate::simd_ops::wu64(rbx.wrapping_add(rsi), (rdi) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 194; continue; } bb = 193; continue; }
            193 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 194; continue; }
            194 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = (r15) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 186; continue; } bb = 195; continue; }
            195 => { rsp = ((rsp).wrapping_add(0xa8u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a41b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416a7cb0; const P1: u64 = 0x1416a7100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rsi = ((r9 as u32)) as u32 as u64; rdi = r8; r14 = rdx; rbx = rcx; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), (0xffffffffu32) as u32); bb = 3; continue; }
            3 => { rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rax as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)))) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shr(4u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 8; continue; }
            7 => { rdx = ((rbp as u32)) as u32 as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 54; continue; } bb = 9; continue; }
            9 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), (0xffffffffu32) as u32); bb = 12; continue; }
            12 => { rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rax as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)))) as u32 as u64; rax = (0xcccccccccccccccdu64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; fl_l = ((rdx).wrapping_shr(5u32)) as u64; rdx = ((fl_l as u64)) as u64; bb = 17; continue; }
            16 => { rdx = ((rbp as u32)) as u32 as u64; bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 54; continue; } bb = 18; continue; }
            18 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), (0xffffffffu32) as u32); bb = 21; continue; }
            21 => { rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 22; continue; }
            22 => { rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rax as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)))) as u32 as u64; rax = (0x2492492492492493u64) as u64; let prod = (rax as u128).wrapping_mul((r8) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; r8 = ((r8).wrapping_shr(1u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; fl_l = ((r8).wrapping_shr(4u32)) as u64; r8 = ((fl_l as u64)) as u64; bb = 26; continue; }
            25 => { r8 = ((rbp as u32)) as u32 as u64; bb = 26; continue; }
            26 => { crate::simd_ops::wu32(rbx.wrapping_add(0x64u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 54; continue; } bb = 27; continue; }
            27 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x70u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x70u64), (0xffffffffu32) as u32); bb = 30; continue; }
            30 => { rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 31; continue; }
            31 => { rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x70u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rax as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x70u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)))) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shr(5u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 35; continue; }
            34 => { rdx = ((rbp as u32)) as u32 as u64; bb = 35; continue; }
            35 => { crate::simd_ops::wu32(rbx.wrapping_add(0x7cu64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 54; continue; } bb = 36; continue; }
            36 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 45; continue; } bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x88u64), (0xffffffffu32) as u32); bb = 39; continue; }
            39 => { rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 43; continue; } bb = 40; continue; }
            40 => { rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x88u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 42; continue; } bb = 41; continue; }
            41 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rax as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)))) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 44; continue; }
            43 => { rdx = ((rbp as u32)) as u32 as u64; bb = 44; continue; }
            44 => { crate::simd_ops::wu32(rbx.wrapping_add(0x94u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x90u64), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 54; continue; } bb = 45; continue; }
            45 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 55; continue; } bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0xa0u64), (0xffffffffu32) as u32); bb = 48; continue; }
            48 => { rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 52; continue; } bb = 49; continue; }
            49 => { rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0xa0u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x18u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 51; continue; } bb = 50; continue; }
            50 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rax as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 51; continue; }
            51 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)))) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 53; continue; }
            52 => { rdx = ((rbp as u32)) as u32 as u64; bb = 53; continue; }
            53 => { crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xa8u64), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 55; continue; } bb = 54; continue; }
            54 => { crate::simd_ops::wu8(r14, ((rbp as u8)) as u8); bb = 56; continue; }
            55 => { crate::simd_ops::wu8(r14, (0x1u8) as u8); bb = 56; continue; }
            56 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = r14; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a4570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x58; const P2: u64 = 0x48; const P3: u64 = 0x38; const P4: u64 = 0x28; const P5: u64 = 0x18; const P6: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(P1)))) as u32 as u64;
    rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(P2)))) as u32 as u64;
    rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(P3)))) as u32 as u64;
    rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(P4)))) as u32 as u64;
    rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(P5)))) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(P6)))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a4590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1433e16f0; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x4c; const P11: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x1u32) as u32);
    rax = P1;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdx as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rdx);
    crate::simd_ops::wu64(rcx, rax);
    rax = 0xffff;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P8), (0xfu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P9), (0x7fffffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P10), (0x7fffffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P11), (0x7fffffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(0x54u64), (0x7fffffffu32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), rdx);
    crate::simd_ops::wu16(rcx.wrapping_add(0x60u64), ((rax as u16)) as u16);
    crate::simd_ops::wu32(rcx.wrapping_add(0x68u64), (0x7f7fffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(0x6cu64), (0x7f7fffffu32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(0x70u64), (0x7f7fffffu32) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rdx);
    crate::simd_ops::wu16(rcx.wrapping_add(0x80u64), ((rax as u16)) as u16);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(0x88u64), ((rdx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a4620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x28; const P2: u64 = 0x68; const P3: u64 = 0x6c; const P4: u64 = 0x70; const P5: u64 = 0xffffffffffffffff; const P6: u64 = 0x48; const P7: u64 = 0x4c; const P8: u64 = 0x50; const P9: u64 = 0x54; const P10: u64 = 0x78; const P11: u64 = 0x143932190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx); crate::simd_ops::wu64(rcx.wrapping_add(P1), rsi); rbx = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P4), (0x7f7fffffu32) as u32); rdi = ((rsi.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P6), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P7), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P8), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P9), (0x7fffffffu32) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P10))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P11)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), rsi); rbp = 0xffff; crate::simd_ops::wu16(rbx.wrapping_add(0x80u64), ((rbp as u16)) as u16); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P11)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rsi); crate::simd_ops::wu16(rbx.wrapping_add(0x60u64), ((rbp as u16)) as u16); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x88u64), ((rsi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a46f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1416a4cb0; const P2: u64 = 0x143932190;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r15 = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; r13 = ((crate::simd_ops::ru16(rcx.wrapping_add(0x60u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rbx); rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(0x60u64))) as u32) as u32 as u64; rax = (((rdx as u16)) as u32) as u32 as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add((rax as u16))) as u16 as u64)); rdx = (rdx & !0xffffu64) | (((((rdx as u16)).wrapping_add((rax as u16))) as u16 as u64)); crate::simd_ops::wu16(rbp.wrapping_sub(0x28u64), ((rdx as u16)) as u16); r14 = 0xffff; crate::simd_ops::wu16(rbp.wrapping_sub(0x26u64), ((r14 as u16)) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rbx); rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x18u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rbp.wrapping_sub(0x16u64), ((r14 as u16)) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rbx); rdx = (rdx & !0xffffu64) | (((((rdx as u16)).wrapping_add(0x2u16)) as u16 as u64)); crate::simd_ops::wu16(rbp.wrapping_sub(0x8u64), ((rdx as u16)) as u16); crate::simd_ops::wu16(rbp.wrapping_sub(0x6u64), ((r14 as u16)) as u16); r12 = 0; fl_l = 0; fl_r = 0; rax = rbp.wrapping_sub(0x30); crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rax); r11 = P0; r10 = 0x3fff; bb = 1; continue; }
            1 => { rax = ((((r12 as u32)) as i32).wrapping_mul((0xaaabu32) as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(17u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rcx = ((r12 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((r12 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(crate::simd_ops::ru32(r11.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x33e1568u64)))) as u32 as u64; rax = ((rcx.wrapping_add(0x2u64)) as u32) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(rax.wrapping_mul(8)).wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0xffff3fffu32) as u32); rsi = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x38u64))) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = (((rsi as u16)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(29u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x5; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rax = ((((rax as u32)) & ((r10 as u32)))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(0x60u64))) as u32) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rax = ((rdi.wrapping_add(rcx)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x68u64))) as u16 as u64; fl_r = ((r14 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 16; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(0x8u64))) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 16; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x48u64))) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; bb = 12; continue; }
            10 => { fl_l = ((((rax as u32)) & ((r10 as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu16(rbp.wrapping_sub(0x46u64), ((r14 as u16)) as u16); crate::simd_ops::wu16(rbp.wrapping_sub(0x48u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rdi); bb = 12; continue; }
            12 => { rsi = (((rax as u16)) as u32) as u32 as u64; r14 = ((((rsi as u32)) as i32).wrapping_mul((0xaaabu32) as i32) as u32) as u32 as u64; r14 = (((r14 as u32)).wrapping_shr(17u32)) as u32 as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 14; continue; }
            14 => { rdx = rbp.wrapping_sub(0x50); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 45; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rdi); rax = ((r14.wrapping_add(r14.wrapping_mul(2))) as u32) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; r11 = P0; rsi = (rsi & !0xffffu64) | (((((rsi as u16)).wrapping_sub(crate::simd_ops::ru16(r11.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x33e1568u64)))) as u16 as u64)); fl_l = (((rsi as u16)).wrapping_add(0x2u16)) as u16 as u64; fl_r = fl_l; rsi = (rsi & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rbp.wrapping_sub(0x38u64), ((rsi as u16)) as u16); r14 = 0xffff; crate::simd_ops::wu16(rbp.wrapping_sub(0x36u64), ((r14 as u16)) as u16); r10 = 0x3fff; bb = 2; continue; }
            16 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r9)) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = ((crate::simd_ops::ru16(r9.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(29u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 39; continue; } bb = 17; continue; }
            17 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 18; continue; }
            18 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 19; continue; }
            19 => { r8 = ((((r8 as u32)) & ((r10 as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru16(rsi.wrapping_add(0x60u64))) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rsi)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x68u64))) as u16 as u64; fl_r = ((r14 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 39; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0x8u64))) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 39; continue; } bb = 23; continue; }
            23 => { r8 = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x38u64))) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; bb = 25; continue; }
            24 => { fl_l = ((((r8 as u32)) & ((r10 as u32)))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rsi); crate::simd_ops::wu16(rbp.wrapping_sub(0x38u64), ((r8 as u16)) as u16); crate::simd_ops::wu16(rbp.wrapping_sub(0x36u64), ((r14 as u16)) as u16); bb = 25; continue; }
            25 => { rdx = (((r8 as u16)) as u32) as u32 as u64; rax = ((((rdx as u32)) as i32).wrapping_mul((0xaaabu32) as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(17u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xfffffffeu32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add(crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x33e1600u64)))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 26; continue; }
            26 => { crate::simd_ops::wu16(rbp.wrapping_sub(0x46u64), ((r14 as u16)) as u16); crate::simd_ops::wu16(rbp.wrapping_sub(0x48u64), ((rdx as u16)) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rsi); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = (((rdx as u16)) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(29u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 38; continue; } bb = 27; continue; }
            27 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 28; continue; }
            28 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 29; continue; }
            29 => { rdi = ((((rdi as u32)) & ((r10 as u32)))) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((crate::simd_ops::ru16(rsi.wrapping_add(0x60u64))) as u32) as u32 as u64; rax = ((rax).wrapping_add(rsi)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x68u64))) as u16 as u64; fl_r = ((r14 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 38; continue; } bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0x8u64))) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 38; continue; } bb = 33; continue; }
            33 => { rdi = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x38u64))) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; bb = 35; continue; }
            34 => { fl_l = ((((rdi as u32)) & ((r10 as u32)))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x38u64), ((rdi as u16)) as u16); crate::simd_ops::wu16(rbp.wrapping_sub(0x36u64), ((r14 as u16)) as u16); bb = 35; continue; }
            35 => { rdx = rbp.wrapping_sub(0x40); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 56; continue; } bb = 36; continue; }
            36 => { rdx = (((rdi as u16)) as u32) as u32 as u64; rax = ((((rdx as u32)) as i32).wrapping_mul((0xaaabu32) as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(17u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; r11 = P0; rdx = (rdx & !0xffffu64) | (((((rdx as u16)).wrapping_sub(0x2u16)) as u16 as u64)); fl_l = (((rdx as u16)).wrapping_add(crate::simd_ops::ru16(r11.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x33e1600u64)))) as u16 as u64; fl_r = fl_l; rdx = (rdx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); r10 = 0x3fff; bb = 26; continue; }
            37 => { r14 = 0xffff; bb = 38; continue; }
            38 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; bb = 39; continue; }
            39 => { r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; r9 = ((r9).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), r9); fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 40; continue; }
            40 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x78u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 41; continue; }
            41 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 43; continue; }
            43 => { crate::simd_ops::wu64(r15.wrapping_add(0x78u64), (0x0u64) as u64); crate::simd_ops::wu16(r15.wrapping_add(0x80u64), ((r14 as u16)) as u16); crate::simd_ops::wu32(r15.wrapping_add(0x88u64), (0x3u32) as u32); bb = 44; continue; }
            44 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            45 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; bb = 47; continue; }
            47 => { crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; bb = 49; continue; }
            49 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x78u64))) as u64; rbx = 0xffffffff; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 50; continue; }
            50 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 52; continue; } bb = 51; continue; }
            51 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 52; continue; }
            52 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; bb = 53; continue; }
            53 => { crate::simd_ops::wu64(r15.wrapping_add(0x78u64), rdi); crate::simd_ops::wu16(r15.wrapping_add(0x80u64), ((r14 as u16)) as u16); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 54; continue; }
            54 => { let xa_src: u32 = (rbx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u32 as u64; rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 68; continue; } bb = 55; continue; }
            55 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; bb = 67; continue; }
            56 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 58; continue; } bb = 57; continue; }
            57 => { fl_l = (crate::simd_ops::lk_add_u32(rsi, (1) as u32)) as u32 as u64; rdi = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x38u64))) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; bb = 58; continue; }
            58 => { crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), rsi); rax = (((rdi as u16)) as u32) as u32 as u64; rdi = ((((rax as u32)) as i32).wrapping_mul((0xaaabu32) as i32) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_shr(17u32)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 59; continue; }
            59 => { fl_l = (crate::simd_ops::lk_add_u32(rsi, (1) as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; bb = 60; continue; }
            60 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x78u64))) as u64; rbx = 0xffffffff; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 61; continue; }
            61 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 62; continue; }
            62 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 63; continue; }
            63 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; bb = 64; continue; }
            64 => { crate::simd_ops::wu64(r15.wrapping_add(0x78u64), rsi); crate::simd_ops::wu16(r15.wrapping_add(0x80u64), ((rdi as u16)) as u16); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 65; continue; }
            65 => { let xa_src: u32 = (rbx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u32 as u64; rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 68; continue; } bb = 66; continue; }
            66 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; bb = 67; continue; }
            67 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 68; continue; }
            68 => { crate::simd_ops::wu32(r15.wrapping_add(0x88u64), (0x5u32) as u32); bb = 44; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a4bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x58; const P2: u64 = 0x143932190; const P3: u64 = 0x60; const P4: u64 = 0x8; const P5: u64 = 0x68; const P6: u64 = 0x70; const P7: u64 = 0x48; const P8: u64 = 0x4c; const P9: u64 = 0x50; const P10: u64 = 0x54; const P11: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32); rbp = r8; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = rdx; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = 0xffffffff; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rax = ((crate::simd_ops::ru16(rsi.wrapping_add(P4))) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(P3), ((rax as u16)) as u16); xmm0 = ((crate::simd_ops::ru64(rbp)) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P5), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P7), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P9), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P10), (0x7fffffffu32) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0xffff; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), (0x0u64) as u64); crate::simd_ops::wu16(rbx.wrapping_add(0x80u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbx.wrapping_add(0x88u64), (0x1u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a4cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416a8090;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = ((crate::simd_ops::ru16(rdx.wrapping_add(0x8u64))) as u32) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0xaaabu32) as i32) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x48u64))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(17u32)) as u32 as u64; rbx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4cu64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x3cu64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 2; continue; }
            2 => { r11 = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; r15 = rcx.wrapping_add(0x50); rbp = (crate::simd_ops::ru32(r15.wrapping_add(0x4u64))) as u32 as u64; rax = ((rbp as u32)) as u32 as u64; r14 = (crate::simd_ops::ru32(r15)) as u32 as u64; rcx = ((crate::simd_ops::ru8(r11.wrapping_add(0x33u64))) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(r11.wrapping_add(0x30u64))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(r11.wrapping_add(0x32u64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(r11.wrapping_add(0x31u64))) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_shl(7u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub((r14 as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(7u32)) as u32 as u64; r9 = (((((r9 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r14 as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(7u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(7u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r9 = ((((r9 as u32)) & ((rax as u32)))) as u32 as u64; r8 = (((((r8 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r9 = ((((r9 as u32)) & (0x1u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub(1)) as u32 as u64; rax = ((rbp as u32)) as u32 as u64; r9 = ((((r9 as u32)) & ((r8 as u32)))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rdx = (((crate::simd_ops::ru32(r11.wrapping_add(0x14u64))) as i32 as i64) as u64) as u64; rax = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; r8 = (((crate::simd_ops::ru32(r11.wrapping_add(0x1cu64))) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(r11)) as u64; r8 = ((r8).wrapping_add(r11)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); r9 = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; rax = ((rbx.wrapping_add(0x2u64)) as u32) as u32 as u64; r10 = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; rcx = ((rcx).wrapping_shr(14u32)) as u64; rcx = ((((rcx as u32)) & (0xfffu32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(14u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(14u32)) as u32 as u64; r9 = ((((r9 as u32)) & (0xfffu32))) as u32 as u64; r10 = ((((r10 as u32)) & (0xfffu32))) as u32 as u64; rdx = ((crate::simd_ops::ru8(r8.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0x1du64))) as u32) as u32 as u64; rbx = ((crate::simd_ops::ru8(r8.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0x1cu64))) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(7u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((rdx as u32)) as u32); rcx = ((rdx as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_shl(7u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rbx as u32)) as u32); rsi = ((crate::simd_ops::ru8(r8.wrapping_add(r9.wrapping_mul(2)).wrapping_add(0x1cu64))) as u32) as u32 as u64; r11 = ((crate::simd_ops::ru8(r8.wrapping_add(r9.wrapping_mul(2)).wrapping_add(0x1du64))) as u32) as u32 as u64; rsi = (((rsi as u32)).wrapping_shl(7u32)) as u32 as u64; r11 = (((r11 as u32)).wrapping_shl(7u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r11 as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rsi as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((r11 as u32)) as u32); rdi = ((crate::simd_ops::ru8(r8.wrapping_add(r10.wrapping_mul(2)).wrapping_add(0x1cu64))) as u32) as u32 as u64; r12 = ((crate::simd_ops::ru8(r8.wrapping_add(r10.wrapping_mul(2)).wrapping_add(0x1du64))) as u32) as u32 as u64; r10 = ((r11 as u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r10 = ((((r10 as u32)) & ((rax as u32)))) as u32 as u64; rdi = (((rdi as u32)).wrapping_shl(7u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_add((rdx as u32))) as u32 as u64; r12 = (((r12 as u32)).wrapping_shl(7u32)) as u32 as u64; rax = ((((rax as u32)) & ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rdi as u32)) as u32); r8 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((r12 as u32)) as u32); r11 = (((r11 as u32)).wrapping_add((rax as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rbx as u32))) as u32 as u64; r9 = ((r8 as u32)) as u32 as u64; r9 = (((((r9 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rdx = ((((rdx as u32)) & ((r9 as u32)))) as u32 as u64; r9 = ((((r9 as u32)) & ((r8 as u32)))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rbx as u32))) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rbx = ((rdx as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rbx = (((((rbx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rbx = ((((rbx as u32)) & ((rax as u32)))) as u32 as u64; rax = ((r12 as u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r10 as u32))) as u32 as u64; rdx = ((r10 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((r12 as u32))) as u32 as u64; rdx = (((((rdx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rdx = ((((rdx as u32)) & ((rax as u32)))) as u32 as u64; rax = ((r9.wrapping_add(rsi)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((r10 as u32))) as u32 as u64; rcx = ((((rcx as u32)) & ((rax as u32)))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rdi as u32))) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 5; continue; }
            5 => { r11 = (((r11 as u32)).wrapping_sub((r12 as u32))) as u32 as u64; rax = ((r11 as u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rax = ((((rax as u32)) & ((r11 as u32)))) as u32 as u64; rax = (((rax as u32)).wrapping_add((r12 as u32))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 7; continue; }
            7 => { r9 = rsp.wrapping_add(0x60); rcx = r15; r8 = rsp.wrapping_add(0x68); rdx = rsp.wrapping_add(0x70); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a4ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x88; const P1: u64 = 0x20; const P2: u64 = 0x68; const P3: u64 = 0x48; const P4: u64 = 0x6c; const P5: u64 = 0x1416b1580; const P6: u64 = 0x58; const P7: u64 = 0x60; const P8: u64 = 0x1416a46f0; const P9: u64 = 0x143932190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x1; rbx = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32) as u128; r8 = ((rbx.wrapping_add(P3)) as u64) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32) as u128; rdx = rsp.wrapping_add(0x30); rcx = ((rcx).wrapping_add(0x58u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = 0xffff; fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(P2))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(P7))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            7 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P9)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx.wrapping_add(P6), (0x0u64) as u64); bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x2u32) as u32); bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a4f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x20; const P2: u64 = 0x68; const P3: u64 = 0x60; const P4: u64 = 0x1416a46f0; const P5: u64 = 0x143932190; const P6: u64 = 0x88;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = 0xffff; fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(P2))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P3))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            5 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rbx.wrapping_add(P6), (0x2u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a5000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffe8; const P1: u64 = 0xffffffffffffff98; const P2: u64 = 0xffffffffffffffd8; const P3: u64 = 0xffffffffffffffa8; const P4: u64 = 0xffffffffffffffc8; const P5: u64 = 0xffffffffffffff88; const P6: u64 = 0xffffffffffffffb8; const P7: u64 = 0xffffffffffffff90; const P8: u64 = 0xffffffffffffffb0; const P9: u64 = 0xffffffffffffffa0; const P10: u64 = 0x1416a5240; const P11: u64 = 0x4;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; crate::simd_ops::wu128(rax.wrapping_add(P0), (xmm6) as u128); r9 = ((rax.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu128(rax.wrapping_add(P2), (xmm7) as u128); r8 = ((rax.wrapping_add(P3)) as u64) as u64; rbx = rdx; crate::simd_ops::wu128(rax.wrapping_add(P4), (xmm8) as u128); rdx = ((rax.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu128(rax.wrapping_add(P6), (xmm10) as u128); crate::simd_ops::wu64(rax.wrapping_add(P5), (0x0u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(P7), (0x0u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P3), (0x0u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(P8), (0x0u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P1), (0x0u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(P9), (0x0u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32) as u128; xmm6 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x28u64)))).to_bits() as u128); xmm10 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; xmm8 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32) as u128; xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm5 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(crate::simd_ops::ru32(rbx))).to_bits() as u128); xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(P11)))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x28u64)))).to_bits() as u128); xmm0 = (xmm6) as u128; xmm4 = (xmm2) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm5) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) / f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x28u64)))).to_bits() as u128); xmm0 = (xmm4) as u128; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a5130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbx = (crate::simd_ops::ru64(rcx)) as u64; r12 = r9; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x8u64))) as u32) as u32 as u64; r14 = rdx; r15 = r8; r10 = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rsi = (((crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as i32 as i64) as u64) as u64; r8 = (((crate::simd_ops::ru32(rbx.wrapping_add(0x3cu64))) as i32 as i64) as u64) as u64; rbp = (((crate::simd_ops::ru32(r10.wrapping_add(0x1cu64))) as i32 as i64) as u64) as u64; r11 = (((crate::simd_ops::ru32(r10.wrapping_add(0x14u64))) as i32 as i64) as u64) as u64; rbp = ((rbp).wrapping_add(r10)) as u64; r11 = ((r11).wrapping_add(r10)) as u64; r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rax = ((r10.wrapping_add(0x1u64)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru32(r11.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; rax = ((r10.wrapping_add(0x2u64)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru32(r11.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = (((rdi as u32)).wrapping_shr(14u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_shr(14u32)) as u32 as u64; rdi = ((((rdi as u32)) & (0xfffu32))) as u32 as u64; r9 = ((((r9 as u32)) & (0xfffu32))) as u32 as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x74u64))) as i32 as i64) as u64) as u64; rsi = (((rsi) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r8 = (((r8) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(r11.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; rcx = ((rcx).wrapping_shr(14u32)) as u64; rcx = ((((rcx as u32)) & (0xfffu32))) as u32 as u64; rax = ((crate::simd_ops::ru8(rbp.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0x1cu64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rbp.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0x1du64))) as u32) as u32 as u64; rax = ((rax).wrapping_shl(7u32)) as u64; rax = ((rax).wrapping_add(rsi)) as u64; rdx = ((rdx).wrapping_shl(7u32)) as u64; crate::simd_ops::wu64(r14, rax); rdx = ((rdx).wrapping_add(r8)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdx); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(r9.wrapping_mul(2)).wrapping_add(0x1cu64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(r9.wrapping_mul(2)).wrapping_add(0x1du64))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(7u32)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rax = ((rax).wrapping_shl(7u32)) as u64; rax = ((rax).wrapping_add(rsi)) as u64; crate::simd_ops::wu64(r15, rax); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rcx); rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0x1du64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0x1cu64))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(7u32)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rax = ((rax).wrapping_shl(7u32)) as u64; fl_l = ((rax).wrapping_add(rsi)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r12, rax); crate::simd_ops::wu64(r12.wrapping_add(0x8u64), rcx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a5240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); r10 = (crate::simd_ops::ru64(rcx)) as u64; xmm0 = 0u128; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x8u64))) as u32) as u32 as u64; r12 = rdx; r14 = (crate::simd_ops::ru64(r10.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; r11 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r13 = (((crate::simd_ops::ru32(r10.wrapping_add(0x38u64))) as i32 as i64) as u64) as u64; rcx = (((crate::simd_ops::ru32(r10.wrapping_add(0x3cu64))) as i32 as i64) as u64) as u64; rbx = (((crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as i32 as i64) as u64) as u64; rax = (((crate::simd_ops::ru32(rsi.wrapping_add(0x74u64))) as i32 as i64) as u64) as u64; rbx = ((rbx).wrapping_add(r14)) as u64; xmm3 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x64u64))) as u32) as u128; r15 = (((crate::simd_ops::ru32(r14.wrapping_add(0x1cu64))) as i32 as i64) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((rax) as i64) as u64) as u64; r13 = (((r13) as i64).wrapping_mul((rax) as i64) as u64) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; r15 = ((r15).wrapping_add(r14)) as u64; rdx = ((rdx).wrapping_shr(14u32)) as u64; rax = ((r11.wrapping_add(0x1u64)) as u32) as u32 as u64; r10 = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; rdx = ((((rdx as u32)) & (0xfffu32))) as u32 as u64; r10 = ((r10).wrapping_shr(14u32)) as u64; rax = ((r11.wrapping_add(0x2u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; r10 = ((((r10 as u32)) & (0xfffu32))) as u32 as u64; rbx = ((rbx).wrapping_shr(14u32)) as u64; r8 = ((crate::simd_ops::ru8(r15.wrapping_add(rdx.wrapping_mul(2)).wrapping_add(0x1du64))) as u32) as u32 as u64; rbx = ((((rbx as u32)) & (0xfffu32))) as u32 as u64; rax = ((crate::simd_ops::ru8(r15.wrapping_add(rdx.wrapping_mul(2)).wrapping_add(0x1cu64))) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(r15.wrapping_add(r10.wrapping_mul(2)).wrapping_add(0x1cu64))) as u32) as u32 as u64; rdi = ((crate::simd_ops::ru8(r15.wrapping_add(r10.wrapping_mul(2)).wrapping_add(0x1du64))) as u32) as u32 as u64; r11 = ((crate::simd_ops::ru8(r15.wrapping_add(rbx.wrapping_mul(2)).wrapping_add(0x1cu64))) as u32) as u32 as u64; rbp = ((crate::simd_ops::ru8(r15.wrapping_add(rbx.wrapping_mul(2)).wrapping_add(0x1du64))) as u32) as u32 as u64; rax = ((rax).wrapping_shl(7u32)) as u64; rax = ((rax).wrapping_add(r13)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rcx = (((crate::simd_ops::ru32(r14.wrapping_add(0x24u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((r8 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rax).wrapping_shl(7u32)) as u64; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x24u64))) as u32) as u128; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); rax = ((rax).wrapping_add(rdx)) as u64; crate::simd_ops::wu32(r12.wrapping_add(0x8u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(r12, ((xmm0 as u32)) as u32); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((r9 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(7u32)) as u64; rax = ((rax).wrapping_add(r13)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(r12.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); xmm0 = 0u128; xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x24u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((rdi as u32)) as u32 as u64; rax = ((rax).wrapping_shl(7u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((xmm2 as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(r8, ((xmm0 as u32)) as u32); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((r11 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(7u32)) as u64; rax = ((rax).wrapping_add(r13)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); xmm0 = 0u128; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0x24u64))) as u32) as u128; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((rbp as u32)) as u32 as u64; rax = ((rax).wrapping_shl(7u32)) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((xmm1 as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a53f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416c3c60; const P1: u64 = 0x1416c1af0; const P2: u64 = 0x141721400; const P3: u64 = 0x1416b6570; const P4: u64 = 0x1416da260; const P5: u64 = 0x140f35bf0; const P6: u64 = 0x1416daf20; const P7: u64 = 0x143932190; const P8: u64 = 0x14169edb0; const P9: u64 = 0x1416c3e50; const P10: u64 = 0x1411ff9f0; const P11: u64 = 0x1416c31e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = rcx; crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), (0x41f00000u32) as u32); crate::simd_ops::wu32(rcx, (0x40800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), (0x41f00000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x41f00000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), (0x3f000000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x41200000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), (0x3c23d70au32) as u32); r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0xb0u64), r13); r14 = rcx.wrapping_add(0xb8); crate::simd_ops::wu64(r14, r13); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r13); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), r13); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), r13); crate::simd_ops::wu64(r14.wrapping_add(0x20u64), r13); crate::simd_ops::wu64(r14.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(r14.wrapping_add(0x30u64), r13); crate::simd_ops::wu64(r14.wrapping_add(0x38u64), r13); crate::simd_ops::wu64(r14.wrapping_add(0x40u64), r13); rcx = ((rcx).wrapping_add(0x100u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rdi.wrapping_add(0x1b8); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbp); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), r13); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((r13 as u8)) as u8); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x88u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0x98u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0xa0u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0xa8u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0xb8u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0xc8u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0xd8u64), r13); crate::simd_ops::wu64(rbp.wrapping_add(0xe0u64), r13); crate::simd_ops::wu32(rbp.wrapping_add(0x130u64), ((r13 as u32)) as u32); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rdi.wrapping_add(0x2f8); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rsi); crate::simd_ops::wu64(rsi, r13); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x40800000u32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), (0x41f00000u32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), (0x41f00000u32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x14u64), (0x3f000000u32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), (0x41200000u32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), (0x3c23d70au32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), r13); crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), r13); crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), r13); crate::simd_ops::wu64(rsi.wrapping_add(0x50u64), r13); crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), r13); crate::simd_ops::wu64(rsi.wrapping_add(0x60u64), r13); crate::simd_ops::wu64(rsi.wrapping_add(0x68u64), r13); crate::simd_ops::wu64(rsi.wrapping_add(0x70u64), r13); crate::simd_ops::wu64(rsi.wrapping_add(0x78u64), r13); crate::simd_ops::wu64(rsi.wrapping_add(0x80u64), r13); crate::simd_ops::wu32(rsi.wrapping_add(0x88u64), ((r13 as u32)) as u32); rcx = rsi.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rcx); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x34u64), r13); crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), ((r13 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), r13); rbx = 0xffff; crate::simd_ops::wu16(rcx.wrapping_add(0x48u64), ((rbx as u16)) as u16); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0xe0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x348u64), r13); crate::simd_ops::wu16(rsi.wrapping_add(0x350u64), ((rbx as u16)) as u16); crate::simd_ops::wu32(rsi.wrapping_add(0x358u64), ((r13 as u32)) as u32); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0x658u64), ((r13 as u32)) as u32); rcx = rdi.wrapping_add(0x65c); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x684u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x68cu64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x694u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x69cu64), (0xffffffffffffffffu64) as u64); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x65cu64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x66cu64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x67cu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), r13); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), r13); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x78u64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P7)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), r13); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), r13); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x78u64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P7)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), r13); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), r13); crate::simd_ops::wu32(rdi.wrapping_add(0x78u64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 11; continue; }
            11 => { fl_l = rax; fl_r = (0xfffffffffffffffeu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(P7)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), r13); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), r13); bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), r13); rcx = rdi.wrapping_add(0x100); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x88); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xb0u64))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0xa8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 22; continue; } bb = 17; continue; }
            17 => { fl_l = rax; fl_r = (0xfffffffffffffffeu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(P7)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), r13); bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rdi.wrapping_add(0xb0u64), r13); bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), r13); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a5820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x1416a71e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rdi); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); crate::simd_ops::wu32(rcx.wrapping_add(0x38u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rdi); crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), rdi); crate::simd_ops::wu32(rcx.wrapping_add(0x68u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rdi); crate::simd_ops::wu32(rcx.wrapping_add(0x80u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), rdi); crate::simd_ops::wu32(rcx.wrapping_add(0x98u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0xb0u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0xb8u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0xc0u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0xc8u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0xd0u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0xd8u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0xe0u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0xe8u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0xf0u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0xf8u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x100u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x108u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x110u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x118u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x120u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x128u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x130u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x138u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x140u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x148u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x150u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x158u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x160u64), rdi); rax = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x168u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x170u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x178u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x180u64), rax); rax = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x188u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x190u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x198u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x1a0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x1a8u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x1b0u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x1b8u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x1c0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x1c8u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x1d0u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x1d8u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x1e0u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x1e8u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x1f0u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x1f8u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x200u64), rax); rax = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x208u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x210u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x218u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x220u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x228u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x230u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x238u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x240u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x248u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x250u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x258u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x260u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x268u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x270u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x278u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x280u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x288u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(P0)) as u64; rcx = ((rcx).wrapping_add(0x290u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); crate::simd_ops::wu32(rcx, (0xffffffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xacu64), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xa8u64), (0x7f7fffffu32) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x340u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x348u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x350u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x358u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x360u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x368u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x370u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x378u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x380u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x388u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x390u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x398u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x3a0u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x3a8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x3b0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x3b8u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x3c0u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x3c8u64), rdi); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a5b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426aa748; const P1: u64 = 0x1416a78a0; const P2: u64 = 0x143932190; const P3: u64 = 0x1426aa760;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = rcx; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rbp); rax = P0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rbp); crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x24u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), rbp); crate::simd_ops::wu32(rcx.wrapping_add(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x34u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rbp); crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x44u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rbp); crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x54u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), rbp); crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x64u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rbp); crate::simd_ops::wu32(rcx.wrapping_add(0x70u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x74u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rbp); crate::simd_ops::wu32(rcx.wrapping_add(0x80u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x84u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), rbp); crate::simd_ops::wu32(rcx.wrapping_add(0x90u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x94u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), rbp); crate::simd_ops::wu32(rcx.wrapping_add(0xa0u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xa4u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), rbp); crate::simd_ops::wu32(rcx.wrapping_add(0xb0u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xb4u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xb8u64), rbp); crate::simd_ops::wu32(rcx.wrapping_add(0xc0u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xc4u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xc8u64), rbp); crate::simd_ops::wu32(rcx.wrapping_add(0xd0u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xd4u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xd8u64), rbp); crate::simd_ops::wu32(rcx.wrapping_add(0xe0u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xe4u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xe8u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0xf0u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0xf8u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x100u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x108u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x110u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x118u64), rbp); crate::simd_ops::wu64(rcx.wrapping_add(0x120u64), rbp); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x10u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x4000u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsp.wrapping_add(0x20); rdx = P3; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); r8 = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rsi as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = rbx; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a5d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x143932190; const P3: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rbx = ((rdi.wrapping_add(P1)) as u64) as u64; rbx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rbx = ((rdx.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_sub(0x18u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a5dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x1416a7720; const P3: u64 = 0x143932190; const P4: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rbx = ((rdi.wrapping_add(P1)) as u64) as u64; rbx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rbx = ((rdx.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_sub(0x18u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a5e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143932190; const P2: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rbx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(0xffffffffffffffe0u64)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx).wrapping_add(rdx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_sub(0x30u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a5ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffe0; const P2: u64 = 0x143932190; const P3: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = rdi; rax = ((rax).wrapping_shl(5u32)) as u64; rbx = ((rdx.wrapping_add(P1)) as u64) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_sub(0x20u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a5f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffc0; const P2: u64 = 0x143932190; const P3: u64 = 0x20; const P4: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = rdi; rax = ((rax).wrapping_shl(6u32)) as u64; rbx = ((rdx.wrapping_add(P1)) as u64) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx.wrapping_add(P1)) as u64) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a5fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffd0; const P2: u64 = 0x18; const P3: u64 = 0x143932190; const P4: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rbx = ((rdx.wrapping_add(P1)) as u64) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_sub(0x30u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; bb = 8; continue; }
            8 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a60f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412026f0; const P1: u64 = 0x143932190; const P2: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a6120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141201db0; const P1: u64 = 0x1416a67b0; const P2: u64 = 0x1416a6930; const P3: u64 = 0x1416c3cd0; const P4: u64 = 0x143932190; const P5: u64 = 0x1412026f0; const P6: u64 = 0x14169edb0; const P7: u64 = 0x1416daf20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x2f8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x1b8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x100); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd0u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi.wrapping_add(0xb8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xb8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi.wrapping_add(0xa0); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x78u64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x78u64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a62c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x208; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x1f0; const P4: u64 = 0x1d8; const P5: u64 = 0x150; const P6: u64 = 0x1416cf3d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a6350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x88; const P2: u64 = 0x60; const P3: u64 = 0x68; const P4: u64 = 0x48; const P5: u64 = 0x28; const P6: u64 = 0x8;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a63c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416a7b20; const P1: u64 = 0x1416a5fe0; const P2: u64 = 0x1416a6350; const P3: u64 = 0x1417218a0; const P4: u64 = 0x143932190; const P5: u64 = 0x1416a5e60; const P6: u64 = 0x1416a5f60; const P7: u64 = 0x1416a7790; const P8: u64 = 0x1416a5dd0; const P9: u64 = 0x1416a5d50; const P10: u64 = 0x1416a5ee0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x3b8u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x3c0u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x3b8u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x3c0u64), (0xffffffffu32) as u32); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x3a0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x3a8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x3a0u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x3a8u64), (0xffffffffu32) as u32); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x388u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x390u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x388u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x390u64), (0xffffffffu32) as u32); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x370u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x378u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x370u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x378u64), (0xffffffffu32) as u32); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x358u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x360u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x358u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x360u64), (0xffffffffu32) as u32); bb = 10; continue; }
            10 => { rcx = rdi.wrapping_add(0x340); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x290); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x208); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x268u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x248u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x228); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x208u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x1e8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1c8u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1a8u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x188u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x168u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x158u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x150u64))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x150u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = rdi.wrapping_add(0x138); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x120); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x108u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf0u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rcx = rdi.wrapping_add(0xc0); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xa8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x98u64), (0xffffffffu32) as u32); bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x80u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x80u64), (0xffffffffu32) as u32); bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), (0xffffffffu32) as u32); bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x50u64), (0xffffffffu32) as u32); bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), (0xffffffffu32) as u32); bb = 30; continue; }
            30 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a6700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1417218a0; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x40; const P4: u64 = 0x20; const P5: u64 = 0x1416a5e60;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rbx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a6760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416a5f60; const P1: u64 = 0x143932190; const P2: u64 = 0x40; const P3: u64 = 0x60; const P4: u64 = 0x20;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x60u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rbx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a67b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x348; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0xe0; const P4: u64 = 0x1416a62c0; const P5: u64 = 0xd0; const P6: u64 = 0xffffffffffffffff; const P7: u64 = 0xb0; const P8: u64 = 0x14169edb0; const P9: u64 = 0x40; const P10: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = 0xffffffff; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; rax = ((rdi.wrapping_add(P6)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a6870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x20; const P4: u64 = 0x14169edb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a68d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x18; const P4: u64 = 0x1412026f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a6a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416a6bc0; const P1: u64 = 0x1416a6e90; const P2: u64 = 0x1416a6d60; const P3: u64 = 0x143932190; const P4: u64 = 0x1416a6fd0; const P5: u64 = 0x1416a6120; const P6: u64 = 0x1416a63c0; const P7: u64 = 0x1416a33d0; const P8: u64 = 0x1426aa730;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0xe8u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), rbp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xf0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = 0x1; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), rbp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xf8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = 0x1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0xf8u64), rbp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rdx, rbp); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), (0xffffffffu32) as u32); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x100u64), rbp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x110u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = 0x1; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rbp); bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x108u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), rbp); bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x118u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x118u64), rbp); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 17; continue; }
            17 => { rdi = rbx.wrapping_add(0x18); rsi = 0xd; bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rbp); bb = 22; continue; }
            22 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x120u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x74u64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x74u64), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rax = P8; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a6bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0xc8u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xd0u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0xd0u64), (0xffffffffu32) as u32); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xb0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xb8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0xb8u64), (0xffffffffu32) as u32); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0xa0u64), (0xffffffffu32) as u32); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x88u64), (0xffffffffu32) as u32); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rsi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), (0xffffffffu32) as u32); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); crate::simd_ops::wu64(rbx, rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), (0xffffffffu32) as u32); bb = 14; continue; }
            14 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rdx = 0xe8; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a6d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141200ad0; const P1: u64 = 0x143932190; const P2: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; rdi = 0xffffffff; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rdx); crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), (0xffffffffu32) as u32); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdx); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), (0xffffffffu32) as u32); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((rdx as u8)) as u8); crate::simd_ops::wu64(rbx, rdx); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), (0xffffffffu32) as u32); bb = 15; continue; }
            15 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { rdx = 0xc0; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a6e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), r8); crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), (0xffffffffu32) as u32); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), r8); crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), (0xffffffffu32) as u32); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx, r8); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), (0xffffffffu32) as u32); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r8); crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), (0xffffffffu32) as u32); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), r8); crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), (0xffffffffu32) as u32); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), r8); crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), (0xffffffffu32) as u32); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r8); crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), (0xffffffffu32) as u32); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx, r8); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), (0xffffffffu32) as u32); bb = 16; continue; }
            16 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            19 => { rdx = 0x78; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a6fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx, r8); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), (0xffffffffu32) as u32); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), r8); crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), (0xffffffffu32) as u32); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), r8); crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), (0xffffffffu32) as u32); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x80u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), r8); crate::simd_ops::wu32(rbx.wrapping_add(0x80u64), (0xffffffffu32) as u32); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), r8); crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), (0xffffffffu32) as u32); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), r8); crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), (0xffffffffu32) as u32); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rbx, r8); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), (0xffffffffu32) as u32); bb = 14; continue; }
            14 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rdx = 0xa8; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r14 = rcx; rbp = ((rdx as u32)) as u32 as u64; r15 = rbp.wrapping_add(0x2); r15 = ((r15).wrapping_add(r15)) as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(r15.wrapping_mul(8)))) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((r10 as u32))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; r9 = ((r8 as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; let d = ((r10 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } r9 = (((r9 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((r9 as u32))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (((rsi as u32)).wrapping_add((r10 as u32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rsi as u32)) as u32 as u64; r12 = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = r14.wrapping_add(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); rdi = ((rax.wrapping_add(0xdu64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); r13 = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xcu64))) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, r13); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r13 as u32)) as u32); bb = 8; continue; }
            8 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(r14)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = r12; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rbp).wrapping_add(rbp)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; crate::simd_ops::wu32(r14.wrapping_add(r15.wrapping_mul(8)), ((rsi as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x18u64), rax); bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a71e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; rbx = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = (0xfffffffffffffffeu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rdi); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rdi); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rdi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = (0xfffffffffffffffeu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 18; continue; } bb = 13; continue; }
            13 => { fl_l = rax; fl_r = (0xfffffffffffffffeu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rdi); bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rdi); bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rdi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 24; continue; } bb = 19; continue; }
            19 => { fl_l = rax; fl_r = (0xfffffffffffffffeu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rdi); bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), rdi); bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rdi); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 30; continue; } bb = 25; continue; }
            25 => { fl_l = rax; fl_r = (0xfffffffffffffffeu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x88u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), rdi); bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rdi); bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), (0xff7fffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xa8u64), (0x7f7fffffu32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x1416c95f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0xe8; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x24u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x28u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x2cu64), (0x80000000u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x30u64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x38u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x48u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x58u64), rbx); crate::simd_ops::wu32(rax.wrapping_add(0x60u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x78u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x80u64), rbx); crate::simd_ops::wu32(rax.wrapping_add(0x88u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x90u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x98u64), rbx); crate::simd_ops::wu32(rax.wrapping_add(0xa0u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0xa8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0xb0u64), rbx); crate::simd_ops::wu32(rax.wrapping_add(0xb8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0xc0u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0xc8u64), rbx); crate::simd_ops::wu32(rax.wrapping_add(0xd0u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0xd8u64), rbx); bb = 3; continue; }
            2 => { rax = rbx; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(0xe8u64), rax); rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x78u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rbx); crate::simd_ops::wu32(rax.wrapping_add(0x30u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x38u64), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x40u64), rbx); crate::simd_ops::wu32(rax.wrapping_add(0x48u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x50u64), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x58u64), rbx); crate::simd_ops::wu32(rax.wrapping_add(0x60u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x68u64), (0x0u64) as u64); rcx = 0xffff; crate::simd_ops::wu16(rax.wrapping_add(0x70u64), ((rcx as u16)) as u16); bb = 6; continue; }
            5 => { rax = rbx; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi.wrapping_add(0xf0u64), rax); rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0xc0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            8 => { rax = rbx; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi.wrapping_add(0xf8u64), rax); rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x38u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x1cu64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x24u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x30u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x100u64), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { crate::simd_ops::wu64(rdi.wrapping_add(0x100u64), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x48; const P2: u64 = 0x1416a53f0; const P3: u64 = 0x108;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = 0x6a8;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rbx.wrapping_add(P3), (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a75e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x48; const P2: u64 = 0x1416a5820; const P3: u64 = 0x118;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = 0x3d0;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rbx.wrapping_add(P3), (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x48; const P2: u64 = 0x8; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x24; const P7: u64 = 0x2c; const P8: u64 = 0x30; const P9: u64 = 0x40; const P10: u64 = 0x50; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = 0xa8; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax, rdx); crate::simd_ops::wu32(rax.wrapping_add(P2), (0xffffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P3), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P4), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), (0x80000000u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P6), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(P7), (0xffffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P8), ((rdx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(P9), rdx); crate::simd_ops::wu32(rax.wrapping_add(P1), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P10), rdx); crate::simd_ops::wu32(rax.wrapping_add(P11), ((rdx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x60u64), rdx); crate::simd_ops::wu32(rax.wrapping_add(0x68u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x70u64), rdx); crate::simd_ops::wu64(rax.wrapping_add(0x78u64), rdx); crate::simd_ops::wu32(rax.wrapping_add(0x80u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x88u64), rdx); crate::simd_ops::wu32(rax.wrapping_add(0x90u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x94u64), (0x7fffffffu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x98u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x9cu64), (0xffffffff80000000u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(0xa4u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rdx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x30; const P2: u64 = 0x1416a5fe0; const P3: u64 = 0x18; const P4: u64 = 0x143932190; const P5: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = ((rdx.wrapping_add(P0)) as u64) as u64; rbx = ((rbx.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; rbx = ((rcx.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_sub(0x48u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 8; continue; }
            8 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x60; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0xffffffffffffffd0; const P4: u64 = 0xffffffffffffffb8; const P5: u64 = 0xffffffffffffffa0; const P6: u64 = 0xffffffffffffff88;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = (((rdx) as i64).wrapping_mul((0x98u64) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = ((rbx).wrapping_add(rcx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rbx = ((rbx).wrapping_sub(0x98u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 16; continue; }
            16 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1416a7970; const P2: u64 = 0x28; const P3: u64 = 0x8; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = ((rdx as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a78a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x24; const P3: u64 = 0x28; const P4: u64 = 0x30; const P5: u64 = 0x34; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x44; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x54;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P5), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); crate::simd_ops::wu32(rcx.wrapping_add(P7), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P8), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(P9), rax); crate::simd_ops::wu32(rcx.wrapping_add(P10), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P11), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x64u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x74u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x80u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x84u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x90u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x94u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xa0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xa4u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xb0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xb4u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xb8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xc0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xc4u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xc8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xd0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xd4u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xd8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xe0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xe4u64), ((rax as u8)) as u8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932190; const P1: u64 = 0x1426aa760; const P2: u64 = 0x18; const P3: u64 = 0x28; const P4: u64 = 0x8; const P5: u64 = 0x20;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64; rbx = rcx; rdi = ((rdx as u32)) as u32 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = r11.wrapping_sub(0x48); rax = 0; fl_l = 0; fl_r = 0; rdx = P1; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), (0x10u64) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), (0x4000u64) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), (0x4000u64) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), (0xffffffffffffffffu64) as u64); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); rdx = ((rdi as u32)) as u32 as u64; rcx = rax; r8 = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x143932190; const P2: u64 = 0x60; const P3: u64 = 0x20; const P4: u64 = 0x24; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x34; const P8: u64 = 0x38; const P9: u64 = 0x40; const P10: u64 = 0x44; const P11: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rsi = 0xd; bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(P5), rax); crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(P8), rax); crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P10), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(P11), rax); crate::simd_ops::wu32(rdi.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x54u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x64u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x74u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0x80u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x84u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x88u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0x90u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x94u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x98u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0xa0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0xa4u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0xb0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0xb4u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0xb8u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0xc0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0xc4u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0xc8u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0xd0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0xd4u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0xd8u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0xe0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0xe4u64), ((rax as u8)) as u8); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r8); crate::simd_ops::wu32(rcx.wrapping_add(0x38u64), (0xffffffffu32) as u32); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x50u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), r8); crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), (0xffffffffu32) as u32); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x68u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), r8); crate::simd_ops::wu32(rcx.wrapping_add(0x68u64), (0xffffffffu32) as u32); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x80u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), r8); crate::simd_ops::wu32(rcx.wrapping_add(0x80u64), (0xffffffffu32) as u32); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), r8); crate::simd_ops::wu32(rcx.wrapping_add(0x98u64), (0xffffffffu32) as u32); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x358u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x360u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x358u64), r8); crate::simd_ops::wu32(rcx.wrapping_add(0x360u64), (0xffffffffu32) as u32); bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x370u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x378u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x370u64), r8); crate::simd_ops::wu32(rcx.wrapping_add(0x378u64), (0xffffffffu32) as u32); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x388u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x390u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x388u64), r8); crate::simd_ops::wu32(rcx.wrapping_add(0x390u64), (0xffffffffu32) as u32); bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x3a0u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x3a8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x3a0u64), r8); crate::simd_ops::wu32(rcx.wrapping_add(0x3a8u64), (0xffffffffu32) as u32); bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x3b8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x3c0u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), ((r8 as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x3b8u64), r8); crate::simd_ops::wu32(rcx.wrapping_add(0x3c0u64), (0xffffffffu32) as u32); bb = 20; continue; }
            20 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = rcx; rdx = ((rcx.wrapping_add(P0)) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xd; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = 0xffffffff; return rax; }
            4 => { rcx = ((rax as u32)) as u32 as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu8(r8.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P0), (0x1u8) as u8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426d10; const P1: u64 = 0x1434268d0; const P2: u64 = 0x143426ff0; const P3: u64 = 0x143427080;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128; crate::simd_ops::wu64(rcx, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0xcu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((rax as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x4u64)))).to_bits() as u128); xmm4 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm7) as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x1cu64), ((xmm1 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((xmm4 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x4u64)))).to_bits() as u128); xmm6 = ((crate::simd_ops::ru32(r9)) as u32) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); crate::simd_ops::wu128(rsp, (xmm8) as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x24u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((xmm6 as u32)) as u32); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x28u64), ((xmm4 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x2cu64), ((xmm6 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32) as u128; xmm0 = (xmm1) as u128; xmm5 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x38u64), ((xmm3 as u32)) as u32); xmm1 = 0u128; fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x30u64), ((xmm5 as u32)) as u32); xmm7 = (xmm5) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x34u64), ((xmm7 as u32)) as u32); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), ((xmm7 as u32)) as u32); xmm8 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x24u64)))).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((xmm8 as u32)) as u32); if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 24; continue; } bb = 2; continue; }
            2 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 27; continue; } bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp)) as u128; crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), ((rax as u32)) as u32); rax = rcx; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 6; continue; }
            6 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 25; continue; } bb = 7; continue; }
            7 => { xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm7 = 0u128; bb = 10; continue; }
            9 => { xmm7 = (xmm2) as u128; bb = 10; continue; }
            10 => { xmm4 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm0 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm8 = ((crate::simd_ops::ru32(P3)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm0 = (xmm4) as u128; bb = 13; continue; }
            12 => { xmm0 = (xmm8) as u128; bb = 13; continue; }
            13 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm7 = (xmm2) as u128; bb = 16; continue; }
            15 => { xmm7 = 0u128; bb = 16; continue; }
            16 => { fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { xmm0 = 0u128; bb = 19; continue; }
            18 => { xmm0 = (xmm2) as u128; bb = 19; continue; }
            19 => { xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { xmm4 = (xmm8) as u128; bb = 21; continue; }
            21 => { xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { xmm2 = 0u128; bb = 23; continue; }
            23 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 3; continue; } bb = 24; continue; }
            24 => { rax = 0x2; bb = 4; continue; }
            25 => { fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 26; continue; }
            26 => { fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 27; continue; }
            27 => { rax = 0x1; bb = 4; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = (crate::simd_ops::ru32(rcx)) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; r14 = (crate::simd_ops::ru32(r8)) as u32 as u64; rax = (crate::simd_ops::ru32(r9)) as u32 as u64; rbp = (((rbp as u32)).wrapping_sub((rax as u32))) as u32 as u64; r10 = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; r14 = (((r14 as u32)).wrapping_sub((rax as u32))) as u32 as u64; rbx = (crate::simd_ops::ru32(rdx)) as u32 as u64; rdi = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rbx = (((rbx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((rax as u32))) as u32 as u64; rsi = (((rsi as u32)).wrapping_sub((rax as u32))) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub((rax as u32))) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rdi as u32)) as i32) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rbx as u32)) as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = ((r14 as u32)) as u32 as u64; r11 = ((((rcx as u32)) as i32 as i64) as u64) as u64; rcx = ((r10 as u32)) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((r10 as u32)) as i32) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((r14 as u32)) as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = ((rbp as u32)) as u32 as u64; r9 = ((((rcx as u32)) as i32 as i64) as u64) as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rsi as u32)) as i32) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rbp as u32)) as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((r14 as u32)) as i32) as u32) as u32 as u64; rdx = ((((rcx as u32)) as i32 as i64) as u64) as u64; rdi = ((((rdi as u32)) as i32).wrapping_mul(((rbp as u32)) as i32) as u32) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((r10 as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rbx as u32)) as i32) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; r10 = ((((r10 as u32)) as i32).wrapping_mul(((rbp as u32)) as i32) as u32) as u32 as u64; r8 = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((rsi as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rbx as u32)) as i32) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; r8 = (((r8) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rsi = ((((rsi as u32)) as i32).wrapping_mul(((r14 as u32)) as i32) as u32) as u32 as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = ((((rdi as u32)) as i32 as i64) as u64) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; rax = (((rax) as i64).wrapping_mul((r9) as i64) as u64) as u64; rcx = ((((r10 as u32)) as i32 as i64) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; rcx = ((((rsi as u32)) as i32 as i64) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a7fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = (crate::simd_ops::ru32(r9)) as u32 as u64; r11 = (crate::simd_ops::ru32(r8)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r11 as u32))) as u32 as u64; r10 = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdx)) as u32 as u64; r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rdx = (crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32 as u64; r9 = ((rbp as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub((r10 as u32))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r10 as u32))) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r11 as u32))) as u32 as u64; r11 = (((r11 as u32)).wrapping_sub((rsi as u32))) as u32 as u64; r9 = ((((r9 as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((rbx as u32))) as u32 as u64; rsi = (((rsi as u32)).wrapping_sub((rbx as u32))) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((r10 as u32)) as i32) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; r9 = (((r9 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; r9 = (((((r9 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((r11 as u32)) as i32) as u32) as u32 as u64; rbp = ((((rbp as u32)) as i32).wrapping_mul(((rsi as u32)) as i32) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; rdi = ((((rdi as u32)) as i32).wrapping_mul(((r8 as u32)) as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rbp = (((rbp as u32)).wrapping_sub((rdi as u32))) as u32 as u64; r9 = ((((r9 as u32)) & ((rcx as u32)))) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbp = (((((rbp as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a8090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); r10 = (crate::simd_ops::ru32(rdx)) as u32 as u64; r13 = ((r10 as u32)) as u32 as u64; rdi = (crate::simd_ops::ru32(r9)) as u32 as u64; r12 = ((rdi as u32)) as u32 as u64; r11 = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; r13 = (((r13 as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rbx = (crate::simd_ops::ru32(r8)) as u32 as u64; rsi = ((r11 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as u32 as u64; r12 = (((r12 as u32)).wrapping_sub((rbx as u32))) as u32 as u64; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; r9 = ((r8 as u32)) as u32 as u64; r15 = ((r14 as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_sub((r11 as u32))) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r10 as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; r15 = (((r15 as u32)).wrapping_sub((rax as u32))) as u32 as u64; r15 = ((((r15 as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as i32) as u32) as u32 as u64; rsi = (((rsi as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rbp = ((rax as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r10 as u32))) as u32 as u64; r10 = ((r14 as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rsi as u32)) as i32) as u32) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub((r11 as u32))) as u32 as u64; r11 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; r14 = (((r14 as u32)).wrapping_sub((r8 as u32))) as u32 as u64; r10 = ((((r10 as u32)) as i32).wrapping_mul(((r12 as u32)) as i32) as u32) as u32 as u64; rsi = (0u32.wrapping_sub((rsi as u32))) as u32 as u64; r14 = ((((r14 as u32)) as i32).wrapping_mul(((r13 as u32)) as i32) as u32) as u32 as u64; rsi = (((((rsi as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rbx as u32))) as u32 as u64; r8 = ((r15 as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((r9 as u32)) as i32) as u32) as u32 as u64; r15 = (0u32.wrapping_sub((r15 as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rdi as u32))) as u32 as u64; r15 = (((((r15 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r15 = ((((r15 as u32)) & (0x1u32))) as u32 as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul(((rbp as u32)) as i32) as u32) as u32 as u64; r9 = (0u32.wrapping_sub((r9 as u32))) as u32 as u64; r8 = (((((r8 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r9 = (((((r9 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r15 as u32))) as u32 as u64; rsi = ((((rsi as u32)) & ((rax as u32)))) as u32 as u64; r14 = (((r14 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = ((r10 as u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((r15 as u32))) as u32 as u64; r10 = (0u32.wrapping_sub((r10 as u32))) as u32 as u64; rdx = (((((rdx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r10 = (((((r10 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rcx = ((r14 as u32)) as u32 as u64; r10 = ((((r10 as u32)) & (0x1u32))) as u32 as u64; rcx = (((((rcx as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_sub((rsi as u32))) as u32 as u64; r14 = (0u32.wrapping_sub((r14 as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; r14 = (((((r14 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rax = ((r11 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r14 = ((((r14 as u32)) & (0x1u32))) as u32 as u64; r15 = ((((r15 as u32)) & ((rax as u32)))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r10 as u32))) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rsi as u32))) as u32 as u64; r9 = ((((r9 as u32)) & ((rax as u32)))) as u32 as u64; rax = ((r12 as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((r10 as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_sub((r9 as u32))) as u32 as u64; r10 = ((((r10 as u32)) & ((rax as u32)))) as u32 as u64; r10 = (((r10 as u32)).wrapping_add((r9 as u32))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (0u32.wrapping_sub((rbp as u32))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (0u32.wrapping_sub((r12 as u32))) as u32 as u64; rbp = (((((rbp as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r11 = (0u32.wrapping_sub((r11 as u32))) as u32 as u64; r12 = (((((r12 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r14 as u32))) as u32 as u64; r11 = (((((r11 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; rbp = ((((rbp as u32)) & ((rax as u32)))) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((r10 as u32))) as u32 as u64; rax = ((r13 as u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((r14 as u32))) as u32 as u64; r13 = (0u32.wrapping_sub((r13 as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r14 = (((r14 as u32)).wrapping_sub((rbp as u32))) as u32 as u64; r13 = (((((r13 as u32)) as i32).wrapping_shr(31u32) as u32)) as u32 as u64; r14 = ((((r14 as u32)) & ((rax as u32)))) as u32 as u64; r12 = ((((r12 as u32)) & ((rdx as u32)))) as u32 as u64; r14 = (((r14 as u32)).wrapping_add((rbp as u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r15 as u32))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (((rcx as u32)).wrapping_sub((r14 as u32))) as u32 as u64; r13 = ((((r13 as u32)) & ((rcx as u32)))) as u32 as u64; r12 = (((r12 as u32)).wrapping_add((r10 as u32))) as u32 as u64; r13 = (((r13 as u32)).wrapping_add((r14 as u32))) as u32 as u64; r11 = ((((r11 as u32)) & ((r8 as u32)))) as u32 as u64; r13 = ((((r13 as u32)) & ((r12 as u32)))) as u32 as u64; r11 = (((r11 as u32)).wrapping_add((r15 as u32))) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416a8220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1434268d0; const P2: u64 = 0xc; const P3: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128; xmm1 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm3 = (xmm2) as u128; bb = 3; continue; }
            2 => { xmm3 = 0u128; bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm5 = (xmm2) as u128; bb = 6; continue; }
            5 => { xmm5 = 0u128; bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P3)))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm4 = (xmm2) as u128; bb = 9; continue; }
            8 => { xmm4 = 0u128; bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P3)))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm2 = 0u128; bb = 11; continue; }
            11 => { xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}
