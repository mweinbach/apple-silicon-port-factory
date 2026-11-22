#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_140fa9ca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x108; const P4: u64 = 0x4; const P5: u64 = 0x100; const P6: u64 = 0xe; const P7: u64 = 0x20; const P8: u64 = 0x1425371a0; const P9: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rax.wrapping_add(P1), rcx); rcx = (tmp_xchg) as u64; rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rcx = ((r9.wrapping_add(P2)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P1), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P7), r9); if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = rdx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            8 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            9 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140fbd4b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x1401e84a0; const P4: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rdi = rdx; rbx = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbx = ((rbx).wrapping_sub(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); r14 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, r14); crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); rbp = ((rbx.wrapping_add(rsi)) as u64) as u64; crate::simd_ops::wu32(rsi.wrapping_add(P1), ((rax as u32)) as u32); rdx = rbp; rcx = rsi; crate::simd_ops::wu8(rsi.wrapping_add(P2), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(P4), ((rax as u8)) as u8); rsi = ((rsi).wrapping_add(0x20u64)) as u64; rax = ((rbx.wrapping_add(rsi)) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140fbd530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rax as u32)) as u32 as u64;
    rcx = P1;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0xd8u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(P2))) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (((rax) as i64).wrapping_mul((0xd8u64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(P2))) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140fbd580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rax as u32)) as u32 as u64;
    rcx = P1;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0xd8u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(rbx)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (((rax) as i64).wrapping_mul((0xd8u64) as i64) as u64) as u64;
    rax = ((rax).wrapping_add(rbx)) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140fbd670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff58; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598; const P3: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; r14 = rcx; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x10); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = r8; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rcx); bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x10); bb = 9; continue; }
            9 => { fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = r8; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { rax = (0x69fafafafaffu64) as u64; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x28eu32) as u32); r8 = rsi.wrapping_add(0x8); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rcx = r8.wrapping_add(0x4); crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rcx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x8u64) as u64); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = ((xmm0 as u64)) as u64; xmm0 = (ymm0_hi) as u128; ymm0_hi = 0; crate::simd_ops::lk_and_u32(rax, (0x1ffu32) as u32); crate::simd_ops::lk_or_u32(rax, (0x200u32) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(r8.wrapping_sub(0x8u64), (((xmm0 >> 64) as u64)) as u64); crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140fbd7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff58; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598; const P3: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; r14 = rcx; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x10); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = r8; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rcx); bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x10); bb = 9; continue; }
            9 => { fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = r8; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { rax = (0x69fafafafaffu64) as u64; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x293u32) as u32); r8 = rsi.wrapping_add(0x8); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rcx = r8.wrapping_add(0x4); crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rcx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x8u64) as u64); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = ((xmm0 as u64)) as u64; xmm0 = (ymm0_hi) as u128; ymm0_hi = 0; crate::simd_ops::lk_and_u32(rax, (0x1ffu32) as u32); crate::simd_ops::lk_or_u32(rax, (0x200u32) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(r8.wrapping_sub(0x8u64), (((xmm0 >> 64) as u64)) as u64); crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140fbd950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ff58; const P1: u64 = 0x1426455a8; const P2: u64 = 0x142645598; const P3: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; r15 = rcx; r14 = rdx; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x20); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = r8; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rcx); bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x20); bb = 8; continue; }
            8 => { fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = r8; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 12; continue; }
            12 => { rax = (0x69fafafafaffu64) as u64; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x28bu32) as u32); r8 = rsi.wrapping_add(0x8); rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rcx = r8.wrapping_add(0x4); crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rcx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x14u64) as u64); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = ((xmm0 as u64)) as u64; xmm0 = (ymm0_hi) as u128; ymm0_hi = 0; crate::simd_ops::lk_and_u32(rax, (0x1ffu32) as u32); crate::simd_ops::lk_or_u32(rax, (0x200u32) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(r8.wrapping_sub(0x8u64), (((xmm0 >> 64) as u64)) as u64); crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140fbdac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x1421f49e0; const P3: u64 = 0x1426455a8; const P4: u64 = 0x142645598; const P5: u64 = 0x141e5d3e0; const P6: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rbp = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; r8 = P1; r8 = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm1 = ((crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32) as u128; rcx = (crate::simd_ops::ru64(r14)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x28); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x28); bb = 11; continue; }
            11 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { rax = (0x69fafafafaffu64) as u64; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(r14)) as u128; rax = P5; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0xcu64), rax); crate::simd_ops::wu128(rsi.wrapping_add(0x14u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsi, (0x1cu64) as u64); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140fe0c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru64(rdx)) as u64; rsi = 0x3fff; r9 = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rdi = rcx; rax = ((crate::simd_ops::ru16(r10)) as u32) as u32 as u64; r11 = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((rsi as u16)))) as u16 as u64)); r11 = (r11 & !0xffffu64) | ((((((r11 as u16)) & ((rsi as u16)))) as u16 as u64)); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r11 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { fl_k = 0; bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r9)) as u64; fl_l = (crate::simd_ops::ru64(r10)) as u64; fl_r = rax; fl_k = 1; bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((if (match fl_k { 0 => (((fl_l as u16)) < ((fl_r as u16))), 1 => (((fl_l as u64)) < ((fl_r as u64))), _ => false }) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm1 = (crate::simd_ops::ru128(r10)) as u128; xmm0 = ((crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64) as u128; rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(r10, rax); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(r10.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r9.wrapping_add(0xcu64))) as u64; crate::simd_ops::wu64(r10.wrapping_add(0xcu64), rax); crate::simd_ops::wu128(rsp, (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xcu64))) as u64; crate::simd_ops::wu64(r9, ((xmm1 as u64)) as u64); crate::simd_ops::wu64(r9.wrapping_add(0xcu64), rax); crate::simd_ops::wu32(r9.wrapping_add(0x8u64), (((xmm1 >> 64) as u32) as u32) as u32); bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r8)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((rsi as u16)))) as u16 as u64)); rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) & ((rsi as u16)))) as u16 as u64)); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { fl_k = 0; bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = rax; fl_k = 1; bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((if (match fl_k { 0 => (((fl_l as u16)) < ((fl_r as u16))), 1 => (((fl_l as u64)) < ((fl_r as u64))), _ => false }) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { xmm1 = (crate::simd_ops::ru128(r8)) as u128; xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64) as u128; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8, rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xcu64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0xcu64), rax); crate::simd_ops::wu64(rdx, ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), (((xmm1 >> 64) as u32) as u32) as u32); crate::simd_ops::wu128(rsp, (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xcu64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0xcu64), rax); rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((rsi as u16)))) as u16 as u64)); r8 = (r8 & !0xffffu64) | ((((((r8 as u16)) & ((rsi as u16)))) as u16 as u64)); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r8 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { fl_k = 0; bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = rax; fl_k = 1; bb = 9; continue; }
            9 => { rax = (rax & !0xffu64) | (((if (match fl_k { 0 => (((fl_l as u16)) < ((fl_r as u16))), 1 => (((fl_l as u64)) < ((fl_r as u64))), _ => false }) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm1 = (crate::simd_ops::ru128(rdx)) as u128; xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64) as u128; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx, rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xcu64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0xcu64), rax); crate::simd_ops::wu128(rsp, (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xcu64))) as u64; crate::simd_ops::wu64(rcx, ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0xcu64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (((xmm1 >> 64) as u32) as u32) as u32); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410128c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143cd12b8; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x143cc7c80; const P4: u64 = 0x580e0; const P5: u64 = 0x141e99470;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(r8.wrapping_mul(2)))) as u32) as u32 as u64;
    rcx = ((rcx).wrapping_shl(4u32)) as u64;
    rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P2)))) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64;
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64);
    r8 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    rcx = rsp.wrapping_add(0x40);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141013e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143cd0bbc; const P1: u64 = 0x180ac; const P2: u64 = 0x18098; const P3: u64 = 0x142489ee0; const P4: u64 = 0x40; const P5: u64 = 0x141e970b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; rbx = 0; fl_l = 0; fl_r = 0; r11 = (((rdx as u16)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rsi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rbx as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { r8 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); rdx = (crate::simd_ops::ru32(rax.wrapping_add(rsi).wrapping_add(P1))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(P2))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(P4))) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 7; continue; }
            6 => { rdi = rbx; bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rdi; rax = ((r14.wrapping_add(rdi)) as u64) as u64; rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rdi }) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r8 = rsi; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdx = rsi; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 11; continue; }
            11 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141049a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143cada18; const P2: u64 = 0x143c9a338; const P3: u64 = 0x142489ee0; const P4: u64 = 0x141eade80;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141049a90, funcinfo_rva: 0x3667a7c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r14 = (rcx) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_add(r8)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = (crate::simd_ops::ru32(rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141049ad1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rax as u32)) as u32 as u64; r15 = (((rax) as i64).wrapping_mul((0x188u64) as i64) as u64) as u64; r15 = ((r15).wrapping_add(rdi)) as u64; rbp = (crate::simd_ops::ru64(r14)) as u64; rcx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x580e0u64))) as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rdi = ((rbx.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rdx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdx) as u64); rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbp).wrapping_add(0x180acu64))) as u32 as u64; r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbp).wrapping_add(0x18098u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141049b67); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0xffffffffffffffffu64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((r11.wrapping_add(0xc03u64)) as u64) as u64; fl_l = ((rax).wrapping_shl(5u32)) as u64 as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rbp).wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rcx = ((rsi.wrapping_add(rax)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rsp.wrapping_add(0x50u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm1 as u64)) as u64); r8 = (r14) as u64; rdx = ((r15.wrapping_add(0x580f0u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141049bc6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 12; continue; }
            12 => { r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141049ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c900d8; const P1: u64 = 0x141048720; const P2: u64 = 0x141eb1010;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rbp = r11.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xe8u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rsi); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdi); rdi = ((rbx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r13); r13 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r15); rsi = (crate::simd_ops::ru64(r13.wrapping_add(0x580e0u64))) as u64; rsi = (crate::simd_ops::ru64(rsi.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x20u64), r14); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 7; continue; }
            7 => { r8 = r13.wrapping_add(0x40); crate::simd_ops::wu16(rbp.wrapping_sub(0x79u64), ((r9 as u16)) as u16); rcx = rbp.wrapping_sub(0x19); crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), r13); crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r11 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { r11 = ((rax.wrapping_add(r14)) as u64) as u64; bb = 10; continue; }
            10 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r10).wrapping_add(r14)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((r9).wrapping_add(r14)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((r8).wrapping_add(r14)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdx).wrapping_add(r14)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rcx).wrapping_add(r14)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = ((rax).wrapping_add(r14)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { let y_addr: u64 = rbp.wrapping_sub(0x79u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rdx); rdx = rsi; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rcx); rcx = rbp.wrapping_sub(0x19); let y_addr: u64 = rbp.wrapping_sub(0x19u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), r11); crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), r10); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r9); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), r8); let y_addr: u64 = rbp.wrapping_sub(0x59u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x7u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 24; continue; }
            24 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; bb = 25; continue; }
            25 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0xe8u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14104a070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143cd4460; const P1: u64 = 0x143c900d8; const P2: u64 = 0x1410484b0; const P3: u64 = 0x141eb0b90;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rbp = r11.wrapping_sub(0x38); rsp = ((rsp).wrapping_sub(0x128u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdi); rdi = ((rbx.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r12); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r15); r15 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rax = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x580e0u64))) as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rsi); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(r15.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 7; continue; }
            7 => { r8 = r15.wrapping_add(0x40); crate::simd_ops::wu16(rsp.wrapping_add(0x40u64), ((r9 as u16)) as u16); rcx = rbp.wrapping_sub(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r11 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { r11 = ((rax.wrapping_add(rsi)) as u64) as u64; bb = 10; continue; }
            10 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r10).wrapping_add(rsi)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((r9).wrapping_add(rsi)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((r8).wrapping_add(rsi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdx).wrapping_add(rsi)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rcx).wrapping_add(rsi)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = ((rax).wrapping_add(rsi)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r8); r8 = r12; let y_addr: u64 = rbp.wrapping_sub(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rdx); rdx = rbp; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rcx); rcx = rbp.wrapping_sub(0x60); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r11); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r10); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); let y_addr: u64 = rsp.wrapping_add(0x60u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x40u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; bb = 25; continue; }
            25 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x158u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x148u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0x128u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14104a820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143cd4460; const P1: u64 = 0x143cccd90; const P2: u64 = 0x142489ee0; const P3: u64 = 0x143c8f7a0; const P4: u64 = 0x143cd4970; const P5: u64 = 0x141eac650;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xf0u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rsi); rsi = ((rbx.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r13); r13 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); rax = (crate::simd_ops::ru64(r13)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r15); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r14); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; r15 = (crate::simd_ops::ru64(r13.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rdx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x38u64), ((rax as u16)) as u16); rax = ((rax).wrapping_shl(5u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); rbp = (crate::simd_ops::ru32(rax.wrapping_add(r13).wrapping_add(0x180acu64))) as u32 as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(r13).wrapping_add(0x18098u64))) as u64; rdx = ((rbp as u32)) as u32 as u64; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r13).wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = 0xc03; rdx = rbp; rcx = r12; r14 = ((r11.wrapping_add(rax)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r14).wrapping_shl(5u32)) as u64; r14 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(r13).wrapping_add(0x40u64))) as u64; r11 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r11 = 0; fl_l = 0; fl_r = 0; fl_l = ((r14).wrapping_shl(5u32)) as u64; r14 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { r8 = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rdx = rbp; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(r13).wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 16; continue; }
            15 => { rax = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; ymm0_hi = 0; fl_l = rdi; fl_r = rdi; rcx = ((r15.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm0 as u64)) as u64); let y_addr: u64 = rsp.wrapping_add(0x38u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rdi }) as u64; rdx = rsp.wrapping_add(0x70); fl_l = r11; fl_r = r11; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); rcx = ((r15.wrapping_add(r11)) as u64) as u64; r11 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { r11 }) as u64; rcx = ((rax.wrapping_add(r15)) as u64) as u64; let y_addr: u64 = rsp.wrapping_add(0x90u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r11); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x58u64))) as u128; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rcx } else { rax }) as u64; rcx = rsp.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; bb = 20; continue; }
            20 => { rsp = ((rsp).wrapping_add(0xf0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14104b260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143cd49ec; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x143c8ce80; const P4: u64 = 0x580e0; const P5: u64 = 0x141039f40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    r9 = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = (0x86304bf548c63ad1u64) as u64;
    rdx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    rax = (crate::simd_ops::ru64(r9)) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64;
    rdx = rsp.wrapping_add(0x40);
    rcx = ((rcx).wrapping_shl(4u32)) as u64;
    rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r9.wrapping_add(P2)))) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64;
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64);
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410767a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de830; const P1: u64 = 0x18; const P2: u64 = 0x1403d1260; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rbx.wrapping_add(P3), rax);
    rax = (tmp_xchg) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141076d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142119010; const P1: u64 = 0x142118e10; const P2: u64 = 0x1401fa730; const P3: u64 = 0x143cca1f8; const P4: u64 = 0x142489ee0; const P5: u64 = 0x1426467e8; const P6: u64 = 0x1406574b0; const P7: u64 = 0x1406573b0; const P8: u64 = 0x142116bc0; const P9: u64 = 0x1421158a0; const P10: u64 = 0x143373588; const P11: u64 = 0x141ef4820;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdx = 0x1; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r10); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r10); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 5; continue; }
            4 => { r8 = rsp.wrapping_add(0x70); rcx = rbx; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); rcx = ((rdx as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 11; continue; } bb = 8; continue; }
            8 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rsi = (((rbx as u16)) as u32) as u32 as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(rbp).wrapping_add(0x180a8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rbp).wrapping_add(0x18090u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(rbp).wrapping_add(0x180acu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rbp).wrapping_add(0x18098u64))) as u64; r11 = (((rbx as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = r11.wrapping_add(0xc03); rax = ((rax).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_shr(32u32)) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rbp).wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64)))) as u64; fl_l = ((rax).wrapping_add(rbx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141076ee1, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x78); rcx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141076f15, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141076f27, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { r8 = P10; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141076f3c, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141076f40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x30; const P2: u64 = 0x143cbe480; const P3: u64 = 0x180a8; const P4: u64 = 0x18090; const P5: u64 = 0x142489ee0; const P6: u64 = 0x180ac; const P7: u64 = 0x18098; const P8: u64 = 0x40; const P9: u64 = 0x1426467e8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (((rbx as u16)) as u32) as u32 as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(rsi).wrapping_add(P3))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rsi).wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(rsi).wrapping_add(P6))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(rsi).wrapping_add(P7))) as u64; r11 = (((rbx as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = r11.wrapping_add(0xc03); rax = ((rax).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_shr(32u32)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(P8))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64)))) as u64; rax = (((rbx) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = rdx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rdx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141077007, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141077390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142119010; const P1: u64 = 0x142118e10; const P2: u64 = 0x1401fa730; const P3: u64 = 0x143cd6e58; const P4: u64 = 0x142489ee0; const P5: u64 = 0x1426467e8; const P6: u64 = 0x1406574b0; const P7: u64 = 0x1406573b0; const P8: u64 = 0x142116bc0; const P9: u64 = 0x1421158a0; const P10: u64 = 0x1433735e8; const P11: u64 = 0x141ef4820;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdx = 0x1; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r10); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r10); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 5; continue; }
            4 => { r8 = rsp.wrapping_add(0x70); rcx = rbx; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); rcx = ((rdx as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 11; continue; } bb = 8; continue; }
            8 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rsi = (((rbx as u16)) as u32) as u32 as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(rbp).wrapping_add(0x180a8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rbp).wrapping_add(0x18090u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(rbp).wrapping_add(0x180acu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rbp).wrapping_add(0x18098u64))) as u64; r11 = (((rbx as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = r11.wrapping_add(0xc03); rax = ((rax).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_shr(32u32)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rbp).wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64)))) as u64; rcx = (((rbx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; fl_l = ((rax).wrapping_add(rcx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1410774f4, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x78); rcx = rsp.wrapping_add(0x20); xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141077528, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rdx = 0x1; rcx = rdi; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14107753a, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { r8 = P10; rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x14107754f, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141077720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401ffc30; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; r15 = 0x3fffffff; rdi = (((rdi) & (r15))) as u64; r14 = rdx; rax = rdi; rsi = rcx; rax = ((rax).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = (0x8u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = 0x8; bb = 3; continue; }
            2 => { fl_l = rdi; fl_r = r15; rdi = (if (((fl_l as u64)) > ((fl_r as u64))) { r15 } else { rdi }) as u64; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rdx = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; rbx = (((rbx) & (0xffffffffffffffe0u64))) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rbx.wrapping_add(0x1du64), ((rax as u16)) as u16); crate::simd_ops::wu8(rbx.wrapping_add(0x1fu64), ((rax as u8)) as u8); crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x1cu64), ((rax as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0xbf800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), (0xbf800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), (0xbf800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0xbf800000u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x14u64), (0xffffffffbf800000u64) as u64); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = rbp; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rdx = ((rdx).wrapping_add(0x20u64)) as u64; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rcx = rcx.wrapping_add(0x20); fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { bb = 14; continue; }
            8 => { rax = rbp; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rdx = ((rdx).wrapping_add(0x20u64)) as u64; let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rax = rax.wrapping_add(0x20); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = rbx.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(r14)) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { let y_addr: u64 = rcx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rcx = ((rcx).wrapping_add(0x20u64)) as u64; let y_addr: u64 = rdx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rdx = rdx.wrapping_add(0x20); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x40000000; r9 = (crate::simd_ops::ru64(rsi)) as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 15; continue; }
            15 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 16; continue; }
            16 => { rcx = r9.wrapping_sub(0x1); rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); if (((fl_l as u32)) != 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            20 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 22; continue; }
            21 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 22; continue; }
            22 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rdi = ((((rdi as u32)) & ((r15 as u32)))) as u32 as u64; crate::simd_ops::wu64(rsi, rbp); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32); rax = rbx; crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410a7200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(r9)) as u32) as u128;
    crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32);
    xmm1 = ((crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm1 as u32)) as u32);
    xmm4 = ((crate::simd_ops::ru32(r9.wrapping_add(P1))) as u32) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((xmm4 as u32)) as u32);
    xmm3 = ((crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm3 as u32)) as u32);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128);
    crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32);
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P0)))).to_bits() as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm1 as u32)) as u32);
    xmm0 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P1)))).to_bits() as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((xmm0 as u32)) as u32);
    xmm1 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P2)))).to_bits() as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm1 as u32)) as u32);
    xmm2 = crate::simd_ops::vbroadcastss((xmm2 as u32));
    xmm1 = crate::simd_ops::vmulps(xmm2, crate::simd_ops::ru128(rcx));
    crate::simd_ops::wu128(rcx, (xmm1) as u128);
    xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128);
    crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32);
    xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32) as u128;
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P0)))).to_bits() as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm2 as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32) as u128;
    xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P1)))).to_bits() as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((xmm1 as u32)) as u32);
    xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32) as u128;
    xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P2)))).to_bits() as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm0 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c6220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1410ca800; const P1: u64 = 0x143cdb988; const P2: u64 = 0x1420af120; const P3: u64 = 0x140215750; const P4: u64 = 0x143c8cea0; const P5: u64 = 0x140215610; const P6: u64 = 0x143cdb400; const P7: u64 = 0x143cdb540; const P8: u64 = 0x143c8ceec; const P9: u64 = 0x1420a28e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; rbx = r9; rdi = r8; rsi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rcx = rsp.wrapping_add(0xb8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r9 = ((crate::simd_ops::ru16(r9.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; r8 = rsp.wrapping_add(0x50); rdx = rdi; rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rsi.wrapping_add(0xf0); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0x90u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x40u64), ((xmm0 as u64)) as u64); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rbx = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x40u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x44u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x40u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x40u64), (1) as u32); rbx = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x50u64), (1) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x113u64), (0x1u8) as u8); rbx = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x50u64), (1) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x113u64), (0x1u8) as u8); rbx = ((crate::simd_ops::ru16(P8)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x50u64), (1) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x113u64), (0x1u8) as u8); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm1 as u64)) as u64); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c6420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1410ca6b0; const P1: u64 = 0x143cdb40c; const P2: u64 = 0x1420af120; const P3: u64 = 0x140215750; const P4: u64 = 0x143cdb640; const P5: u64 = 0x143cdbcd4; const P6: u64 = 0x143cdb70c; const P7: u64 = 0x1420a25b0; const P8: u64 = 0x143c8cea0; const P9: u64 = 0x140215610; const P10: u64 = 0x143cdb400; const P11: u64 = 0x1420a28e0; const P12: u64 = 0x141834a78; const P13: u64 = 0x1410c8e20; const P14: u64 = 0x141834a0c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 61>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0xe0); rsp = ((rsp).wrapping_sub(0x1e0u64)) as u64; r14 = r9; rsi = r8; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rcx); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = ((crate::simd_ops::ru16(r9.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = rsi; rcx = rbp.wrapping_sub(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rdi.wrapping_add(0xf0); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x50u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x40u64), ((xmm0 as u64)) as u64); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rbx = P4; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x11u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x7u64) as u64); rax = P6; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdx = rsp.wrapping_add(0x70); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((crate::simd_ops::ru16(P8)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x44u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rdi.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x40u64), ((rax as u32)) as u32); rbx = ((crate::simd_ops::ru16(P10)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x44u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rdi.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x40u64), (1) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x1e0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x30u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rdx.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x50u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rdx.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_add(0x60u64), (xmm0) as u128); rdx = rdx.wrapping_add(0x80); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x70u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x10u64), (xmm1) as u128); rax = rax.wrapping_add(0x80); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { bb = 8; continue; }
            10 => { rcx = P5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P5)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 11; continue; }
            11 => { rcx = rbp.wrapping_sub(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; r8 = 0x2; bb = 8; continue; }
            12 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rcx = P5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c68d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c9f9c0; const P1: u64 = 0x143cdb414; const P2: u64 = 0x143ccef30; const P3: u64 = 0x142489ee0; const P4: u64 = 0x141f443a0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; r11 = (((rdx as u16)) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(rbx.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rsi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x580e0u64))) as u64; rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); rdi = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r12); r12 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(rdi.wrapping_mul(8)))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx).wrapping_add(0x180acu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x18098u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rbx).wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 7; continue; }
            6 => { rbp = rsi; bb = 7; continue; }
            7 => { fl_l = rbp; fl_r = rbp; rax = ((r15.wrapping_add(rbp)) as u64) as u64; rbp = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbp }) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); r9 = r12; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); r8 = rdi; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; rdx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c6a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1410c8ca0; const P1: u64 = 0x143cdb414; const P2: u64 = 0x1401fc320; const P3: u64 = 0x143cdb458; const P4: u64 = 0x143cdb418; const P5: u64 = 0x143cdb424; const P6: u64 = 0x1420a25b0; const P7: u64 = 0x143ccef30; const P8: u64 = 0x140215610; const P9: u64 = 0x143c9f9c0; const P10: u64 = 0x1420a28e0; const P11: u64 = 0x141834a78; const P12: u64 = 0x143c8f7a0; const P13: u64 = 0x143ccef2c; const P14: u64 = 0x143cdb428; const P15: u64 = 0x141834a0c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rdi = r9; r14 = r8; rsi = rcx; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), r8); crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), r9); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), rcx); rax = P0; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x31u64), ((rax as u16)) as u16); rbx = rcx.wrapping_add(0x100); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rdi); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rsi); rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x1u64) as u64); rax = P4; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), (0x1u64) as u64); rax = P5; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); let y_addr: u64 = rbp.wrapping_sub(0x39u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x27u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x19u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), ((xmm0 as u64)) as u64); rdx = rbp.wrapping_add(0x27); rcx = rbp.wrapping_sub(0x39); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x50u64), (1) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x113u64), (0x1u8) as u8); rbx = ((crate::simd_ops::ru16(P9)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x40u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x44u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x40u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x40u64), (1) as u32); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x7u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), ((xmm1 as u64)) as u64); rcx = rbp.wrapping_sub(0x19); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(P12)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x39u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), (0x2u32) as u32); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), (0x1u32) as u32); rax = ((crate::simd_ops::ru16(P13)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x67u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), (0x3u32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x2du64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; ymm0_hi = 0; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (xmm1 & 0xffff_ffff_ffff_ffffu128); ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); crate::simd_ops::wu128(P4, (xmm0) as u128); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x29u64))) as u32 as u64; crate::simd_ops::wu32(P14, ((rax as u32)) as u32); rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c6d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x16u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c6d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x24; const P6: u64 = 0x2c; const P7: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    xmm0 = ((crate::simd_ops::ru64(r8)) as u64) as u128;
    crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P3), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(r8.wrapping_add(P5))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P5), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P6))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P7), (0x17u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c6de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x18u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c6e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x30; const P3: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(rdx.wrapping_add(P0), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rdx.wrapping_add(P1), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P2))) as u128;
    crate::simd_ops::wu128(rdx.wrapping_add(P2), (xmm0) as u128);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P3), (0x19u8) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c6e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x1au8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c6ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x1cu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c6f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x1eu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c6f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x20u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c6fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x22u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x24u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c70c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x26u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x28u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x2au8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c71e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x2cu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x2eu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c72a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x30u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x32u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x1401e84a0; const P4: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P4), (0x33u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x34u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c73e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x36u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x38u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c74a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x3au8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x3cu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x3eu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c75c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x40u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x42u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x44u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c76e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x46u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x48u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c77a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x4au8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x4cu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x4eu8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c78c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x50u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410c7920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rdx, r9);
    rax = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(r8, r9);
    crate::simd_ops::wu64(r8.wrapping_add(P0), r9);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x52u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410d0960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x108; const P5: u64 = 0x4; const P6: u64 = 0x100; const P7: u64 = 0xe; const P8: u64 = 0x20; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x230u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = ((rcx).wrapping_add(rbx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x230u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rcx = ((r9.wrapping_add(P3)) as u64) as u64; rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = ((rcx.wrapping_add(P4)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P6))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(P8), r9); if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = rdx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            10 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            11 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1410d0f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f3d00; const P1: u64 = 0x8; const P2: u64 = 0x143391070; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x20u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P2;
    crate::simd_ops::wu64(rdi, rax);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x30; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14113c3e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d910; const P1: u64 = 0x143ce0c20; const P2: u64 = 0x143c8f63c; const P3: u64 = 0x14113bf30; const P4: u64 = 0x141faf5e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rbp = r11.wrapping_sub(0x18); rsp = ((rsp).wrapping_sub(0x108u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rsi); rsi = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rdi); rdi = ((rbx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r13); r13 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r14); r14 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x580e0u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(rsi.wrapping_mul(8)))) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(r14.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r15); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(r13.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 7; continue; }
            7 => { r8 = r13.wrapping_add(0x40); crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), ((r9 as u16)) as u16); rcx = rbp.wrapping_sub(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r11 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { r11 = ((rax.wrapping_add(r15)) as u64) as u64; bb = 10; continue; }
            10 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r10).wrapping_add(r15)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((r9).wrapping_add(r15)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((r8).wrapping_add(r15)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdx).wrapping_add(r15)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rcx).wrapping_add(r15)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = ((rax).wrapping_add(r15)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r9); r9 = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r8); r8 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdx); rdx = r13; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rcx); rcx = rbp.wrapping_sub(0x70); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; let y_addr: u64 = rbp.wrapping_sub(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r11); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r10); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x50u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 24; continue; }
            24 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; bb = 25; continue; }
            25 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0x108u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14113c6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143ce0ed8; const P1: u64 = 0x143ce0c20; const P2: u64 = 0x14113bb60; const P3: u64 = 0x141faeb80;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rbp = r11.wrapping_sub(0x38); rsp = ((rsp).wrapping_sub(0x128u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdi); rdi = ((rbx.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r12); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r15); r15 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rax = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x580e0u64))) as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rsi); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(r15.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 7; continue; }
            7 => { r8 = r15.wrapping_add(0x40); crate::simd_ops::wu16(rsp.wrapping_add(0x40u64), ((r9 as u16)) as u16); rcx = rbp.wrapping_sub(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r11 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { r11 = ((rax.wrapping_add(rsi)) as u64) as u64; bb = 10; continue; }
            10 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r10).wrapping_add(rsi)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((r9).wrapping_add(rsi)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = ((r8).wrapping_add(rsi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdx).wrapping_add(rsi)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = ((rcx).wrapping_add(rsi)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = ((rax).wrapping_add(rsi)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r8); r8 = r12; let y_addr: u64 = rbp.wrapping_sub(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rdx); rdx = rbp; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rcx); rcx = rbp.wrapping_sub(0x60); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r11); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r10); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r9); let y_addr: u64 = rsp.wrapping_add(0x60u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x40u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); xmm1 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; bb = 25; continue; }
            25 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x158u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x148u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0x128u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14113e200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141facc80; const P1: u64 = 0x140225740; const P2: u64 = 0x1402c1130; const P3: u64 = 0x141facd00; const P4: u64 = 0x1402c11a0; const P5: u64 = 0x142535070; const P6: u64 = 0x1426456c8; const P7: u64 = 0x1426453a0; const P8: u64 = 0x1401f2bc0; const P9: u64 = 0x141fae620; const P10: u64 = 0x141fae560; const P11: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rdx; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rdx, rax); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r8, rbx); bb = 15; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; rbx = (crate::simd_ops::ru64(r8)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x10); rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x20); rcx = rdi.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x30); rcx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x40); rcx = rdi.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            7 => { rbp = (crate::simd_ops::ru64(r8)) as u64; r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = rdi; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 12; continue; }
            10 => { rdx = 0x50; r8 = ((rdx.wrapping_sub(0x40u64)) as u32) as u32 as u64; rcx = r9; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rdi = rbx; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); rdx = rbp; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x10); crate::simd_ops::wu64(rcx, rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbx); rdx = rbp.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x20); crate::simd_ops::wu64(rcx, rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbx); rdx = rbp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x30); crate::simd_ops::wu64(rcx, rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbx); rdx = rbp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x40); crate::simd_ops::wu64(rcx, rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rbx); rdx = rbp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rdi); bb = 15; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rbx.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, rbx); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14129d5e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x118; const P1: u64 = 0x142442de0; const P2: u64 = 0x8; const P3: u64 = 0x14240ab10; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x138; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ab2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412ac6d0; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x142183db0; const P5: u64 = 0x1401f4d00;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; rdi = rdx; r14 = rcx; r15 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = r9; rcx = rax.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x28u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rbp as u32)) as u32); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x10u64), (1) as u32); rdx = rbx; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = rbx; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r14.wrapping_add(0x18u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 40; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = r14; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rbx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { bb = 40; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, r15); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x10u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rsi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rsi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = rcx; r9 = rsi; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rbx.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = rcx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { bb = 40; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, r15); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 40; continue; }
            34 => { rbx = ((crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u32) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 35; continue; }
            35 => { rsi = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { r8 = rsi; rdx = (crate::simd_ops::ru64(r9)) as u64; rcx = rsp.wrapping_add(0x24); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            37 => { crate::simd_ops::wu64(rsp.wrapping_add(0x2cu64), rsi); bb = 39; continue; }
            38 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); bb = 39; continue; }
            39 => { crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rbx as u8)) as u8); rdx = rsp.wrapping_add(0x2c); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 40; continue; }
            40 => { rax = rdi; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ab680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412ac900; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x142199fd0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = ((r8 as u32)) as u32 as u64; rdi = rdx; rbp = rcx; r15 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = r9; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x28u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x20u64), (1) as u32); rdx = rbx; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = rbx; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rbp.wrapping_add(0x30u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 35; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rbx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { bb = 35; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, r15); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x20u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rsi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rsi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = rcx; r9 = rsi; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rbx.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = rcx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { bb = 35; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, r15); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 35; continue; }
            34 => { r8 = rbp; rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ab950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412acc20; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x142199620; const P5: u64 = 0x1401f3c00;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412ab950, funcinfo_rva: 0x3690620, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; rdi = (rdx) as u64; r14 = (rcx) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (0xfffffffffffffffcu64) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = (r9) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ab997); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x28u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rsi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::lk_add_u32(rsi, (1) as u32); rdx = (rbx) as u64; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = (rbx) as u64; fl_l = (rax) as u64 as u64; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r14.wrapping_add(0x28u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, (rsi) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 38; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ab9f4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi, (rsi) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412aba04); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rbx.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412aba69); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412aba7b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { bb = 38; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, (r15) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412abac2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412abac8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { r10 = ((rsi.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rsi) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rsi) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412abb29); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = (rcx) as u64; r9 = (rsi) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412abb41); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412abb5d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((rbx.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412abbbf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = (rcx) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412abbd4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            32 => { bb = 38; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, (r15) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 38; continue; }
            34 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); bb = 37; continue; }
            36 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); bb = 37; continue; }
            37 => { rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = (r9) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412abc13); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412abc1b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 38; continue; }
            38 => { rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412abc40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412accf0; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x142183db0; const P5: u64 = 0x1401f4d00;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; rdi = rdx; r14 = rcx; r15 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = r9; rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rbp as u32)) as u32); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x10u64), (1) as u32); rdx = rbx; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = rbx; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r14.wrapping_add(0x18u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 40; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = r14; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rbx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { bb = 40; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, r15); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x10u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rsi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rsi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = rcx; r9 = rsi; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rbx.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = rcx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { bb = 40; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, r15); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 40; continue; }
            34 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 36; continue; } bb = 35; continue; }
            35 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(r9)) as u64; rcx = rsp.wrapping_add(0x24); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            36 => { rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x2cu64), rax); bb = 39; continue; }
            38 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); bb = 39; continue; }
            39 => { rdx = rsp.wrapping_add(0x2c); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 40; continue; }
            40 => { rax = rdi; r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412abf40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412acdc0; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x1412ab5d0; const P5: u64 = 0x14219a120;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; rdi = rdx; r14 = rcx; r15 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = r9; rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rbp as u32)) as u32); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x20u64), (1) as u32); rdx = rbx; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = rbx; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r14.wrapping_add(0x20u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 40; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = r14; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rbx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { bb = 40; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, r15); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x20u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rsi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rsi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = rcx; r9 = rsi; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rbx.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = rcx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { bb = 40; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, r15); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 40; continue; }
            34 => { rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { r8 = (crate::simd_ops::ru64(r14)) as u64; rax = ((r8 as u32)) as u32 as u64; bb = 37; continue; }
            36 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 37; continue; }
            37 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rcx = rdi; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((r15 as u8)) as u8); rdx = rsp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 40; continue; }
            39 => { rdx = (crate::simd_ops::ru64(r9)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 40; continue; }
            40 => { rax = rdi; r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ac240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412ace90; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x1401f4d00; const P5: u64 = 0x14219a3f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412ac240, funcinfo_rva: 0x3690718, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r10 = (r9) as u64; rbp = ((r8 as u32)) as u32 as u64; rdi = (rdx) as u64; r14 = (rcx) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (0xfffffffffffffffcu64) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = (r9) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac28a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x28u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rsi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rbp as u32)) as u32); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x10u64), (1) as u32); rdx = (rbx) as u64; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = (rbx) as u64; fl_l = (rax) as u64 as u64; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r14.wrapping_add(0x28u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, (rsi) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 37; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac2e8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi, (rsi) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac2f8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rbx.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac35d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac36f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { bb = 37; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, (r15) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x10u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac3b7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac3bd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { r10 = ((rsi.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rsi) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rsi) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac41e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = (rcx) as u64; r9 = (rsi) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac436); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac452); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((rbx.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac4b4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = (rcx) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac4c9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            32 => { bb = 37; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, (r15) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 37; continue; }
            34 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x3cu64), ((r15 as u8)) as u8); rdx = ((rsp.wrapping_add(0x34u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac4fb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 37; continue; }
            36 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru64(r10)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ac517); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 37; continue; }
            37 => { rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ac9d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140208a00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (((crate::simd_ops::ru8(rdx.wrapping_add(0x108u64))) as i8 as i64) as u64) as u64; r8 = rcx; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 33; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; r9 = rdx.wrapping_add(0x8); fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 7; continue; }
            6 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r9); r9 = rdx.wrapping_add(0x50); rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x4bu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r9); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x18u64), (0x4u32) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            15 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 18; continue; }
            17 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r9); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x18u64), (0x1u32) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 24; continue; }
            23 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r9); r9 = rdx.wrapping_add(0x50); rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x4bu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 29; continue; }
            28 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r9); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x18u64), (0x0u32) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            32 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            33 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1412acb5f, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412acf60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f4c80; const P1: u64 = 0x1401f4e00; const P2: u64 = 0x142442de0; const P3: u64 = 0x14219a3f0; const P4: u64 = 0x1412ad440; const P5: u64 = 0x1401f3450; const P6: u64 = 0x1401f4c10;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412acf60, funcinfo_rva: 0x3690870, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x98, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r14 = (rcx) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x30u64), ((r15 as u32)) as u32); rsi = ((r15.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x48u64), ((r15 as u8)) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412acfa0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); crate::simd_ops::wu16(rbp.wrapping_sub(0x18u64), (0x100u16) as u16); rdx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412acfb9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rbx) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), ((r15 as u32)) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x10u64), (1) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x30u64), (0x2u32) as u32); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x10u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412acff4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412acffa); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ad003); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rdx)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ad021); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rbp.wrapping_add(0x30u64), (0x1u32) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x28u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x28u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x29u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x27u64), ((rax as u8)) as u8); crate::simd_ops::wu64(r14.wrapping_add(0x20u64), (r15) as u64); r9 = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x38u64))) as u32 as u64; rdx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ad059); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rbp.wrapping_add(0x30u64), (0x5u32) as u32); rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ad069); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 9; continue; }
            9 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ad09a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ad0a0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 13; continue; }
            13 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ad0a9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (r15) as u64); bb = 15; continue; }
            15 => { rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ad0b6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 16; continue; }
            16 => { let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x10u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 17; continue; }
            17 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ad0e6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ad0ec); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 20; continue; }
            20 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ad0f5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 21; continue; }
            21 => { r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ad120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412ac640; const P1: u64 = 0x1412ad320; const P2: u64 = 0x142442de0; const P3: u64 = 0x14219a120; const P4: u64 = 0x1412ad250;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu8(rax.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbp = rcx; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_add(0x18u64), ((rdi as u32)) as u32); r9 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u32) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x58u64), r8); rax = ((r8 as u32)) as u32 as u64; bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 3; continue; }
            3 => { rsi = 0xffffffff; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), ((rdi as u8)) as u8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu16(rsp.wrapping_add(0x48u64), (0x100u16) as u16); rdx = rsp.wrapping_add(0x50); rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x20u64), (1) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), (0x2u32) as u32); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x20u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { bb = 11; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = rbp.wrapping_add(0x18); rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 12; continue; }
            12 => { let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x20u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = (((rdi) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ad320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x142645598; const P2: u64 = 0x1426455c0; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = rcx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = ((xmm0 as u32)) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu128(rsi, (xmm0) as u128); crate::simd_ops::wu8(rsi.wrapping_add(0x10u64), ((rax as u8)) as u8); bb = 11; continue; }
            6 => { crate::simd_ops::wu32(rsi, ((rcx as u32)) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x10u64), ((rax as u8)) as u8); bb = 11; continue; }
            7 => { crate::simd_ops::wu8(rsi.wrapping_add(0x10u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu128(rsi, (xmm0) as u128); bb = 10; continue; }
            9 => { crate::simd_ops::wu32(rsi, ((rcx as u32)) as u32); bb = 10; continue; }
            10 => { crate::simd_ops::wu8(rsi.wrapping_add(0x18u64), (0x1u8) as u8); bb = 11; continue; }
            11 => { r14 = 0x1; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rsi.wrapping_add(0x28u64), r14); r14 = (tmp_xchg) as u64; rax = (((r14 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(1u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rsi.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x40u64), (1) as u32); rcx = rsi.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { fl_l = (((r14) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x20u64), (1) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rax = (crate::simd_ops::ru64(r14)) as u64; rdx = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x20u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ad440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412ada30; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x1401f4e00; const P5: u64 = 0x1401f3c00;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = ((r8 as u32)) as u32 as u64; rdi = rdx; rbp = rcx; r15 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = r9; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x28u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); crate::simd_ops::lk_add_u32(rsi, (1) as u32); rdx = rbx; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = rbx; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rbp.wrapping_add(0x18u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 35; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rbx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { bb = 35; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, r15); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rsi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rsi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = rcx; r9 = rsi; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rbx.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = rcx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { bb = 35; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, r15); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 35; continue; }
            34 => { crate::simd_ops::wu8(r9.wrapping_add(0x9u64), (0x1u8) as u8); rdx = rbp; rcx = (crate::simd_ops::ru64(r9)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412add20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14219daa0; const P1: u64 = 0x1402174f0; const P2: u64 = 0x14240aa00; const P3: u64 = 0x1412adc40; const P4: u64 = 0x14240ab10; const P5: u64 = 0x140200070; const P6: u64 = 0x1412adee2;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412add20, funcinfo_rva: 0x3690d00, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x108, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; r15 = (r8) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x80u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412add52); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r12 = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r15)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412add60); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbp = ((rbx.wrapping_add(0x50u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 2; continue; }
            2 => { r14 = (crate::simd_ops::ru64(rax.wrapping_add(0x2b0u64))) as u64; rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rsi = (r14) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rcx = ((rbx.wrapping_add(0x1cu64)) as u64) as u64; rdx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412add97); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            6 => { rsi = (rbx) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdx = ((rsi.wrapping_add(0x1cu64)) as u64) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412addc3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (rsi) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x118u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64; rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x33u64), (0x8u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rdx = ((rdx).wrapping_add(0x8u64)) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ade1d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ade30); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x44u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0xd0u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); rcx = ((rdi.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x118u64), (rcx) as u64); crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x8u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ade78); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xd0u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x50u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rdi.wrapping_add(0x58u64), (0x1u8) as u8); rcx = ((rsp.wrapping_add(0x88u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412ade95); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 16; continue; }
            12 => { crate::simd_ops::wu8(rdi.wrapping_add(0x58u64), (0x0u8) as u8); fl_l = ((r12 as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rdi, (0x58u32) as u32); bb = 16; continue; }
            14 => { crate::simd_ops::wu32(rdi, (0x54u32) as u32); bb = 16; continue; }
            15 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x58u64), (0x0u8) as u8); bb = 16; continue; }
            16 => { rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0xe0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            17 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412adedd); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P6, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412adf00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0x14240aa00; const P4: u64 = 0x50; const P5: u64 = 0x58;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    crate::simd_ops::lk_and_u32(rcx, (0xff000008u32) as u32);
    crate::simd_ops::lk_or_u32(rcx, (0x8u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x8u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8);
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdi.wrapping_add(P5), (0x1u8) as u8);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ae060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1401fb4d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rcx).wrapping_add(0x1cu64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ae0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140267680; const P1: u64 = 0x142409b50; const P2: u64 = 0x1402174f0; const P3: u64 = 0x14240ab10; const P4: u64 = 0x1425371a0; const P5: u64 = 0x142533200; const P6: u64 = 0x140208500; const P7: u64 = 0x140200130;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r12 = (r9) as u64; rbx = (r8) as u64; r15 = (rdx) as u64; rbp = (rcx) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r13) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rsi = ((rax.wrapping_add(0x1cu64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (rsi) as u64); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((r13 as u32)) as u32); rdx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu8(rdx, ((r13 as u8)) as u8); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rdx, (0x0u8) as u8); bb = 7; continue; }
            4 => { r9 = (0xffffffffffffffffu64) as u64; bb = 5; continue; }
            5 => { r9 = ((r9).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r9))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(r12)) as u32) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x28u64), ((rax as u8)) as u8); crate::simd_ops::wu64(r14, (rdi) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rdi) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu16(r14.wrapping_add(0x18u64), (0x0u16) as u16); rdi = (crate::simd_ops::ru64(rbp)) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r13 as u32)) as u32); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); rcx = ((rbx.wrapping_add(0x1cu64)) as u64) as u64; rdx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r13 as u32)) as u32); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; bb = 12; continue; }
            11 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rdi = (rbx) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 14; continue; }
            14 => { rdx = ((rdi.wrapping_add(0x1cu64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 23; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(r15, (rdi) as u64); crate::simd_ops::wu8(r15.wrapping_add(0x8u64), ((rax as u8)) as u8); rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r10 = ((r14.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            19 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            20 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rdx = (rcx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            22 => { bb = 25; continue; }
            23 => { rax = (0x38e38e38e38e38eu64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = (r14) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r15, (rax) as u64); crate::simd_ops::wu8(r15.wrapping_add(0x8u64), (0x1u8) as u8); bb = 25; continue; }
            25 => { rax = (r15) as u64; r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ae660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426455a8; const P1: u64 = 0x140941bd0; const P2: u64 = 0x142645598; const P3: u64 = 0x14128fd40; const P4: u64 = 0x1412a7e60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rdx; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; crate::simd_ops::wu8(rsp.wrapping_add(0x64u64), (0x1u8) as u8); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r12 = 0; fl_l = 0; fl_r = 0; rbx = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r15 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); rcx = rax.wrapping_add(0xa0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { r14 = rbp.wrapping_add(0xc0); fl_l = r14; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(r14, rax); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsi, r12); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), r12); bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbp.wrapping_add(0xa8u64), (0x2u32) as u32); bb = 5; continue; }
            4 => { crate::simd_ops::wu32(rbp.wrapping_add(0xa8u64), ((r12 as u32)) as u32); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x64u64), (0x0u8) as u8); bb = 5; continue; }
            5 => { rcx = rbp.wrapping_add(0xb0); rax = rsp.wrapping_add(0x20); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r12); bb = 7; continue; }
            7 => { rcx = rbp.wrapping_add(0xa0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((r15 as u32)) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rbx.wrapping_add(0x9u64), (0x1u8) as u8); rdx = rsp.wrapping_add(0x60); rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412ae790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1412b0040; const P2: u64 = 0x1401fa0a0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412aea10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412b0b40; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x1421a5b60; const P5: u64 = 0x1401f3c00;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; rdi = rdx; r14 = rcx; r15 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = r9; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x28u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbp as u32)) as u32); crate::simd_ops::lk_add_u32(rsi, (1) as u32); rdx = rbx; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = rbx; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r14.wrapping_add(0x10u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 37; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = r14; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rbx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { bb = 37; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, r15); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rsi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rsi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = rcx; r9 = rsi; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rbx.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = rcx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { bb = 37; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, r15); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 37; continue; }
            34 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x2cu64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 36; continue; }
            36 => { rdx = rsp.wrapping_add(0x28); rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            37 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412aeea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412b1050; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x1412ae660; const P5: u64 = 0x1401f3c00;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = ((r8 as u32)) as u32 as u64; rdi = rdx; rbp = rcx; r15 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = r9; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); crate::simd_ops::lk_add_u32(rsi, (1) as u32); rdx = rbx; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = rbx; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rbp.wrapping_add(0x28u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 35; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rbx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { bb = 35; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, r15); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rsi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rsi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = rcx; r9 = rsi; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rbx.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = rcx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { bb = 35; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, r15); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 35; continue; }
            34 => { rdx = rbp; rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412af170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412b11c0; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x1421a4660; const P5: u64 = 0x14128fc90;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412af170, funcinfo_rva: 0x3690ffc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; rdi = (rdx) as u64; r14 = (rcx) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (0xfffffffffffffffcu64) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = (r9) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af1b7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x28u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rsi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rbp as u32)) as u32); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0xcu64), (1) as u32); rdx = (rbx) as u64; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = (rbx) as u64; fl_l = (rax) as u64 as u64; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r14.wrapping_add(0x10u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, (rsi) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 38; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af215); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi, (rsi) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af225); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rbx.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af28a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af29c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { bb = 38; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, (r15) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af2e4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af2ea); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { r10 = ((rsi.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rsi) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rsi) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af34b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = (rcx) as u64; r9 = (rsi) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af363); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af37f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((rbx.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af3e1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = (rcx) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af3f6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            32 => { bb = 38; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, (r15) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 38; continue; }
            34 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r15 as u8)) as u8); bb = 37; continue; }
            36 => { r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(r9)) as u64; rcx = ((rsp.wrapping_add(0x24u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af42e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 37; continue; }
            37 => { rdx = ((rsp.wrapping_add(0x24u64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412af43b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 38; continue; }
            38 => { rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412afa40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412b1470; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x1421a2990;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412afa40, funcinfo_rva: 0x36910e0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; rdi = (rdx) as u64; r14 = (rcx) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (0xfffffffffffffffcu64) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = (r9) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412afa87); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbp as u32)) as u32); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0xcu64), (1) as u32); rdx = (rbx) as u64; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = (rbx) as u64; fl_l = (rax) as u64 as u64; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r14.wrapping_add(0x10u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, (rsi) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 37; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412afae5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi, (rsi) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412afaf5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rbx.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412afb5a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412afb6c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { bb = 37; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, (r15) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412afbb4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412afbba); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { r10 = ((rsi.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rsi) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rsi) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412afc1b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = (rcx) as u64; r9 = (rsi) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412afc33); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412afc4f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((rbx.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412afcb1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = (rcx) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412afcc6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            32 => { bb = 37; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, (r15) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 37; continue; }
            34 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x2cu64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 36; continue; }
            36 => { r8 = ((rsp.wrapping_add(0x28u64)) as u64) as u64; rcx = (r9) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412afcf9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 37; continue; }
            37 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412afd20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412b1540; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x14128fc90;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r15 = ((r8 as u32)) as u32 as u64; rdi = rdx; r14 = rcx; rbx = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = rbp; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = r9; rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x20u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r15 as u32)) as u32); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0xcu64), (1) as u32); rdx = rbp; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = rbp; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r14.wrapping_add(0x10u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 38; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = r14; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rbx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { bb = 38; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rsi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rsi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = rcx; r9 = rsi; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rbx.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = rcx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { bb = 38; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, rbx); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rbx as u32)) as u32); bb = 38; continue; }
            34 => { rsi = ((crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x2cu64), ((rsi as u8)) as u8); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rax.wrapping_add(0xd0u64), ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rax.wrapping_add(0xd1u64), ((rbx as u8)) as u8); rbx = (rbx & !0xffu64) | (((tmp_xchg) as u8 as u64)); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); bb = 37; continue; }
            36 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rax.wrapping_add(0xd1u64), ((rbx as u8)) as u8); rbx = (rbx & !0xffu64) | (((tmp_xchg) as u8 as u64)); bb = 37; continue; }
            37 => { crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rsi as u8)) as u8); rdx = rsp.wrapping_add(0x30); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            38 => { rax = rdi; r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b0040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412b1610; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x14128fc90;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; rdi = rdx; r14 = rcx; r15 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { rcx = rsp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rbp as u32)) as u32); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0xcu64), (1) as u32); rdx = rbx; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = rbx; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r14.wrapping_add(0x10u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 37; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = r14; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rbx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { bb = 37; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, r15); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rsi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rsi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = rcx; r9 = rsi; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rbx.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = rcx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { bb = 37; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, r15); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 37; continue; }
            34 => { rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rcx as u8)) as u8); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); bb = 36; continue; }
            36 => { rdx = rsp.wrapping_add(0x24); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            37 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b0320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412b16d0; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x14128fc90;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = r9; r15 = ((r8 as u32)) as u32 as u64; rdi = rdx; rbp = rcx; r12 = 0; fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = ((r14 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = r14; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = rbx; rcx = rsp.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x28u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r15 as u32)) as u32); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0xcu64), (1) as u32); rdx = r14; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = r14; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rbp.wrapping_add(0x10u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 41; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rbx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { bb = 41; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, r12); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r12 as u32)) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rsi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rsi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = rcx; r9 = rsi; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rbx.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = rcx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { bb = 41; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, r12); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r12 as u32)) as u32); bb = 41; continue; }
            34 => { rsi = ((crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; rax = (((rsi as u8)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru8(r9.wrapping_add(0xcu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0x1; let tmp_xchg: u8 = crate::simd_ops::atomic_xchg_u8(rax.wrapping_add(0xd0u64), ((rcx as u8)) as u8); rcx = (rcx & !0xffu64) | (((tmp_xchg) as u8 as u64)); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x34u64))) as u32) as u32 as u64; bb = 36; continue; }
            36 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 39; continue; } bb = 37; continue; }
            37 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xcu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 40; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); bb = 40; continue; }
            39 => { crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); bb = 40; continue; }
            40 => { rdx = rsp.wrapping_add(0x24); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 41; continue; }
            41 => { rax = rdi; r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b0650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412b17c0; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x1421a23a0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1412b0650, funcinfo_rva: 0x369123c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = ((r8 as u32)) as u32 as u64; rdi = (rdx) as u64; r14 = (rcx) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = (rbx) as u64 as u64; fl_r = (0xfffffffffffffffcu64) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = (r9) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412b0697); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x28u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rsi) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbp as u32)) as u32); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0xcu64), (1) as u32); rdx = (rbx) as u64; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = (rbx) as u64; fl_l = (rax) as u64 as u64; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(r14.wrapping_add(0x10u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, (rsi) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); bb = 37; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (r14) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412b06f5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi, (rsi) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412b0705); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rbx.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412b076a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412b077c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 12; continue; }
            12 => { bb = 37; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, (r15) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = (0xffffffffu32) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412b07c4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412b07ca); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { r10 = ((rsi.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (rsi) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (rsi) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412b082b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = (rcx) as u64; r9 = (rsi) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412b0843); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412b085f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((rbx.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412b08c1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = (rcx) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412b08d6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            32 => { bb = 37; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, (r15) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 37; continue; }
            34 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x2cu64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 36; continue; }
            36 => { r8 = ((rsp.wrapping_add(0x28u64)) as u64) as u64; rcx = (r9) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1412b0909); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 37; continue; }
            37 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b1950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acda;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rbp = r9; rsi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 1; continue; }
            1 => { rsi = ((rsi).wrapping_add(0x4u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 44; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; rdi = rsi; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r12 = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; rdx = r8.wrapping_add(0x138); rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_add(0x133u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(r8).wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 8; continue; }
            8 => { r15 = (0xffffffffffffffffu64) as u64; bb = 9; continue; }
            9 => { r15 = ((r15).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r15))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 10; continue; }
            10 => { r13 = (((r12) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; r9 = r8.wrapping_add(0x138); r9 = ((r9).wrapping_add(r13)) as u64; rcx = ((crate::simd_ops::ru8(r8.wrapping_add(r13).wrapping_add(0x133u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 13; continue; }
            12 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(r13).wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 15; continue; }
            15 => { r14 = (0xffffffffffffffffu64) as u64; bb = 16; continue; }
            16 => { r14 = ((r14).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r14))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { fl_l = r15; fl_r = r14; r8 = r14; rcx = r9; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { rbx = rsi; bb = 20; continue; }
            20 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rbx = ((rbx).wrapping_sub(0x4u64)) as u64; r12 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = r8.wrapping_add(0x138); rax = (((r12) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_add(0x133u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 27; continue; } bb = 21; continue; }
            21 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 28; continue; }
            22 => { fl_l = r14; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 23; continue; }
            23 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = rsi; rdx = rsi.wrapping_add(0x4); bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x4u64))) as u32 as u64; rdx = rdx.wrapping_sub(0x4); rax = ((rax).wrapping_add(0xfffffffffffffffcu64)) as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu32(rax, ((r12 as u32)) as u32); bb = 42; continue; }
            27 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(r8).wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 30; continue; }
            30 => { r15 = (0xffffffffffffffffu64) as u64; bb = 31; continue; }
            31 => { r15 = ((r15).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r15))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 31; continue; } bb = 32; continue; }
            32 => { rcx = ((crate::simd_ops::ru8(r8.wrapping_add(r13).wrapping_add(0x133u64))) as u32) as u32 as u64; r9 = r8.wrapping_add(0x138); r9 = ((r9).wrapping_add(r13)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 35; continue; }
            34 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(r13).wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 37; continue; }
            37 => { r14 = (0xffffffffffffffffu64) as u64; bb = 38; continue; }
            38 => { r14 = ((r14).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r14))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 38; continue; } bb = 39; continue; }
            39 => { fl_l = r15; fl_r = r14; r8 = r14; rcx = r9; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 45; continue; } bb = 40; continue; }
            40 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 46; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsi = ((rsi).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 43; continue; }
            43 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 44; continue; }
            44 => { crate::simd_ops::wu64(rcx, rax); rax = rcx; rsp = ((rsp).wrapping_add(0x58u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            45 => { fl_l = r14; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 41; continue; } bb = 46; continue; }
            46 => { crate::simd_ops::wu32(rdi, ((r12 as u32)) as u32); rdi = rbx; bb = 20; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b1c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acda; const P1: u64 = 0x1412b3c50;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rdi = r8; rdx = ((rdx).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rax = rcx; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; r15 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rdx); r15 = ((((r15) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r15; fl_r = r15; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 28; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbx); r8 = rdx.wrapping_sub(0x1); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rbp); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r13); r13 = r8; fl_l = ((((r13) as i64).wrapping_shr(1u32) as u64)) as u64; r13 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rax)) as u64; r15 = ((r15).wrapping_sub(1)) as u64; r14 = r15; rcx = r15; rax = (crate::simd_ops::ru32(rbx.wrapping_add(r15.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((rax as u32)) as u32); fl_l = r15; fl_r = r13; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 23; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r12 = ((rcx.wrapping_add(rcx)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; rdx = r8.wrapping_add(0x138); rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_add(0x133u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 6; continue; }
            5 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(r8).wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 8; continue; }
            8 => { rbp = (0xffffffffffffffffu64) as u64; bb = 9; continue; }
            9 => { rbp = ((rbp).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rbp))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; r9 = r8.wrapping_add(0x138); rax = (((rax) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_add(0x133u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 13; continue; }
            12 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(r8).wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 15; continue; }
            15 => { rsi = (0xffffffffffffffffu64) as u64; bb = 16; continue; }
            16 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { fl_l = rbp; fl_r = rsi; r8 = rsi; rcx = r9; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rbp } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = 0xffffffff; bb = 21; continue; }
            20 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rbp; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 21; continue; }
            21 => { rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((((rcx) as i64).wrapping_shr(63u32) as u64)) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; rcx = ((rcx).wrapping_add(r12)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(r14.wrapping_mul(4)), ((rax as u32)) as u32); r14 = rcx; fl_l = rcx; fl_r = r13; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 3; continue; } bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 23; continue; }
            23 => { fl_l = r14; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rdx.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(r14.wrapping_mul(4)), ((rax as u32)) as u32); r14 = r8; bb = 26; continue; }
            26 => { r9 = rsp.wrapping_add(0x88); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); r8 = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdx = r14; rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r8 = rdx.wrapping_sub(0x1); fl_l = r15; fl_r = r15; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 2; continue; } bb = 27; continue; }
            27 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 28; continue; }
            28 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b1e30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412b3de0; const P1: u64 = 0x14183acda;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x68u64)) as u64; r10 = (crate::simd_ops::ru64(rdx)) as u64; r15 = (r9) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rdx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; r9 = (rax) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (r10) as u64); r9 = ((r9).wrapping_sub(r10)) as u64; r8 = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; r9 = ((((r9) as i64).wrapping_shr(3u32) as u64)) as u64; r13 = ((rax.wrapping_sub(0x4u64)) as u64) as u64; rcx = (r13) as u64; rcx = ((rcx).wrapping_sub(r10)) as u64; rdi = ((r10.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rdi) as u64); r9 = (r15) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x28u64) as u64 as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(1)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; r14 = ((rcx.wrapping_mul(8)) as u64) as u64; rsi = ((rcx.wrapping_mul(4)) as u64) as u64; rax = ((r10.wrapping_add(r14)) as u64) as u64; r12 = ((rsi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rax) as u64); rcx = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (r12) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rsi.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rax) as u64); r8 = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; rax = (rdi) as u64; rdx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rax = ((rax).wrapping_sub(rsi)) as u64; rcx = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; r9 = (r15) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (r13) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (r13) as u64); rbx = ((rbx).wrapping_sub(rsi)) as u64; r8 = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; r13 = ((r13).wrapping_sub(r14)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rbx) as u64); r9 = (r15) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (r13) as u64); rdx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (r15) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rbx) as u64); r8 = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (r12) as u64); rdx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; bb = 3; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (r13) as u64); bb = 3; continue; }
            3 => { rcx = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rdi) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; r14 = ((rdi.wrapping_add(0x4u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (r14) as u64); rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 43; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r15 = (rax) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; r9 = ((rdi.wrapping_sub(0x4u64)) as u64) as u64; r8 = (((rax) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (r9) as u64); r8 = ((r8).wrapping_add(r15)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (r8) as u64); rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x133u64))) as u32) as u32 as u64; rdx = ((r8.wrapping_add(0x138u64)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x1cu64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 8; continue; }
            7 => { rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 10; continue; }
            10 => { rsi = (0xffffffffffffffffu64) as u64; bb = 11; continue; }
            11 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(r9)) as u32 as u64; r8 = ((r15.wrapping_add(0x138u64)) as u64) as u64; rax = (((rax) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (r8) as u64); r13 = ((crate::simd_ops::ru8(rax.wrapping_add(r15).wrapping_add(0x133u64))) as u32) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = ((r13.wrapping_mul(8)) as u32) as u32 as u64; bb = 15; continue; }
            14 => { rcx = ((r13 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(r15).wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((rax as u32)) as u32); rcx = (r8) as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(r8)) as u64; bb = 17; continue; }
            17 => { r12 = (0xffffffffffffffffu64) as u64; bb = 18; continue; }
            18 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r12))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 19; continue; }
            19 => { fl_l = (rsi) as u64 as u64; fl_r = (r12) as u64 as u64; r8 = (r12) as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rsi } else { r8 }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 42; continue; } bb = 21; continue; }
            21 => { bb = 23; continue; }
            22 => { fl_l = (r12) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 42; continue; } bb = 23; continue; }
            23 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { r13 = ((r13.wrapping_mul(8)) as u32) as u32 as u64; bb = 26; continue; }
            25 => { r13 = (((r13 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; r13 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x20u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 28; continue; }
            28 => { r12 = (0xffffffffffffffffu64) as u64; bb = 29; continue; }
            29 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r12))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x1cu64))) as u32 as u64; rcx = ((rcx).wrapping_add(0x138u64)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 33; continue; }
            32 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add(0xffff8400u32)) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 35; continue; }
            35 => { rsi = (0xffffffffffffffffu64) as u64; bb = 36; continue; }
            36 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 37; continue; }
            37 => { fl_l = (r12) as u64 as u64; fl_r = (rsi) as u64 as u64; r8 = (rsi) as u64; rdx = (rax) as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { r12 } else { r8 }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 40; continue; } bb = 38; continue; }
            38 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 42; continue; } bb = 39; continue; }
            39 => { bb = 41; continue; }
            40 => { fl_l = (rsi) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 42; continue; } bb = 41; continue; }
            41 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 42; continue; }
            42 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rdi) as u64); bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (r14) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 83; continue; } bb = 44; continue; }
            44 => { r8 = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; r15 = (((rax) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (r8) as u64); r15 = ((r15).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (r15) as u64); r13 = ((crate::simd_ops::ru8(r15.wrapping_add(0x133u64))) as u32) as u32 as u64; rdi = ((r15.wrapping_add(0x138u64)) as u64) as u64; r15 = (crate::simd_ops::ru32(r15.wrapping_add(0x130u64))) as u32 as u64; r15 = ((((r15 as u32)) & (0xffffffu32))) as u32 as u64; fl_l = (((r15 as u32)).wrapping_shl(3u32)) as u32 as u64; r15 = ((fl_l as u32)) as u32 as u64; bb = 45; continue; }
            45 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rax = ((r13.wrapping_mul(8)) as u32) as u32 as u64; bb = 48; continue; }
            47 => { rax = ((r13 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 48; continue; }
            48 => { rdx = (rdi) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 50; continue; } bb = 49; continue; }
            49 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; bb = 50; continue; }
            50 => { rsi = (0xffffffffffffffffu64) as u64; bb = 51; continue; }
            51 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 51; continue; } bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(r8)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rax) as u64); r13 = ((crate::simd_ops::ru8(rax.wrapping_add(0x133u64))) as u32) as u32 as u64; r9 = ((rax.wrapping_add(0x138u64)) as u64) as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 54; continue; } bb = 53; continue; }
            53 => { rcx = ((r13.wrapping_mul(8)) as u32) as u32 as u64; bb = 55; continue; }
            54 => { rcx = ((r13 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 55; continue; }
            55 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 57; continue; } bb = 56; continue; }
            56 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 57; continue; }
            57 => { r12 = (0xffffffffffffffffu64) as u64; bb = 58; continue; }
            58 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r12))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 58; continue; } bb = 59; continue; }
            59 => { fl_l = (rsi) as u64 as u64; fl_r = (r12) as u64 as u64; r8 = (r12) as u64; rcx = (r9) as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rsi } else { r8 }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 62; continue; } bb = 60; continue; }
            60 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 82; continue; } bb = 61; continue; }
            61 => { bb = 63; continue; }
            62 => { fl_l = (r12) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 82; continue; } bb = 63; continue; }
            63 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; rdx = ((rdx).wrapping_add(0x138u64)) as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 65; continue; } bb = 64; continue; }
            64 => { r13 = ((r13.wrapping_mul(8)) as u32) as u32 as u64; bb = 66; continue; }
            65 => { r13 = (((r13 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = (((r13 as u32)).wrapping_add(0xffff8400u32)) as u32 as u64; fl_r = fl_l; r13 = ((fl_l as u32)) as u32 as u64; bb = 66; continue; }
            66 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 68; continue; } bb = 67; continue; }
            67 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 68; continue; }
            68 => { r12 = (0xffffffffffffffffu64) as u64; bb = 69; continue; }
            69 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r12))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 69; continue; } bb = 70; continue; }
            70 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; r13 = ((crate::simd_ops::ru8(rax.wrapping_add(0x133u64))) as u32) as u32 as u64; rcx = ((rax.wrapping_add(0x138u64)) as u64) as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 72; continue; } bb = 71; continue; }
            71 => { rax = ((r13.wrapping_mul(8)) as u32) as u32 as u64; bb = 73; continue; }
            72 => { rax = ((r13 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(0xffff8400u32)) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 73; continue; }
            73 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 75; continue; } bb = 74; continue; }
            74 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 75; continue; }
            75 => { rsi = (0xffffffffffffffffu64) as u64; bb = 76; continue; }
            76 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 76; continue; } bb = 77; continue; }
            77 => { fl_l = (r12) as u64 as u64; fl_r = (rsi) as u64 as u64; r8 = (rsi) as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { r12 } else { r8 }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 80; continue; } bb = 78; continue; }
            78 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 82; continue; } bb = 79; continue; }
            79 => { bb = 81; continue; }
            80 => { fl_l = (rsi) as u64 as u64; fl_r = (r12) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 82; continue; } bb = 81; continue; }
            81 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; r14 = ((r14).wrapping_add(0x4u64)) as u64; fl_l = (r14) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 45; continue; } bb = 82; continue; }
            82 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (r14) as u64); bb = 83; continue; }
            83 => { rbx = (r14) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rdi) as u64); rsi = (rdi) as u64; bb = 84; continue; }
            84 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rbx) as u64); bb = 85; continue; }
            85 => { fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax)) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 128; continue; } bb = 86; continue; }
            86 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; bb = 87; continue; }
            87 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); rax = (((rax) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; r10 = ((rdx.wrapping_add(0x138u64)) as u64) as u64; r10 = ((r10).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (r10) as u64); rcx = ((crate::simd_ops::ru8(rax.wrapping_add(rdx).wrapping_add(0x133u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x1cu64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 89; continue; } bb = 88; continue; }
            88 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 90; continue; }
            89 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 90; continue; }
            90 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(rdx).wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 92; continue; } bb = 91; continue; }
            91 => { r10 = (crate::simd_ops::ru64(r10)) as u64; bb = 92; continue; }
            92 => { r13 = (0xffffffffffffffffu64) as u64; bb = 93; continue; }
            93 => { r13 = ((r13).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r13))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 93; continue; } bb = 94; continue; }
            94 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (rax) as u64); rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x133u64))) as u32) as u32 as u64; r9 = ((rax.wrapping_add(0x138u64)) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 96; continue; } bb = 95; continue; }
            95 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 97; continue; }
            96 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 97; continue; }
            97 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(0x130u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), ((rcx as u32)) as u32); rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 99; continue; } bb = 98; continue; }
            98 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 99; continue; }
            99 => { r12 = (0xffffffffffffffffu64) as u64; bb = 100; continue; }
            100 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r12))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 100; continue; } bb = 101; continue; }
            101 => { fl_l = (r13) as u64 as u64; fl_r = (r12) as u64 as u64; r8 = (r12) as u64; rdx = (r10) as u64; rcx = (r9) as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { r13 } else { r8 }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 104; continue; } bb = 102; continue; }
            102 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 126; continue; } bb = 103; continue; }
            103 => { bb = 105; continue; }
            104 => { fl_l = (r12) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 126; continue; } bb = 105; continue; }
            105 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32 as u64; rdx = ((rdx).wrapping_add(0x138u64)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 107; continue; } bb = 106; continue; }
            106 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 108; continue; }
            107 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 108; continue; }
            108 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 110; continue; } bb = 109; continue; }
            109 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 110; continue; }
            110 => { r13 = (0xffffffffffffffffu64) as u64; bb = 111; continue; }
            111 => { r13 = ((r13).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r13))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 111; continue; } bb = 112; continue; }
            112 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x1cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 114; continue; } bb = 113; continue; }
            113 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 115; continue; }
            114 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 115; continue; }
            115 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x20u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 117; continue; } bb = 116; continue; }
            116 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 117; continue; }
            117 => { r12 = (0xffffffffffffffffu64) as u64; bb = 118; continue; }
            118 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r12))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 118; continue; } bb = 119; continue; }
            119 => { fl_l = (r13) as u64 as u64; fl_r = (r12) as u64 as u64; r8 = (r12) as u64; rcx = (rax) as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { r13 } else { r8 }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 122; continue; } bb = 120; continue; }
            120 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 127; continue; } bb = 121; continue; }
            121 => { bb = 123; continue; }
            122 => { fl_l = (r12) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 127; continue; } bb = 123; continue; }
            123 => { fl_l = (r14) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 125; continue; } bb = 124; continue; }
            124 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; crate::simd_ops::wu32(r14, ((rdx as u32)) as u32); crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); bb = 125; continue; }
            125 => { fl_l = ((r14).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 126; continue; }
            126 => { rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi)) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 87; continue; } bb = 127; continue; }
            127 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (r14) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (rbx) as u64); bb = 128; continue; }
            128 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 170; continue; } bb = 129; continue; }
            129 => { rbx = ((rsi.wrapping_sub(0x4u64)) as u64) as u64; r14 = (rax) as u64; bb = 130; continue; }
            130 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; r10 = ((rdx.wrapping_add(0x138u64)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), ((r8 as u32)) as u32); r10 = ((r10).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), (r10) as u64); rcx = ((crate::simd_ops::ru8(rax.wrapping_add(rdx).wrapping_add(0x133u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x1cu64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 132; continue; } bb = 131; continue; }
            131 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 133; continue; }
            132 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 133; continue; }
            133 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(rdx).wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 135; continue; } bb = 134; continue; }
            134 => { r10 = (crate::simd_ops::ru64(r10)) as u64; bb = 135; continue; }
            135 => { r13 = (0xffffffffffffffffu64) as u64; bb = 136; continue; }
            136 => { r13 = ((r13).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r10.wrapping_add(r13))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 136; continue; } bb = 137; continue; }
            137 => { rax = (((r8) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x133u64))) as u32) as u32 as u64; r9 = ((rax.wrapping_add(0x138u64)) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 139; continue; } bb = 138; continue; }
            138 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 140; continue; }
            139 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 140; continue; }
            140 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(0x130u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(24u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), ((rcx as u32)) as u32); rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 142; continue; } bb = 141; continue; }
            141 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 142; continue; }
            142 => { r12 = (0xffffffffffffffffu64) as u64; bb = 143; continue; }
            143 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r12))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 143; continue; } bb = 144; continue; }
            144 => { fl_l = (r13) as u64 as u64; fl_r = (r12) as u64 as u64; r8 = (r12) as u64; rdx = (r10) as u64; rcx = (r9) as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { r13 } else { r8 }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 147; continue; } bb = 145; continue; }
            145 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 168; continue; } bb = 146; continue; }
            146 => { bb = 148; continue; }
            147 => { fl_l = (r12) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 168; continue; } bb = 148; continue; }
            148 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32 as u64; rdx = ((rdx).wrapping_add(0x138u64)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 150; continue; } bb = 149; continue; }
            149 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 151; continue; }
            150 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 151; continue; }
            151 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 153; continue; } bb = 152; continue; }
            152 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 153; continue; }
            153 => { r13 = (0xffffffffffffffffu64) as u64; bb = 154; continue; }
            154 => { r13 = ((r13).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r13))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 154; continue; } bb = 155; continue; }
            155 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x1cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 157; continue; } bb = 156; continue; }
            156 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 158; continue; }
            157 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 158; continue; }
            158 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x20u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 160; continue; } bb = 159; continue; }
            159 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 160; continue; }
            160 => { r12 = (0xffffffffffffffffu64) as u64; bb = 161; continue; }
            161 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r12))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 161; continue; } bb = 162; continue; }
            162 => { fl_l = (r13) as u64 as u64; fl_r = (r12) as u64 as u64; r8 = (r12) as u64; rcx = (rax) as u64; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { r13 } else { r8 }) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 165; continue; } bb = 163; continue; }
            163 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 169; continue; } bb = 164; continue; }
            164 => { bb = 166; continue; }
            165 => { fl_l = (r12) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 169; continue; } bb = 166; continue; }
            166 => { rdi = ((rdi).wrapping_sub(0x4u64)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 168; continue; } bb = 167; continue; }
            167 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; crate::simd_ops::wu32(rdi, ((rcx as u32)) as u32); crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); bb = 168; continue; }
            168 => { rsi = ((rsi).wrapping_sub(0x4u64)) as u64; rbx = ((rbx).wrapping_sub(0x4u64)) as u64; fl_l = (crate::simd_ops::ru64(r14)) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 130; continue; } bb = 169; continue; }
            169 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rsi) as u64); bb = 170; continue; }
            170 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx)) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 175; continue; } bb = 171; continue; }
            171 => { fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 180; continue; } bb = 172; continue; }
            172 => { fl_l = (r14) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 174; continue; } bb = 173; continue; }
            173 => { rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); crate::simd_ops::wu32(r14, ((rcx as u32)) as u32); bb = 174; continue; }
            174 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; r14 = ((r14).wrapping_add(0x4u64)) as u64; rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rdi = ((rdi).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); fl_l = ((rbx).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (r14) as u64); bb = 84; continue; }
            175 => { rsi = ((rsi).wrapping_add(0xfffffffffffffffcu64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rsi) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 179; continue; } bb = 176; continue; }
            176 => { rdi = ((rdi).wrapping_sub(0x4u64)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 178; continue; } bb = 177; continue; }
            177 => { rcx = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); crate::simd_ops::wu32(rdi, ((rcx as u32)) as u32); bb = 178; continue; }
            178 => { rax = (crate::simd_ops::ru32(r14.wrapping_sub(0x4u64))) as u32 as u64; fl_l = (r14) as u64 as u64; fl_r = (0x4u64) as u64 as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(r14, ((rcx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (r14) as u64); bb = 85; continue; }
            179 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); fl_l = ((rbx).wrapping_add(0x4u64)) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rsi, ((rcx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rsi) as u64); bb = 84; continue; }
            180 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rax, (rdi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (r14) as u64); rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b27d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acda;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rbp = r9; rsi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 1; continue; }
            1 => { rsi = ((rsi).wrapping_add(0x4u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; rdi = rsi; r12 = (crate::simd_ops::ru32(rsi)) as u32 as u64; r15 = (0xffffffffffffffffu64) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r13 = ((r12.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(r13))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 4; continue; }
            4 => { r15 = ((r15).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r15))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { r14 = (0xffffffffffffffffu64) as u64; bb = 6; continue; }
            6 => { r14 = ((r14).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r14))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = r14; fl_r = r15; r8 = r15; rdx = r9; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { r14 } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 8; continue; }
            8 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 21; continue; } bb = 9; continue; }
            9 => { rbx = rsi; bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = (0x4u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r15 = (0xffffffffffffffffu64) as u64; r12 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r12.wrapping_mul(8)))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r13))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 11; continue; }
            11 => { r15 = ((r15).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r15))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 12; continue; }
            12 => { r14 = (0xffffffffffffffffu64) as u64; bb = 13; continue; }
            13 => { r14 = ((r14).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r14))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 14; continue; }
            14 => { fl_l = r14; fl_r = r15; r8 = r15; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { r14 } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 15; continue; }
            15 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 26; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsi = ((rsi).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rcx, rax); rax = rcx; rsp = ((rsp).wrapping_add(0x58u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            20 => { fl_l = r15; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 21; continue; }
            21 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rax = rsi; rdx = rsi.wrapping_add(0x4); bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru32(rax.wrapping_sub(0x4u64))) as u32 as u64; rdx = rdx.wrapping_sub(0x4); rax = ((rax).wrapping_add(0xfffffffffffffffcu64)) as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu32(rax, ((r12 as u32)) as u32); bb = 17; continue; }
            25 => { fl_l = r15; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu32(rdi, ((r12 as u32)) as u32); rdi = rbx; bb = 10; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b29a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acda;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = r8; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; rax = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; rdx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 29; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbx); r9 = rax.wrapping_sub(0x1); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); r15 = r9; fl_l = ((((r15) as i64).wrapping_shr(1u32) as u64)) as u64; r15 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rdx).wrapping_sub(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rdx); r13 = rdx; rcx = rdx; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((r8 as u32)) as u32); fl_l = rdx; fl_r = r15; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 14; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r14 = ((rcx.wrapping_add(rcx)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; rbp = (0xffffffffffffffffu64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; bb = 5; continue; }
            5 => { rbp = ((rbp).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbp))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rsi = (0xffffffffffffffffu64) as u64; bb = 7; continue; }
            7 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 8; continue; }
            8 => { fl_l = rsi; fl_r = rbp; r8 = rbp; rdx = r9; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rsi } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rbp; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = 0xffffffff; bb = 12; continue; }
            11 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = rbp; fl_r = rsi; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 12; continue; }
            12 => { rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((((rcx) as i64).wrapping_shr(63u32) as u64)) as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32)) as u32); r13 = rcx; fl_l = rcx; fl_r = r15; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 14; continue; }
            14 => { fl_l = r13; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32)) as u32); r13 = r9; bb = 17; continue; }
            17 => { r14 = r13.wrapping_sub(0x1); r14 = ((((r14) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = rdx; fl_r = r13; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 27; continue; } bb = 18; continue; }
            18 => { rax = ((r8 as u32)) as u32 as u64; r12 = ((rax.wrapping_mul(8)) as u64) as u64; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbp = (0xffffffffffffffffu64) as u64; r15 = (crate::simd_ops::ru32(rbx.wrapping_add(r14.wrapping_mul(4)))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r12))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r15.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 20; continue; }
            20 => { rbp = ((rbp).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rbp))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 21; continue; }
            21 => { rsi = (0xffffffffffffffffu64) as u64; bb = 22; continue; }
            22 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 23; continue; }
            23 => { fl_l = rsi; fl_r = rbp; r8 = rbp; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rsi } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 24; continue; }
            24 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 31; continue; } bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 26; continue; }
            26 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r9 = rax.wrapping_sub(0x1); fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 2; continue; } bb = 28; continue; }
            28 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 29; continue; }
            29 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            30 => { fl_l = rbp; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 25; continue; } bb = 31; continue; }
            31 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(r13.wrapping_mul(4)), ((r15 as u32)) as u32); r13 = r14; r14 = ((r14).wrapping_sub(1)) as u64; r14 = ((((r14) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = rdx; fl_r = r13; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 19; continue; } bb = 32; continue; }
            32 => { bb = 26; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b2bf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412b3e80; const P1: u64 = 0x14183acda;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x58u64)) as u64; r10 = (crate::simd_ops::ru64(rdx)) as u64; r13 = r9; rax = (crate::simd_ops::ru64(r8)) as u64; rdx = rbp.wrapping_sub(0x28); r9 = rax; r9 = ((r9).wrapping_sub(r10)) as u64; r9 = ((((r9) as i64).wrapping_shr(3u32) as u64)) as u64; r12 = rax.wrapping_sub(0x4); rcx = r12; rcx = ((rcx).wrapping_sub(r10)) as u64; rbx = ((r10.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; rcx = ((((rcx) as i64).wrapping_shr(2u32) as u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rbx); r9 = r13; fl_l = rcx; fl_r = (0x28u64) as u64; if ((((fl_l as u64)) as i64) <= (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), r10); rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; r8 = rbp.wrapping_sub(0x30); r14 = ((rcx.wrapping_mul(8)) as u64) as u64; rsi = ((rcx.wrapping_mul(4)) as u64) as u64; rax = ((r10.wrapping_add(r14)) as u64) as u64; r15 = ((rsi.wrapping_add(r10)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); rcx = rbp.wrapping_sub(0x20); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), r15); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rsi.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); r8 = rbp.wrapping_sub(0x20); rax = rbx; rdx = rbp.wrapping_sub(0x28); rax = ((rax).wrapping_sub(rsi)) as u64; rcx = rbp.wrapping_sub(0x30); r9 = r13; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = r12; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), r12); rdi = ((rdi).wrapping_sub(rsi)) as u64; r8 = rbp.wrapping_sub(0x20); r12 = ((r12).wrapping_sub(r14)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rdi); r9 = r13; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), r12); rdx = rbp.wrapping_sub(0x28); rcx = rbp.wrapping_sub(0x30); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = r13; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rdi); rdx = rbp.wrapping_sub(0x28); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), r15); bb = 3; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), r10); bb = 3; continue; }
            3 => { r8 = rbp.wrapping_sub(0x20); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rbx); rcx = rbp.wrapping_sub(0x30); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; r14 = rbx.wrapping_add(0x4); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), r14); fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 4; continue; }
            4 => { r14 = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; r13 = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 5; continue; }
            5 => { rax = ((r13 as u32)) as u32 as u64; rsi = (0xffffffffffffffffu64) as u64; r13 = (crate::simd_ops::ru32(rbx.wrapping_sub(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = rbx.wrapping_sub(0x4); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rax = (crate::simd_ops::ru64(r14.wrapping_add(r13.wrapping_mul(8)))) as u64; r12 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 6; continue; }
            6 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r15.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rdi = (0xffffffffffffffffu64) as u64; bb = 8; continue; }
            8 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r12.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 9; continue; }
            9 => { fl_l = rdi; fl_r = rsi; r8 = rsi; rdx = r12; rcx = r15; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 11; continue; }
            11 => { bb = 13; continue; }
            12 => { fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 13; continue; }
            13 => { rsi = (0xffffffffffffffffu64) as u64; bb = 14; continue; }
            14 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r12.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rdi = (0xffffffffffffffffu64) as u64; bb = 16; continue; }
            16 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r15.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { fl_l = rdi; fl_r = rsi; r8 = rsi; rdx = r15; rcx = r12; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 19; continue; }
            19 => { bb = 21; continue; }
            20 => { fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 22; continue; }
            22 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rbx); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; fl_l = r14; fl_r = (crate::simd_ops::ru64(rax)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 43; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(rax.wrapping_mul(8)))) as u64; r12 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rsi = (0xffffffffffffffffu64) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(rax.wrapping_mul(8)))) as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; bb = 26; continue; }
            26 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r15.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 27; continue; }
            27 => { rdi = (0xffffffffffffffffu64) as u64; bb = 28; continue; }
            28 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r12.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 29; continue; }
            29 => { fl_l = rdi; fl_r = rsi; r8 = rsi; rdx = r12; rcx = r15; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 32; continue; } bb = 30; continue; }
            30 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 42; continue; } bb = 31; continue; }
            31 => { bb = 33; continue; }
            32 => { fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 42; continue; } bb = 33; continue; }
            33 => { rsi = (0xffffffffffffffffu64) as u64; bb = 34; continue; }
            34 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r12.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 35; continue; }
            35 => { rdi = (0xffffffffffffffffu64) as u64; bb = 36; continue; }
            36 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r15.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 37; continue; }
            37 => { fl_l = rdi; fl_r = rsi; r8 = rsi; rdx = r15; rcx = r12; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 40; continue; } bb = 38; continue; }
            38 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 42; continue; } bb = 39; continue; }
            39 => { bb = 41; continue; }
            40 => { fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 42; continue; } bb = 41; continue; }
            41 => { r14 = ((r14).wrapping_add(0x4u64)) as u64; fl_l = r14; fl_r = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 42; continue; }
            42 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; bb = 43; continue; }
            43 => { rdi = r14; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rbx); r15 = rbx; bb = 44; continue; }
            44 => { fl_l = rdi; fl_r = (crate::simd_ops::ru64(rax)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 67; continue; } bb = 45; continue; }
            45 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; r12 = (0xffffffffffffffffu64) as u64; rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rdx); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); bb = 47; continue; }
            47 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r12))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 47; continue; } bb = 48; continue; }
            48 => { rsi = (0xffffffffffffffffu64) as u64; bb = 49; continue; }
            49 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 49; continue; } bb = 50; continue; }
            50 => { fl_l = rsi; fl_r = r12; r8 = r12; rcx = rax; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rsi } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 53; continue; } bb = 51; continue; }
            51 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 65; continue; } bb = 52; continue; }
            52 => { bb = 54; continue; }
            53 => { fl_l = r12; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 65; continue; } bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; r12 = (0xffffffffffffffffu64) as u64; bb = 55; continue; }
            55 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r12))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 55; continue; } bb = 56; continue; }
            56 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rsi = (0xffffffffffffffffu64) as u64; bb = 57; continue; }
            57 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 57; continue; } bb = 58; continue; }
            58 => { fl_l = rsi; fl_r = r12; r8 = r12; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rsi } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 61; continue; } bb = 59; continue; }
            59 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 66; continue; } bb = 60; continue; }
            60 => { bb = 62; continue; }
            61 => { fl_l = r12; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 66; continue; } bb = 62; continue; }
            62 => { fl_l = r14; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 64; continue; } bb = 63; continue; }
            63 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; crate::simd_ops::wu32(r14, ((rdx as u32)) as u32); crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); bb = 64; continue; }
            64 => { fl_l = ((r14).wrapping_add(0x4u64)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 65; continue; }
            65 => { rdi = ((rdi).wrapping_add(0x4u64)) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(r15)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 46; continue; } bb = 66; continue; }
            66 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; bb = 67; continue; }
            67 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rax)) as u64; fl_l = rsi; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 77; continue; } bb = 68; continue; }
            68 => { r8 = r15.wrapping_sub(0x4); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), r8); bb = 69; continue; }
            69 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), r9); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rcx); bb = 70; continue; }
            70 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 70; continue; } bb = 71; continue; }
            71 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); r12 = (0xffffffffffffffffu64) as u64; bb = 72; continue; }
            72 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r12))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 72; continue; } bb = 73; continue; }
            73 => { fl_l = r12; fl_r = rax; r8 = rax; rdx = r9; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { r12 } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 82; continue; } bb = 74; continue; }
            74 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 83; continue; } bb = 75; continue; }
            75 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; bb = 76; continue; }
            76 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; r15 = ((r15).wrapping_sub(0x4u64)) as u64; r8 = ((r8).wrapping_sub(0x4u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r15); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), r8); rsi = (crate::simd_ops::ru64(rax)) as u64; fl_l = rsi; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 69; continue; } bb = 77; continue; }
            77 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = r15; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 93; continue; } bb = 78; continue; }
            78 => { fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 98; continue; } bb = 79; continue; }
            79 => { fl_l = r14; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 81; continue; } bb = 80; continue; }
            80 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu32(r14, ((rcx as u32)) as u32); bb = 81; continue; }
            81 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; r14 = ((r14).wrapping_add(0x4u64)) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rdi, ((rcx as u32)) as u32); fl_l = ((rdi).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 44; continue; }
            82 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 75; continue; } bb = 83; continue; }
            83 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; rax = (0xffffffffffffffffu64) as u64; bb = 84; continue; }
            84 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 84; continue; } bb = 85; continue; }
            85 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; r12 = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); bb = 86; continue; }
            86 => { r12 = ((r12).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r12))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 86; continue; } bb = 87; continue; }
            87 => { fl_l = r12; fl_r = rax; r8 = rax; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { r12 } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 90; continue; } bb = 88; continue; }
            88 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 77; continue; } bb = 89; continue; }
            89 => { bb = 91; continue; }
            90 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 77; continue; } bb = 91; continue; }
            91 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rbx = ((rbx).wrapping_sub(0x4u64)) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 76; continue; } bb = 92; continue; }
            92 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); crate::simd_ops::wu32(r8, ((rax as u32)) as u32); bb = 76; continue; }
            93 => { r15 = ((r15).wrapping_add(0xfffffffffffffffcu64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r15); fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 97; continue; } bb = 94; continue; }
            94 => { rbx = ((rbx).wrapping_sub(0x4u64)) as u64; fl_l = r15; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 96; continue; } bb = 95; continue; }
            95 => { rcx = (crate::simd_ops::ru32(r15)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(r15, ((rax as u32)) as u32); crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); bb = 96; continue; }
            96 => { rax = (crate::simd_ops::ru32(r14.wrapping_sub(0x4u64))) as u32 as u64; fl_l = r14; fl_r = (0x4u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(r14, ((rcx as u32)) as u32); bb = 44; continue; }
            97 => { rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); fl_l = ((rdi).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(r15, ((rcx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r15); bb = 44; continue; }
            98 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r14); rsp = ((rsp).wrapping_add(0x58u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b3260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acda; const P1: u64 = 0x1412b3c50;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = r8; r10 = rdx; crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), r15); rbp = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdx); r8 = rdx; r13 = rcx; rdx = rax.wrapping_sub(0x1); r15 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdx); r15 = ((((r15) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r10; fl_r = r15; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 22; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r14); bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r12 = ((r10.wrapping_add(r10)) as u64) as u64; r14 = (crate::simd_ops::ru64(r13)) as u64; rdx = r8.wrapping_add(0x138); rax = (crate::simd_ops::ru32(r14.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_add(0x133u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(r8).wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 7; continue; }
            7 => { rsi = (0xffffffffffffffffu64) as u64; bb = 8; continue; }
            8 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; r9 = r8.wrapping_add(0x138); rax = (((rax) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_add(0x133u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(r8).wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 14; continue; }
            14 => { rdi = (0xffffffffffffffffu64) as u64; bb = 15; continue; }
            15 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 16; continue; }
            16 => { fl_l = rsi; fl_r = rdi; r8 = rdi; rcx = r9; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rsi } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = 0xffffffff; bb = 20; continue; }
            19 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rsi; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 20; continue; }
            20 => { r10 = ((((rax as u32)) as i32 as i64) as u64) as u64; r10 = ((((r10) as i64).wrapping_shr(63u32) as u64)) as u64; r10 = ((r10).wrapping_add(0x2u64)) as u64; r10 = ((r10).wrapping_add(r12)) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(r10.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(rbp.wrapping_mul(4)), ((rax as u32)) as u32); rbp = r10; fl_l = r10; fl_r = r15; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 21; continue; }
            21 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 22; continue; }
            22 => { fl_l = r10; fl_r = r15; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(r13)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rbp.wrapping_mul(4)), ((rax as u32)) as u32); rbp = rdx; bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(r13)) as u64; rcx = rsp.wrapping_add(0x90); rdx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b3460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acda;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdx); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r13 = r8; r8 = ((r8).wrapping_sub(1)) as u64; r9 = ((r9).wrapping_add(0x8u64)) as u64; rbp = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r9); rbp = ((((rbp) as i64).wrapping_shr(1u32) as u64)) as u64; r14 = rdx; r12 = rcx; fl_l = rdx; fl_r = rbp; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 12; continue; } bb = 1; continue; }
            1 => { r13 = r9; bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(r12)) as u64; r15 = ((rdx.wrapping_add(rdx)) as u64) as u64; rdx = (crate::simd_ops::ru64(r13)) as u64; rdi = (0xffffffffffffffffu64) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(r15.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(r15.wrapping_mul(4)).wrapping_add(0x8u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; bb = 3; continue; }
            3 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rbx = (0xffffffffffffffffu64) as u64; bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rdi; r8 = rdi; rdx = r9; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rbx } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0xffffffff; bb = 10; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rbx; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) > ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 10; continue; }
            10 => { rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((((rdx) as i64).wrapping_shr(63u32) as u64)) as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; rdx = ((rdx).wrapping_add(r15)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(r14.wrapping_mul(4)), ((rax as u32)) as u32); r14 = rdx; fl_l = rdx; fl_r = rbp; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 11; continue; }
            11 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r8 = r13.wrapping_sub(0x1); bb = 13; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r9); bb = 13; continue; }
            13 => { fl_l = rdx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = ((r13 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(r12)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(r13.wrapping_mul(4)).wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(r14.wrapping_mul(4)), ((rax as u32)) as u32); r14 = r8; bb = 16; continue; }
            16 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = r14.wrapping_sub(0x1); rbx = (crate::simd_ops::ru64(r12)) as u64; rbp = ((((rbp) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r13; fl_r = r14; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 27; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(r10)) as u32 as u64; r12 = ((rax.wrapping_mul(8)) as u64) as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(r9)) as u64; rsi = (0xffffffffffffffffu64) as u64; r15 = (crate::simd_ops::ru32(rbx.wrapping_add(rbp.wrapping_mul(4)))) as u32 as u64; rax = (crate::simd_ops::ru64(r12.wrapping_add(rcx))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r15.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 19; continue; }
            19 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 20; continue; }
            20 => { rdi = (0xffffffffffffffffu64) as u64; bb = 21; continue; }
            21 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 22; continue; }
            22 => { fl_l = rdi; fl_r = rsi; r8 = rsi; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { bb = 26; continue; }
            25 => { fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(r14.wrapping_mul(4)), ((r15 as u32)) as u32); r14 = rbp; rbp = ((rbp).wrapping_sub(1)) as u64; rbp = ((((rbp) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r13; fl_r = r14; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 18; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(r14.wrapping_mul(4)), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b3760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1412b3f20; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x142442de0; const P4: u64 = 0x14128fd40; const P5: u64 = 0x1401f3c00;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = ((r8 as u32)) as u32 as u64; rdi = rdx; rbp = rcx; r15 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = (0xfffffffffffffffcu64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { r8 = r9; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x28u64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); crate::simd_ops::lk_add_u32(rsi, (1) as u32); rdx = rbx; rdx = ((((rdx as u32)) & (0x3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = (((rdx) | (rcx))) as u64; rax = rbx; fl_l = rax; let cx_src: u64 = rdx; let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u64(rbp.wrapping_add(0x10u64), fl_l as u64, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u64)) as u64; } if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 35; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rbx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { bb = 35; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); crate::simd_ops::wu64(rdi, r15); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r15 as u32)) as u32); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rsi; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rsi); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            21 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            22 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 23; continue; }
            23 => { rdx = rcx; r9 = rsi; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_sub(1)) as u64; rbx = (((rbx) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rbx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rbx.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            29 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            30 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rdx = rcx; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { bb = 35; continue; }
            33 => { xmm0 = 0u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu64(rdx, r15); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); bb = 35; continue; }
            34 => { crate::simd_ops::wu8(r9.wrapping_add(0x9u64), (0x1u8) as u8); rdx = rbp; rcx = (crate::simd_ops::ru64(r9)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b3a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140941bd0; const P1: u64 = 0x14240ab10; const P2: u64 = 0x1425371a0; const P3: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x10u64))) as u32) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x18u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdi, rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx, rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rsi); bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rax as u8)) as u8); bb = 14; continue; }
            6 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rax as u8)) as u8); bb = 14; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rcx, rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rsi); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx, rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rsi); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rax as u8)) as u8); bb = 14; continue; }
            9 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rax as u8)) as u8); bb = 14; continue; }
            10 => { crate::simd_ops::wu8(rcx.wrapping_add(0x10u64), ((rax as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rcx, rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rsi); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdx, rsi); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rsi); bb = 13; continue; }
            12 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); bb = 13; continue; }
            13 => { crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x1u8) as u8); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 27; continue; } bb = 15; continue; }
            15 => { r14 = rbx.wrapping_add(0x8); rdi = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rbp = (((rax) as i64).wrapping_mul((0x4cu64) as i64) as u64) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x4cu64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 17; continue; }
            17 => { r14 = rbx.wrapping_add(0x8); bb = 18; continue; }
            18 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 19; continue; }
            19 => { r10 = r9.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r14 = rbx.wrapping_add(0x8); bb = 26; continue; }
            23 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { r14 = rbx.wrapping_add(0x8); bb = 26; continue; }
            25 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = rcx; rcx = r10; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rbx, rsi); crate::simd_ops::wu32(r14, ((rsi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rsi as u32)) as u32); bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412b3c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acda;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rsi = rdx.wrapping_sub(0x1); r12 = r8; rsi = ((((rsi) as i64).wrapping_shr(1u32) as u64)) as u64; rbp = rdx; r13 = rcx; fl_l = r8; fl_r = rdx; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 22; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r9)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = (((rax) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r14 = (crate::simd_ops::ru64(r13)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r15 = (crate::simd_ops::ru32(r14.wrapping_add(rsi.wrapping_mul(4)))) as u32 as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_add(0x133u64))) as u32) as u32 as u64; rdx = r8.wrapping_add(0x138); rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(r8).wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 7; continue; }
            7 => { rdi = (0xffffffffffffffffu64) as u64; bb = 8; continue; }
            8 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rdi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rax = (((r15) as i64).wrapping_mul((0x2a8u64) as i64) as u64) as u64; r9 = r8.wrapping_add(0x138); r9 = ((r9).wrapping_add(rax)) as u64; rcx = ((crate::simd_ops::ru8(rax.wrapping_add(r8).wrapping_add(0x133u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 12; continue; }
            11 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(r8).wrapping_add(0x130u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { r9 = (crate::simd_ops::ru64(r9)) as u64; bb = 14; continue; }
            14 => { rbx = (0xffffffffffffffffu64) as u64; bb = 15; continue; }
            15 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 16; continue; }
            16 => { fl_l = rdi; fl_r = rbx; r8 = rbx; rcx = r9; r8 = (if (((fl_l as u64)) < ((fl_r as u64))) { rdi } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { bb = 20; continue; }
            19 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(r14.wrapping_add(rbp.wrapping_mul(4)), ((r15 as u32)) as u32); rbp = rsi; rsi = ((rsi).wrapping_sub(1)) as u64; rsi = ((((rsi) as i64).wrapping_shr(1u32) as u64)) as u64; fl_l = r12; fl_r = rbp; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 21; continue; }
            21 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru64(r13)) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(rbp.wrapping_mul(4)), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x38u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412cf210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14340e7d0; const P1: u64 = 0x14240ab10; const P2: u64 = 0x141834d30;
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
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xc0; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412d0b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x1426455a8; const P3: u64 = 0x142646b18; const P4: u64 = 0x142645598; const P5: u64 = 0x1421f2a10; const P6: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = ((rax as u32)) as u32 as u64; rdx = P1; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x11cu64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 3; continue; }
            3 => { rcx = rdi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rcx = rsi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; r8 = r14.wrapping_add(0x20); fl_l = r8; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = r14; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = r14; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(r14, rax); bb = 12; continue; }
            12 => { r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = r14.wrapping_add(0x20); bb = 13; continue; }
            13 => { fl_l = r14; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = r14; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 17; continue; }
            17 => { rax = (0x69fafafafaffu64) as u64; rcx = rsi.wrapping_add(0x18); crate::simd_ops::wu64(r14, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rsi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = P5; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu64(r14.wrapping_add(0xcu64), rax); crate::simd_ops::wu64(r14.wrapping_add(0x14u64), rdi); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(r14, (0x14u64) as u64); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412d0d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143c4aa00; const P2: u64 = 0x1426455a8; const P3: u64 = 0x142645598; const P4: u64 = 0x1421f29c0; const P5: u64 = 0x142645580;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rbp = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; r8 = P1; r8 = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) < ((fl_r as u32))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((crate::simd_ops::ru8(r14.wrapping_add(0xcu64))) as u32) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x68u64), ((rcx as u32)) as u32); crate::simd_ops::wu8(r8.wrapping_add(0x6cu64), ((rax as u8)) as u8); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi.wrapping_add(0x28); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_sub(0x8u64)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (0xffffffffffu64) as u64; crate::simd_ops::wu64(rsi, rax); bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi.wrapping_add(0x28); bb = 11; continue; }
            11 => { fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { rax = (0x69fafafafaffu64) as u64; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu64(rsi, rax); let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdi.wrapping_add(0x28u64), r8); r8 = (tmp_xchg) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(r14)) as u128; rax = P4; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0xcu64), rax); crate::simd_ops::wu128(rsi.wrapping_add(0x14u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64; crate::simd_ops::wu64(rsi, (0x1cu64) as u64); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x30u64), (1) as u32); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1a0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412d1100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411e8bb0; const P1: u64 = 0x1402013d0; const P2: u64 = 0xfffffffffffffff3; const P3: u64 = 0x141834fc0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    rax = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    r9 = P1;
    rdx = 0x10;
    r8 = ((rdx.wrapping_add(P2)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412d1140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14220ea70; const P1: u64 = 0x10; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    rax = (0xffffffffffffffffu64) as u64;
    loop { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = rsp.wrapping_add(0x20);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((rbx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), (0xcu32) as u32);
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412d1330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14220ea70; const P1: u64 = 0x10; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    rax = (0xffffffffffffffffu64) as u64;
    loop { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = rsp.wrapping_add(0x20);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((rbx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), (0xau32) as u32);
    crate::simd_ops::wu64(rbx, rax);
    crate::simd_ops::wu64(rax, (0x0u64) as u64);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412d1530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433ef190; const P1: u64 = 0x143cef790; const P2: u64 = 0x143cef788; const P3: u64 = 0x14220ea70; const P4: u64 = 0x143cef798; const P5: u64 = 0x143cef794;
    let (mut rax, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1fu64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rcx = P1;
    crate::simd_ops::wu64(P2, (0x0u64) as u64);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    rdi = P2;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64);
    rcx = P4;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32);
    rax = rdi;
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu64(P2, rcx);
    crate::simd_ops::wu128(P4, (xmm0) as u128);
    crate::simd_ops::wu32(P5, (0x1bu32) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412d15c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14220ea70; const P1: u64 = 0x10; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    rax = (0xffffffffffffffffu64) as u64;
    loop { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { break; } }
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = rsp.wrapping_add(0x20);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((rbx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x10u32) as u32);
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412d17f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14183acc2; const P5: u64 = 0x1401eecc0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rdx = (((r9) as i64).wrapping_mul((0xbcu64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = 0x6348; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rdi = r9; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; r9 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(r9.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rax); bb = 13; continue; }
            5 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = rsi; bb = 13; continue; }
            7 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            11 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { r8 = rax; bb = 13; continue; }
            13 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            15 => { rsi = r8; bb = 16; continue; }
            16 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rsi; bb = 18; continue; }
            18 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xbc; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xbcu64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 20; continue; }
            20 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rsi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412d1a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143881930; const P1: u64 = 0x143881938; const P2: u64 = 0x1401e9fa0; const P3: u64 = 0x14243ae40; const P4: u64 = 0x1401e9e70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r9 = (crate::simd_ops::ru32(P0)) as u32 as u64; rbx = (rdx) as u64; r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = (rcx) as u64; rdi = (rcx) as u64; rbp = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbp as u32)) as u32); rcx = (rbx) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2du32) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((rdi.wrapping_add(0xa0u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rbp as u32)) as u32); rsi = ((rbx.wrapping_add(0x369u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 4; continue; }
            4 => { rdx = ((rdi.wrapping_add(0xa4u64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x9u32) as u32 as u64; rdx = ((rdi.wrapping_add(0xa8u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 12; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x9u32) as u32 as u64; rdx = ((rdi.wrapping_add(0xacu64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 16; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0xau32) as u32 as u64; rdx = ((rdi.wrapping_add(0xb0u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 20; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0xau32) as u32 as u64; rdx = ((rdi.wrapping_add(0xb4u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 24; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x9u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x40u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 28; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x9u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x44u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 32; continue; }
            30 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 32; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::wu32(rdx, (0x1u32) as u32); bb = 32; continue; }
            32 => { r9 = (0x9u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; rdx = ((r9.wrapping_sub(0x8u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0x1u32) as u32 as u64; rcx = (rbx) as u64; r9 = ((rdx.wrapping_add(0x8u64)) as u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0x1u32) as u32 as u64; rcx = (rbx) as u64; r9 = ((rdx.wrapping_add(0x8u64)) as u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x7u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (0x9u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rbx) as u64; rdx = ((r9.wrapping_sub(0x8u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x6u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x98u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 36; continue; }
            34 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu8(rdx, (0x1u8) as u8); bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x1au32) as u32 as u64; rdx = ((rdi.wrapping_add(0x99u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 40; continue; }
            38 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 40; continue; } bb = 39; continue; }
            39 => { crate::simd_ops::wu8(rdx, (0x1u8) as u8); bb = 40; continue; }
            40 => { rdx = (0x1u32) as u32 as u64; rcx = (rbx) as u64; r9 = ((rdx.wrapping_add(0x21u64)) as u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x1cu64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x28u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x9au64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 42; continue; } bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 44; continue; }
            42 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 44; continue; } bb = 43; continue; }
            43 => { crate::simd_ops::wu8(rdx, (0x1u8) as u8); bb = 44; continue; }
            44 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x28u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x9bu64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 46; continue; } bb = 45; continue; }
            45 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 48; continue; }
            46 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 48; continue; } bb = 47; continue; }
            47 => { crate::simd_ops::wu8(rdx, (0x1u8) as u8); bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x28u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x9cu64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 50; continue; } bb = 49; continue; }
            49 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 52; continue; }
            50 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 52; continue; } bb = 51; continue; }
            51 => { crate::simd_ops::wu8(rdx, ((rbp as u8)) as u8); bb = 52; continue; }
            52 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x28u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x9du64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 54; continue; } bb = 53; continue; }
            53 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 56; continue; }
            54 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 56; continue; } bb = 55; continue; }
            55 => { crate::simd_ops::wu8(rdx, ((rbp as u8)) as u8); bb = 56; continue; }
            56 => { rdx = (0x4u32) as u32 as u64; rcx = (rbx) as u64; r9 = ((rdx.wrapping_add(0x27u64)) as u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x24u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 58; continue; } bb = 57; continue; }
            57 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = (rdi) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 60; continue; }
            58 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 60; continue; } bb = 59; continue; }
            59 => { crate::simd_ops::wu32(rdi, ((rbp as u32)) as u32); bb = 60; continue; }
            60 => { rdx = ((rdi.wrapping_add(0x4u64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 62; continue; } bb = 61; continue; }
            61 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 64; continue; }
            62 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 64; continue; } bb = 63; continue; }
            63 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 64; continue; }
            64 => { rdx = ((rdi.wrapping_add(0x8u64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 66; continue; } bb = 65; continue; }
            65 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 68; continue; }
            66 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 68; continue; } bb = 67; continue; }
            67 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 68; continue; }
            68 => { rdx = ((rdi.wrapping_add(0xcu64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 70; continue; } bb = 69; continue; }
            69 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 72; continue; }
            70 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 72; continue; } bb = 71; continue; }
            71 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 72; continue; }
            72 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x23u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x10u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 74; continue; } bb = 73; continue; }
            73 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 76; continue; }
            74 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 76; continue; } bb = 75; continue; }
            75 => { crate::simd_ops::wu32(rdx, (0x1u32) as u32); bb = 76; continue; }
            76 => { rdx = ((rdi.wrapping_add(0x14u64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 78; continue; } bb = 77; continue; }
            77 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 80; continue; }
            78 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 80; continue; } bb = 79; continue; }
            79 => { crate::simd_ops::wu32(rdx, (0x1u32) as u32); bb = 80; continue; }
            80 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0xau32) as u32 as u64; rdx = ((rdi.wrapping_add(0x18u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 82; continue; } bb = 81; continue; }
            81 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 84; continue; }
            82 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 84; continue; } bb = 83; continue; }
            83 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 84; continue; }
            84 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 86; continue; } bb = 85; continue; }
            85 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 86; continue; }
            86 => { rdx = ((rdi.wrapping_add(0x1cu64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 88; continue; } bb = 87; continue; }
            87 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 90; continue; }
            88 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 90; continue; } bb = 89; continue; }
            89 => { crate::simd_ops::wu32(rdx, (0x1u32) as u32); bb = 90; continue; }
            90 => { rdx = ((rdi.wrapping_add(0x20u64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 92; continue; } bb = 91; continue; }
            91 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 94; continue; }
            92 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 94; continue; } bb = 93; continue; }
            93 => { crate::simd_ops::wu32(rdx, (0x1u32) as u32); bb = 94; continue; }
            94 => { rdx = ((rdi.wrapping_add(0x24u64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 96; continue; } bb = 95; continue; }
            95 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 98; continue; }
            96 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 98; continue; } bb = 97; continue; }
            97 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 98; continue; }
            98 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x1eu32) as u32 as u64; rdx = ((rdi.wrapping_add(0x28u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 100; continue; } bb = 99; continue; }
            99 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 102; continue; }
            100 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 102; continue; } bb = 101; continue; }
            101 => { crate::simd_ops::wu32(rdx, (0x1u32) as u32); bb = 102; continue; }
            102 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x21u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x2cu64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 104; continue; } bb = 103; continue; }
            103 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 106; continue; }
            104 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 106; continue; } bb = 105; continue; }
            105 => { crate::simd_ops::wu32(rdx, (0x1u32) as u32); bb = 106; continue; }
            106 => { rdx = (0x4u32) as u32 as u64; rcx = (rbx) as u64; r9 = ((rdx.wrapping_add(0xfu64)) as u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x9u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x10u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x30u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 108; continue; } bb = 107; continue; }
            107 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 110; continue; }
            108 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 110; continue; } bb = 109; continue; }
            109 => { crate::simd_ops::wu32(rdx, (0x1u32) as u32); bb = 110; continue; }
            110 => { rdx = ((rdi.wrapping_add(0x38u64)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 112; continue; } bb = 111; continue; }
            111 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 114; continue; }
            112 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 114; continue; } bb = 113; continue; }
            113 => { crate::simd_ops::wu32(rdx, (0x1u32) as u32); bb = 114; continue; }
            114 => { rdx = (0x4u32) as u32 as u64; rcx = (rbx) as u64; r9 = ((rdx.wrapping_add(0x12u64)) as u32) as u32 as u64; r8 = ((rdx.wrapping_sub(0x3u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (0x4u32) as u32 as u64; rcx = (rbx) as u64; r9 = ((rdx.wrapping_add(0x5u64)) as u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0xbu32) as u32 as u64; rdx = ((rdi.wrapping_add(0x48u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 116; continue; } bb = 115; continue; }
            115 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 118; continue; }
            116 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 118; continue; } bb = 117; continue; }
            117 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 118; continue; }
            118 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0xcu32) as u32 as u64; rdx = ((rdi.wrapping_add(0x4cu64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 120; continue; } bb = 119; continue; }
            119 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 122; continue; }
            120 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 122; continue; } bb = 121; continue; }
            121 => { crate::simd_ops::wu32(rdx, (0x1u32) as u32); bb = 122; continue; }
            122 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0xbu32) as u32 as u64; rdx = ((rdi.wrapping_add(0x50u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 124; continue; } bb = 123; continue; }
            123 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 126; continue; }
            124 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 126; continue; } bb = 125; continue; }
            125 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 126; continue; }
            126 => { rdx = (0x4u32) as u32 as u64; rcx = (rbx) as u64; r9 = ((rdx.wrapping_add(0x21u64)) as u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x7u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x15u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 128; continue; } bb = 127; continue; }
            127 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = ((rdi.wrapping_add(0x54u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 130; continue; }
            128 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 130; continue; } bb = 129; continue; }
            129 => { crate::simd_ops::wu32(rdi.wrapping_add(0x54u64), ((rbp as u32)) as u32); bb = 130; continue; }
            130 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x24u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x58u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 132; continue; } bb = 131; continue; }
            131 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 134; continue; }
            132 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 134; continue; } bb = 133; continue; }
            133 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 134; continue; }
            134 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0xeu32) as u32 as u64; rdx = ((rdi.wrapping_add(0x5cu64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 136; continue; } bb = 135; continue; }
            135 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 138; continue; }
            136 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 138; continue; } bb = 137; continue; }
            137 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 138; continue; }
            138 => { rdx = (0x4u32) as u32 as u64; rcx = (rbx) as u64; r9 = ((rdx.wrapping_add(0x21u64)) as u32) as u32 as u64; r8 = ((rdx.wrapping_add(0xdu64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x1bu32) as u32 as u64; rdx = ((rdi.wrapping_add(0x60u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 140; continue; } bb = 139; continue; }
            139 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 142; continue; }
            140 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 142; continue; } bb = 141; continue; }
            141 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 142; continue; }
            142 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x24u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x64u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 144; continue; } bb = 143; continue; }
            143 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 146; continue; }
            144 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 146; continue; } bb = 145; continue; }
            145 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 146; continue; }
            146 => { rdx = (0x4u32) as u32 as u64; rcx = (rbx) as u64; r9 = ((rdx.wrapping_add(0x20u64)) as u32) as u32 as u64; r8 = ((rdx.wrapping_add(0x1eu64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0xfu32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 148; continue; } bb = 147; continue; }
            147 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 148; continue; }
            148 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0xbu32) as u32 as u64; rdx = ((rdi.wrapping_add(0x70u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 150; continue; } bb = 149; continue; }
            149 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 152; continue; }
            150 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 152; continue; } bb = 151; continue; }
            151 => { crate::simd_ops::wu32(rdx, (0x1u32) as u32); bb = 152; continue; }
            152 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x14u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x74u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 154; continue; } bb = 153; continue; }
            153 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 156; continue; }
            154 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 156; continue; } bb = 155; continue; }
            155 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 156; continue; }
            156 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x19u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x88u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 158; continue; } bb = 157; continue; }
            157 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 160; continue; }
            158 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 160; continue; } bb = 159; continue; }
            159 => { crate::simd_ops::wu32(rdx, (0x40200000u32) as u32); bb = 160; continue; }
            160 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x19u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x8cu64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 162; continue; } bb = 161; continue; }
            161 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 164; continue; }
            162 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 164; continue; } bb = 163; continue; }
            163 => { crate::simd_ops::wu32(rdx, (0x40200000u32) as u32); bb = 164; continue; }
            164 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x19u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x90u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 166; continue; } bb = 165; continue; }
            165 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 168; continue; }
            166 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 168; continue; } bb = 167; continue; }
            167 => { crate::simd_ops::wu32(rdx, (0xbf800000u32) as u32); bb = 168; continue; }
            168 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x19u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x94u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 170; continue; } bb = 169; continue; }
            169 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 172; continue; }
            170 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 172; continue; } bb = 171; continue; }
            171 => { crate::simd_ops::wu32(rdx, (0xbf800000u32) as u32); bb = 172; continue; }
            172 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x17u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 174; continue; } bb = 173; continue; }
            173 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = ((rdi.wrapping_add(0x68u64)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 176; continue; }
            174 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 176; continue; } bb = 175; continue; }
            175 => { crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), (0x1u32) as u32); bb = 176; continue; }
            176 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x2au32) as u32 as u64; rdx = ((rdi.wrapping_add(0x6cu64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 178; continue; } bb = 177; continue; }
            177 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 180; continue; }
            178 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 180; continue; } bb = 179; continue; }
            179 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 180; continue; }
            180 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x1du32) as u32 as u64; rdx = ((rdi.wrapping_add(0x34u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 182; continue; } bb = 181; continue; }
            181 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 184; continue; }
            182 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 184; continue; } bb = 183; continue; }
            183 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 184; continue; }
            184 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x1fu32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 186; continue; } bb = 185; continue; }
            185 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = ((rdi.wrapping_add(0x3cu64)) as u64) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 188; continue; }
            186 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 188; continue; } bb = 187; continue; }
            187 => { crate::simd_ops::wu32(rdi.wrapping_add(0x3cu64), (0x1u32) as u32); bb = 188; continue; }
            188 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x24u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x7cu64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 190; continue; } bb = 189; continue; }
            189 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 192; continue; }
            190 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 192; continue; } bb = 191; continue; }
            191 => { crate::simd_ops::wu8(rdx, ((rbp as u8)) as u8); bb = 192; continue; }
            192 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x2du32) as u32 as u64; rdx = ((rdi.wrapping_add(0x84u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 194; continue; } bb = 193; continue; }
            193 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 196; continue; }
            194 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 196; continue; } bb = 195; continue; }
            195 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 196; continue; }
            196 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x2cu32) as u32 as u64; rdx = ((rdi.wrapping_add(0x7du64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 198; continue; } bb = 197; continue; }
            197 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 200; continue; }
            198 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 200; continue; } bb = 199; continue; }
            199 => { crate::simd_ops::wu8(rdx, ((rbp as u8)) as u8); bb = 200; continue; }
            200 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; rdx = ((rdi.wrapping_add(0x7eu64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2cu32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 205; continue; } bb = 201; continue; }
            201 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x26u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 203; continue; } bb = 202; continue; }
            202 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 205; continue; }
            203 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 205; continue; } bb = 204; continue; }
            204 => { crate::simd_ops::wu8(rdx, ((rbp as u8)) as u8); bb = 205; continue; }
            205 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x2cu32) as u32 as u64; rdx = ((rdi.wrapping_add(0x80u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 207; continue; } bb = 206; continue; }
            206 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 209; continue; }
            207 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 209; continue; } bb = 208; continue; }
            208 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 209; continue; }
            209 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x28u32) as u32 as u64; rdx = ((rdi.wrapping_add(0x78u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 211; continue; } bb = 210; continue; }
            210 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 213; continue; }
            211 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 213; continue; } bb = 212; continue; }
            212 => { crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 213; continue; }
            213 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x2bu32) as u32 as u64; rdx = ((rdi.wrapping_add(0xb8u64)) as u64) as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 215; continue; } bb = 214; continue; }
            214 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 217; continue; }
            215 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 217; continue; } bb = 216; continue; }
            216 => { crate::simd_ops::wu32(rdx, (0x3cu32) as u32); bb = 217; continue; }
            217 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x17u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 219; continue; } bb = 218; continue; }
            218 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((rbp as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rax as u32)) as u32); bb = 219; continue; }
            219 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x1cu32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 221; continue; } bb = 220; continue; }
            220 => { crate::simd_ops::wu32(rdi.wrapping_add(0x54u64), ((rbp as u32)) as u32); bb = 221; continue; }
            221 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x20u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 223; continue; } bb = 222; continue; }
            222 => { crate::simd_ops::wu32(rdi.wrapping_add(0x3cu64), (0x1u32) as u32); bb = 223; continue; }
            223 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x24u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 225; continue; } bb = 224; continue; }
            224 => { rax = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0x54u64), ((rax as u32)) as u32); bb = 225; continue; }
            225 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x27u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 227; continue; } bb = 226; continue; }
            226 => { crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rbp as u32)) as u32); bb = 227; continue; }
            227 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x28u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 229; continue; } bb = 228; continue; }
            228 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((rbp as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0x78u64), ((rax as u32)) as u32); bb = 229; continue; }
            229 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x29u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 231; continue; } bb = 230; continue; }
            230 => { crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x50u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x5cu64), ((rbp as u32)) as u32); bb = 231; continue; }
            231 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x2cu32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 234; continue; } bb = 232; continue; }
            232 => { crate::simd_ops::wu8(rdi.wrapping_add(0x7du64), (0x1u8) as u8); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = (0x2cu32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 234; continue; } bb = 233; continue; }
            233 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x7eu64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; rbp = (rbp & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0x80u64), ((rbp as u32)) as u32); bb = 234; continue; }
            234 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1412d24c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143881938; const P1: u64 = 0x14243b0b0; const P2: u64 = 0x14243ae40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; rbx = rdx; rdx = rcx; rdi = rcx; rsi = 0; fl_l = 0; fl_r = 0; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2du32) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0xa0); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0xa4); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0xa8); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0xac); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0xb0); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0xb4); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x40); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x44); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = ((rsi.wrapping_add(0x9u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rbx; r9 = ((rsi.wrapping_add(0x9u64)) as u32) as u32 as u64; r8 = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rbx; r9 = ((rsi.wrapping_add(0x9u64)) as u32) as u32 as u64; r8 = ((rsi.wrapping_add(0x8u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = ((rsi.wrapping_add(0x9u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x98); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x99); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rbx; r9 = ((rsi.wrapping_add(0x22u64)) as u32) as u32 as u64; r8 = ((rsi.wrapping_add(0x1du64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x9a); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi.wrapping_add(0x9b); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x9c); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x9d); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = rbx; r9 = ((rsi.wrapping_add(0x2bu64)) as u32) as u32 as u64; r8 = ((rsi.wrapping_add(0x28u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x4); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x8); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0xc); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x10); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x14); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x18); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rsp.wrapping_add(0x70); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x1c); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x24); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x28); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x2c); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = rbx; r9 = ((rsi.wrapping_add(0x13u64)) as u32) as u32 as u64; r8 = ((rsi.wrapping_add(0xdu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x30); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x38); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = rbx; r9 = ((rsi.wrapping_add(0x16u64)) as u32) as u32 as u64; r8 = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = rbx; r9 = ((rsi.wrapping_add(0x9u64)) as u32) as u32 as u64; r8 = ((rsi.wrapping_add(0x5u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x48); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x4c); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x50); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = rbx; r9 = ((rsi.wrapping_add(0x25u64)) as u32) as u32 as u64; r8 = ((rsi.wrapping_add(0xbu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x54); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x58); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x5c); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = rbx; r9 = ((rsi.wrapping_add(0x25u64)) as u32) as u32 as u64; r8 = ((rsi.wrapping_add(0x11u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x60); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x64); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = rbx; r9 = ((rsi.wrapping_add(0x24u64)) as u32) as u32 as u64; r8 = ((rsi.wrapping_add(0x22u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x17; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rsp.wrapping_add(0x80); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x70); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x74); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x88); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x8c); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x90); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x94); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x68); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x6c); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x34); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x3c); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x7c); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x84); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x7d); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x2c; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x7e); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x80); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0x78); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x338u64))) as u64; rdx = rdi.wrapping_add(0xb8); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x17; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; fl_r = 0x1; rax = ((rsi as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((rax as u32)) as u32); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x1c; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(0x54u64), ((rsi as u32)) as u32); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x20; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(0x3cu64), (0x1u32) as u32); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x24; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = ((rsi as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0x54u64), ((rax as u32)) as u32); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x27; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rsi as u32)) as u32); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x28; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; fl_r = 0x1; rax = ((rsi as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0x78u64), ((rax as u32)) as u32); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x29; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x50u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x5cu64), ((rsi as u32)) as u32); bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x2c; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu8(rdi.wrapping_add(0x7du64), (0x1u8) as u8); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x328u64))) as u32 as u64; fl_r = 0x2c; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x7eu64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; rsi = (rsi & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0x80u64), ((rsi as u32)) as u32); bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u32 as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x328u64), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
