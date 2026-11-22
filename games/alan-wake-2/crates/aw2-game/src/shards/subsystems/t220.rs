#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d18; const P1: u64 = 0x4; const P2: u64 = 0x141524330; const P3: u64 = 0x8; const P4: u64 = 0x142688f98; const P5: u64 = 0x14392d450; const P6: u64 = 0x18; const P7: u64 = 0x10; const P8: u64 = 0x14392d458; const P9: u64 = 0x14392d448; const P10: u64 = 0x142688fa8; const P11: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = 0xf0; rcx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax.wrapping_add(P3), rdx); rax = P4; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P7), rdx); fl_l = (crate::simd_ops::ru64(P8)) as u64; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(P8, rcx); bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P7), rcx); bb = 5; continue; }
            5 => { crate::simd_ops::lk_add_u32(P9, (1) as u32); rax = P10; crate::simd_ops::wu64(P5, rcx); crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P11), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0xb0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0xb8u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0xd0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0xd8u64), rdx); crate::simd_ops::wu16(rcx.wrapping_add(0xe0u64), ((rdx as u16)) as u16); crate::simd_ops::wu64(P0, rcx); crate::simd_ops::wu8(rcx.wrapping_add(0xe0u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0xe1u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(P0)) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            6 => { crate::simd_ops::wu64(P0, rdx); rax = rdx; bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f90; const P1: u64 = 0x14159a450;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f78; const P2: u64 = 0x1415f3790; const P3: u64 = 0x1415b1830; const P4: u64 = 0x141605340; const P5: u64 = 0x14158c580; const P6: u64 = 0x14158d2f0; const P7: u64 = 0x14158c3d0; const P8: u64 = 0x1434268d0; const P9: u64 = 0x141603910; const P10: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x0; rbx = r8; rsi = rdx; rdi = rcx; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); rcx = rsp.wrapping_add(0x70); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((r14.wrapping_add(0xfu64)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xe0u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbp = rax.wrapping_add(0x20); bb = 7; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xe1u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = rdi.wrapping_add(0x70); bb = 10; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { xmm2 = ((crate::simd_ops::ru32(P8)) as u32) as u128; rcx = rbp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rdx = rsp.wrapping_add(0x30); xmm3 = 0u128; rcx = rax; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm3 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; r8 = ((r14 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (if (((fl_l as u32)) < ((fl_r as u32))) { crate::simd_ops::ru32(rbx) } else { (rcx as u32) }) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rcx = ((r8 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rdx))) as u128; crate::simd_ops::wu128(rsi.wrapping_add(rcx.wrapping_mul(8)), (xmm0) as u128); fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 13; continue; }
            13 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r14 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { rax = 0x1f; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f78; const P2: u64 = 0x1415f3790; const P3: u64 = 0x10; const P4: u64 = 0xe0; const P5: u64 = 0x20; const P6: u64 = 0x14158c580; const P7: u64 = 0x14158d2f0; const P8: u64 = 0x1416038d0; const P9: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rsi = r8; rcx = rsp.wrapping_add(0x58); rdi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rax.wrapping_add(P5)) as u64) as u64; bb = 5; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; bb = 5; continue; }
            5 => { rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi, ((xmm0 as u32)) as u32); rbx = 0x1; bb = 7; continue; }
            6 => { rbx = 0xf; bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c8b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f90; const P1: u64 = 0x14159a4d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } rax = (0xffffffffffffffffu64) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c8d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f78; const P2: u64 = 0x1415f39b0; const P3: u64 = 0x1415b1830;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsp.wrapping_add(0x38);
    rbx = (((rax as u8)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (((rbx as u8)) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f78; const P2: u64 = 0x1415f3790; const P3: u64 = 0x20; const P4: u64 = 0x60; const P5: u64 = 0x70; const P6: u64 = 0x10; const P7: u64 = 0x80; const P8: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = rdx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P4))) as u128; rbx = 0x1; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(P6), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P7))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(P3), (xmm0) as u128); bb = 4; continue; }
            3 => { rbx = 0x2; bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x1415d79c0; const P2: u64 = 0x1415b1830;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx as u32)) as u32 as u64;
    rdi = r9;
    rcx = rsp.wrapping_add(0x30);
    rsi = r8;
    rbp = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx as u32)) as u32 as u64;
    r9 = rdi;
    r8 = rsi;
    rdx = rbp;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsp.wrapping_add(0x30);
    rbx = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = ((rbx as u32)) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c9f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f78; const P2: u64 = 0x1415f3790; const P3: u64 = 0xf; const P4: u64 = 0x141605350; const P5: u64 = 0x18; const P6: u64 = 0x10; const P7: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; r14 = r8; rcx = rsp.wrapping_add(0x58); r15 = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = ((rax.wrapping_add(P3)) as u32) as u32 as u64; bb = 7; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rcx = rbp; rsi = (crate::simd_ops::ru32(r14)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(r14, ((rcx as u32)) as u32); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = ((rdi as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rsi as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx as u32)) as u32 as u64; fl_l = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rbx = ((fl_l as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; rdx = ((rcx.wrapping_mul(8)) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(r15), rcx); bb = 4; continue; }
            6 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (rdi & !0xffu64) | (((if (((fl_l as u32)) < ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158cab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x1415979b0; const P2: u64 = 0x1415b1830;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsp.wrapping_add(0x38);
    rbx = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rbx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158cae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x141597e30; const P2: u64 = 0x1415b1830;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128);
    rcx = rsp.wrapping_add(0x48);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = rbx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = rsp.wrapping_add(0x48);
    xmm6 = (xmm0) as u128;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (xmm6) as u128;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158cb20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f78; const P2: u64 = 0x1415f3790; const P3: u64 = 0x14183acbc; const P4: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; r14 = r9; rcx = rsp.wrapping_add(0x20); r15 = r8; rsi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rdi = rax.wrapping_add(0x8); fl_l = rax; fl_r = rcx; rdi = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rdi }) as u64; rbp = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rsp.wrapping_add(0x28); rax = (0xfffffffffffffffu64) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) > ((fl_r as u32))) { (rbp as u32) } else { (rbx as u32) }) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; r8 = (((r8) & (rax))) as u64; rsi = ((rbx as u32)) as u32 as u64; r8 = ((r8).wrapping_add(r8)) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rsi = ((rsi).wrapping_shl(4u32)) as u64; rax = rsp.wrapping_add(0x28); rsi = ((rsi).wrapping_add(rax)) as u64; rcx = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdi = rsi.wrapping_add(0x10); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx.wrapping_mul(8)))) as u128; rcx = ((rbp as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rbx as u32))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_sub(0x10u64)) as u64; fl_l = ((rcx).wrapping_shr(3u32)) as u64; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu128(rsi, (xmm0) as u128); while rcx != 0 { crate::simd_ops::wu64(rdi, crate::simd_ops::ru64(rsi)); rdi = rdi.wrapping_add(8); rsi = rsi.wrapping_add(8); rcx = rcx.wrapping_sub(1); } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 11; continue; }
            11 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32) as u128; crate::simd_ops::wu32(r15, ((xmm0 as u32)) as u32); crate::simd_ops::wu32(r14, ((xmm1 as u32)) as u32); bb = 13; continue; }
            12 => { rbp = 0xf; bb = 13; continue; }
            13 => { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rbp as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158cc70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f90; const P1: u64 = 0x14159a5d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = rcx;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r9 = r8; r8 = rdx; rdx = rax; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = 0x2;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158cca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f78; const P2: u64 = 0x1415f3790; const P3: u64 = 0x8; const P4: u64 = 0x20; const P5: u64 = 0x142688fd8; const P6: u64 = 0x142688fb8; const P7: u64 = 0x1439312e8; const P8: u64 = 0x142688fe8; const P9: u64 = 0x142688fc8; const P10: u64 = 0x1439312f8; const P11: u64 = 0x1439312f4; const P12: u64 = 0x1439312e0; const P13: u64 = 0x142688fd0; const P14: u64 = 0x142688fe0; const P15: u64 = 0x1439312ec; const P16: u64 = 0x143931308; const P17: u64 = 0x143931310; const P18: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = rdx; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(P5)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(P6)) as u128; crate::simd_ops::wu32(P7, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(P8)) as u32 as u64; xmm1 = ((crate::simd_ops::ru64(P9)) as u64) as u128; crate::simd_ops::wu128(P10, (xmm0) as u128); crate::simd_ops::wu32(P11, ((rax as u32)) as u32); rax = P12; xmm0 = ((crate::simd_ops::ru64(P13)) as u64) as u128; crate::simd_ops::wu64(P12, ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(P14)) as u64) as u128; crate::simd_ops::wu64(P15, ((xmm0 as u64)) as u64); crate::simd_ops::wu64(P16, ((xmm1 as u64)) as u64); crate::simd_ops::wu32(P17, (0x3ff3fu32) as u32); bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; rbx = 0x1; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x20u64), (xmm0) as u128); bb = 6; continue; }
            5 => { rbx = 0xf; bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158cd90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x14159fc80; const P6: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r15 = rdx; rbp = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x8); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { rdi = r14; bb = 9; continue; }
            9 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            11 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            13 => { rdi = r14; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x8); bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = rdi.wrapping_sub(0x10); fl_l = rdi; fl_r = rdi; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rbx }) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rdi = ((rbx.wrapping_add(0xfu64)) as u32) as u32 as u64; bb = 28; continue; }
            26 => { rdx = r15; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rcx = rbx.wrapping_add(0x10); rdx = 0xffffffff; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158d0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f60; const P2: u64 = 0x1415af900; const P3: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rcx as u32)) as u32 as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x40);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    r8 = rbx;
    rdx = ((rdi as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    rcx = 0x1;
    rbx = 0xf;
    rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rcx as u32) } else { (rbx as u32) }) as u32 as u64;
    rcx = rsp.wrapping_add(0x40);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rbx as u32)) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158d1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f78; const P2: u64 = 0x1415f3790; const P3: u64 = 0x143930fa0; const P4: u64 = 0x1415c3fb0; const P5: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = ((rcx as u32)) as u32 as u64; rsi = r8; rcx = rsp.wrapping_add(0x80); rbx = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P1)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rdx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0xffu8) as u8); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P3)) as u64; r9 = rsp.wrapping_add(0x90); crate::simd_ops::wu32(rsp.wrapping_add(0x98u64), ((rdi as u32)) as u32); r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu8(rsp.wrapping_add(0x9cu64), (0x2u8) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsi, ((rcx as u32)) as u32); rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158d320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x141606080; const P6: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0; r14 = r8; r15 = rdx; rbp = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((r8.wrapping_add(0x1fu64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rsi = rbx.wrapping_add(0x110); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x150u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x148u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 12; continue; }
            12 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 18; continue; }
            15 => { rsi = rbx.wrapping_add(0x110); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 7; continue; } bb = 16; continue; }
            16 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            17 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 26; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 20; continue; } bb = 22; continue; }
            22 => { bb = 27; continue; }
            23 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            26 => { rsi = rbx.wrapping_add(0x110); bb = 27; continue; }
            27 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rbx = ((rdi.wrapping_add(0xfu64)) as u32) as u32 as u64; bb = 32; continue; }
            30 => { r8 = r14; rdx = r15; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; rdx = 0xffffffff; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { rcx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158d520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143413a20; const P1: u64 = 0x143413a30; const P2: u64 = 0x17; const P3: u64 = 0x142647298; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    r9 = rsp.wrapping_add(0x30);
    xmm0 = (crate::simd_ops::ru128(P0)) as u128;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    xmm1 = (crate::simd_ops::ru128(P1)) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rsp.wrapping_add(0x40);
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    r8 = ((rdx.wrapping_add(P2)) as u32) as u32 as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { rax = rbx; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, (0x0u64) as u64); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158d5c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415810d0; const P1: u64 = 0x142647268; const P2: u64 = 0x14268dd38; const P3: u64 = 0x143931340; const P4: u64 = 0x142646940; const P5: u64 = 0x143931546; const P6: u64 = 0x1426472a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; r14 = rdx; rsi = rcx; rdi = 0xffffffff; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { crate::simd_ops::wu32(r14, ((rdi as u32)) as u32); r8 = rsp.wrapping_add(0x68); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; r8 = rsp.wrapping_add(0x20); xmm0 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru16(rsp.wrapping_add(0x20u64))) as u16 as u64; fl_r = (0x1fu16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r8 = 0x103; rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu16(P5, ((rbx as u16)) as u16); rbx = P3; bb = 10; continue; }
            10 => { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158d6d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x30; const P3: u64 = 0x40; const P4: u64 = 0x142646070; const P5: u64 = 0x1415edcc0; const P6: u64 = 0x28; const P7: u64 = 0x38; const P8: u64 = 0x3c; const P9: u64 = 0x44;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm0 = 0u128;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    rbx = rcx;
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = rbx;
    crate::simd_ops::wu64(rbx, rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = 0x4;
    crate::simd_ops::wu32(rbx.wrapping_add(P6), (0x20000u32) as u32);
    crate::simd_ops::wu16(rbx.wrapping_add(P7), ((rax as u16)) as u16);
    crate::simd_ops::wu32(rbx.wrapping_add(P8), (0xbb80u32) as u32);
    crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x1u8) as u8);
    crate::simd_ops::wu32(rbx.wrapping_add(P9), (0x80u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158d730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931328; const P1: u64 = 0x1415ecaf0; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rax; crate::simd_ops::wu64(P0, rax); }
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = ((rcx.wrapping_add(P2)) as u64) as u64;
    fl_l = rcx; fl_r = rcx;
    rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158d770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x1415810d0; const P2: u64 = 0x142647268;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = 0xffffffff;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = ((rbx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { rax = ((rbx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); }
    rax = ((rbx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158d7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647278; const P1: u64 = 0x143413a20; const P2: u64 = 0x143413a30; const P3: u64 = 0x142647298; const P4: u64 = 0x1415810d0; const P5: u64 = 0x14183ace0; const P6: u64 = 0x142647268; const P7: u64 = 0x14268dd38; const P8: u64 = 0x1426472a8; const P9: u64 = 0x1426472d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rbp = rsp.wrapping_sub(0x3f); rsp = ((rsp).wrapping_sub(0x98u64)) as u64; rdi = 0xffffffff; r13 = rcx; r12 = ((rdi as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x98u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rdx = 0x2; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(P1)) as u128; rdx = 0; fl_l = 0; fl_r = 0; rbx = ((rax as u32)) as u32 as u64; xmm1 = (crate::simd_ops::ru128(P2)) as u128; rax = rbp.wrapping_sub(0x19); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (0x0u64) as u64); r9 = rbp.wrapping_sub(0x29); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = ((rdx.wrapping_add(0x17u64)) as u32) as u32 as u64; rcx = rbp.wrapping_sub(0x9); crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rcx); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (0x0u64) as u64); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = rbp.wrapping_sub(0x11); rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(0xfu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), r8); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rcx); bb = 12; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; bb = 12; continue; }
            12 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), (0x0u32) as u32); rdx = rbp.wrapping_add(0x67); rax = (crate::simd_ops::ru64(r8)) as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 34; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r14); r14 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 33; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r15); bb = 16; continue; }
            16 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; r8 = rbp.wrapping_add(0x7f); crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), (0x0u64) as u64); rdx = ((r14 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 31; continue; } bb = 18; continue; }
            18 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rdx = rbp.wrapping_add(0x77); rcx = rsi; crate::simd_ops::wu64(rbp.wrapping_add(0x77u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 29; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; rdx = r13; rdi = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; r15 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = rbp.wrapping_sub(0x29); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), r15); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 29; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; r8 = rbp.wrapping_sub(0x9); rax = 0; fl_l = 0; fl_r = 0; rdx = P7; xmm0 = 0u128; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_sub(0x9); r15 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r15; fl_r = r15; rax = 0xffffffff; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { (rdi as u32) } else { (rax as u32) }) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; bb = 26; continue; }
            26 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rdx = rbp.wrapping_add(0x6f); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0xffffffff; rdi = (if (((fl_l as u32)) != ((fl_r as u32))) { (r12 as u32) } else { (rdi as u32) }) as u32 as u64; r12 = ((rdi as u32)) as u32 as u64; bb = 29; continue; }
            29 => { rdi = 0xffffffff; bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 32; continue; }
            32 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 33; continue; }
            33 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (0x0u64) as u64); bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (0x0u64) as u64); bb = 38; continue; }
            38 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0xffffffff; r12 = (if (((fl_l as u32)) != ((fl_r as u32))) { (rdi as u32) } else { (r12 as u32) }) as u32 as u64; bb = 41; continue; }
            39 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (0x0u64) as u64); bb = 41; continue; }
            41 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 43; continue; }
            43 => { rax = ((r12 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x98u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158dac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143413a20; const P1: u64 = 0x143413a30; const P2: u64 = 0x142647298; const P3: u64 = 0x1415810d0; const P4: u64 = 0x142647268; const P5: u64 = 0x142689018;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x4f); rsp = ((rsp).wrapping_sub(0x88u64)) as u64; xmm0 = (crate::simd_ops::ru128(P0)) as u128; r14 = 0; fl_l = 0; fl_r = 0; rax = rbp.wrapping_add(0xf); xmm1 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); r9 = rbp.wrapping_sub(0x9); rsi = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), r14); r8 = ((r14.wrapping_add(0x17u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbp.wrapping_add(0x1f); crate::simd_ops::wu128(rbp.wrapping_add(0x1fu64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rcx); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); r8 = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), r14); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = rbp.wrapping_add(0x17); rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(0xfu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = r14; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), r14); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; rcx = r14; crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rcx); bb = 10; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; bb = 10; continue; }
            10 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rbp.wrapping_add(0x6fu64), ((r14 as u32)) as u32); rdx = rbp.wrapping_add(0x6f); rax = (crate::simd_ops::ru64(r8)) as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 20; continue; } bb = 12; continue; }
            12 => { rbx = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), r14); fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 20; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; r8 = rbp.wrapping_add(0x7f); rdx = ((rbx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 20; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp.wrapping_sub(0x9); rdi = 0xffffffff; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; rdi = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7fu64), r14); fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 13; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; bb = 21; continue; }
            21 => { rdi = 0x2; bb = 22; continue; }
            22 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(r8)) as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), r14); bb = 24; continue; }
            24 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { rax = ((rdi as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x88u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp.wrapping_add(0x7); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdi = 0x1; rdx = P5; r9 = 0; fl_l = 0; fl_r = 0; r8 = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rbx = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 20; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp.wrapping_add(0x77); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; rbx = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 33; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; bb = 22; continue; }
            33 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; rdi = 0x4; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; bb = 22; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158dd00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647278; const P1: u64 = 0x143413a20; const P2: u64 = 0x143413a30; const P3: u64 = 0x142647298; const P4: u64 = 0x1415810d0; const P5: u64 = 0x142647268; const P6: u64 = 0x14268dd38; const P7: u64 = 0x143931340; const P8: u64 = 0x142646940; const P9: u64 = 0x143931546; const P10: u64 = 0x1426472a8; const P11: u64 = 0x1426469c8; const P12: u64 = 0x1426472d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rax = ((rcx.wrapping_sub(0xaeu64)) as u32) as u32 as u64; rsi = rdx; r14 = ((rcx as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x4; rsp = ((rsp).wrapping_add(0xd0u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rbx); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rdi); rdi = 0x2; rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru32(rsi)) as u32 as u64; r9 = rbp.wrapping_sub(0x19); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsi, ((r12 as u32)) as u32); r8 = ((rdi.wrapping_add(0x15u64)) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(P1)) as u128; rbx = ((rax as u32)) as u32 as u64; rax = rbp.wrapping_sub(0x59); xmm1 = (crate::simd_ops::ru128(P2)) as u128; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((r15 as u32)) as u32); rcx = rbp.wrapping_sub(0x31); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), r12); crate::simd_ops::wu128(rbp.wrapping_sub(0x31u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rcx); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), r12); r8 = r12; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = rbp.wrapping_sub(0x51); rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(0xfu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = r12; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), r12); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; rcx = r12; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rcx); bb = 12; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; bb = 12; continue; }
            12 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x41u64), ((r12 as u32)) as u32); rdx = rbp.wrapping_sub(0x41); rax = (crate::simd_ops::ru64(r8)) as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 59; continue; } bb = 14; continue; }
            14 => { xmm0 = (crate::simd_ops::ru128(P1)) as u128; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0xb0; rax = rbp.wrapping_sub(0x49); xmm1 = (crate::simd_ops::ru128(P2)) as u128; rsi = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), r13); rsi = (if (((fl_l as u32)) == ((fl_r as u32))) { (rdi as u32) } else { (rsi as u32) }) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), (0xffffffffu32) as u32); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rbp.wrapping_sub(0x19); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r14); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbp.wrapping_sub(0x31); crate::simd_ops::wu128(rbp.wrapping_sub(0x31u64), (xmm0) as u128); r8 = ((r14.wrapping_add(0x17u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 26; continue; } bb = 15; continue; }
            15 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r14); r9 = rbp.wrapping_sub(0x39); rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rsi as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 23; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), (0xffffffffu32) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), (0xffffffffu32) as u32); bb = 23; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp.wrapping_sub(0x31); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x67u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; bb = 23; continue; }
            23 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; bb = 26; continue; }
            26 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { r13 = ((r14 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x41u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 58; continue; } bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; bb = 30; continue; }
            30 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 57; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; r8 = rbp.wrapping_sub(0x49); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r14); rdx = ((r13 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 54; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rdx = rbp.wrapping_sub(0x39); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 54; continue; } bb = 34; continue; }
            34 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rsi = r14; r15 = 0xffffffff; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbp.wrapping_sub(0x31); rcx = rdi; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 38; continue; } bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; r15 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            38 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; r8 = rbp.wrapping_sub(0x19); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), r14); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 45; continue; } bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 40; continue; }
            40 => { rax = 0; fl_l = 0; fl_r = 0; r8 = rbp.wrapping_sub(0x9); xmm0 = 0u128; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P6; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 44; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru16(rbp.wrapping_sub(0x9u64))) as u16 as u64; fl_r = (0x1fu16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 44; continue; } bb = 42; continue; }
            42 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { r8 = 0x103; rcx = P7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu16(P9, ((r14 as u16)) as u16); rsi = P7; bb = 44; continue; }
            44 => { rcx = rbp.wrapping_sub(0x9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 45; continue; }
            45 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            47 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 48; continue; }
            48 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 49; continue; }
            49 => { rax = ((r12 as u32)) as u32 as u64; rdi = (0xffffffffffffffffu64) as u64; r14 = (((rax) as i64).wrapping_mul((0x214u64) as i64) as u64) as u64; fl_l = ((r14).wrapping_add(rbx)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; crate::simd_ops::wu32(r14, ((r15 as u32)) as u32); bb = 50; continue; }
            50 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rsi.wrapping_add(rdi.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 50; continue; } bb = 51; continue; }
            51 => { rcx = 0x103; rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rcx; r8 = rsi; rdi = (if (((fl_l as u64)) > ((fl_r as u64))) { rcx } else { rdi }) as u64; rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; r9 = rdi; rcx = r14.wrapping_add(0x4); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; crate::simd_ops::wu16(r14.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0x4u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x39u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x20cu64), ((rax as u32)) as u32); rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(r14.wrapping_add(0x210u64), ((rax as u8)) as u8); r14 = 0; fl_l = 0; fl_r = 0; bb = 52; continue; }
            52 => { fl_l = (((r12 as u32)).wrapping_add(1)) as u32 as u64; r12 = ((fl_l as u32)) as u32 as u64; bb = 53; continue; }
            53 => { r15 = (crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32 as u64; bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 55; continue; }
            55 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 56; continue; }
            56 => { r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_sub(0x41u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 30; continue; } bb = 57; continue; }
            57 => { rbx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x21u64))) as u32 as u64; bb = 58; continue; }
            58 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; rdi = 0x1; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu32(rax, ((r12 as u32)) as u32); r12 = 0; fl_l = 0; fl_r = 0; bb = 59; continue; }
            59 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; bb = 60; continue; }
            60 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 62; continue; } bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), r12); bb = 62; continue; }
            62 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 63; continue; }
            63 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), r12); bb = 64; continue; }
            64 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 66; continue; } bb = 65; continue; }
            65 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 66; continue; }
            66 => { rax = ((rdi as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rsp = ((rsp).wrapping_add(0xd0u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158e170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158d520; const P1: u64 = 0x1415810d0; const P2: u64 = 0x142647268;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdx, (0xffffffffu32) as u32); r14 = ((r9 as u32)) as u32 as u64; rdi = r8; rsi = rdx; rbx = ((rcx as u32)) as u32 as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r8, r15); bb = 2; continue; }
            2 => { rcx = rbp.wrapping_sub(0x10); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; rcx = r15; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rcx); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r10)) as u64; r9 = rbp.wrapping_sub(0x8); r8 = ((r14 as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = r15; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rcx); bb = 6; continue; }
            6 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r10)) as u64; rcx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; r10 = r15; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), r15); bb = 9; continue; }
            8 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; bb = 9; continue; }
            9 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), r15); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(r10)) as u64; r9 = rbp.wrapping_add(0x38); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rbp.wrapping_add(0x38); rdx = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rbx = 0xffffffff; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp.wrapping_add(0x40); crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), r15); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rbx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; bb = 19; continue; }
            19 => { crate::simd_ops::wu32(rsi, ((rbx as u32)) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rdi, rcx); bb = 24; continue; }
            22 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = 0xffffffff; rbx = (rbx & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), r15); bb = 27; continue; }
            27 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru64(r10)) as u64; rcx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rax = (((rbx as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158e2e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158d520;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = ((rcx as u32)) as u32 as u64; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r10)) as u64; r9 = rsp.wrapping_add(0x28); r8 = ((rbx as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); bb = 4; continue; }
            4 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r10)) as u64; rcx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r10 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10); bb = 7; continue; }
            6 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x0u32) as u32); rdx = rsp.wrapping_add(0x48); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; bb = 11; continue; }
            10 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); bb = 13; continue; }
            13 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(r10)) as u64; rcx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); bb = 15; continue; }
            15 => { rax = ((rbx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158e3d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647278; const P1: u64 = 0x14158d520; const P2: u64 = 0x14158d5c0; const P3: u64 = 0x1426472d0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = rdx; crate::simd_ops::wu32(rdx, (0xffffffffu32) as u32); rsi = ((rcx as u32)) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; rbp = ((r8 as u32)) as u32 as u64; rdx = ((rdi.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); rbx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r10)) as u64; r9 = rsp.wrapping_add(0x28); r8 = ((rbp as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); bb = 4; continue; }
            4 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r10)) as u64; rcx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r10 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10); bb = 7; continue; }
            6 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r10)) as u64; r9 = rsp.wrapping_add(0x58); rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsp.wrapping_add(0x58); rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = rax; bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 16; continue; }
            16 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); bb = 18; continue; }
            18 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(r10)) as u64; rcx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); bb = 20; continue; }
            20 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = rdi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158e530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931bdc; const P1: u64 = 0x143931108; const P2: u64 = 0x1416064f0; const P3: u64 = 0x141606400; const P4: u64 = 0x143931be0; const P5: u64 = 0x1416063d0; const P6: u64 = 0x1415acff0; const P7: u64 = 0x142645290; const P8: u64 = 0x143931330; const P9: u64 = 0x1415f1f40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(P4)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xa0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(P1)) as u32 as u64; rcx = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P8, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            6 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
            7 => { rax = 0x1f; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158e5c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141524330; const P2: u64 = 0x142688ff8; const P3: u64 = 0x143931320;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = 0x8;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = P2; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(P3, rax); rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    crate::simd_ops::wu64(P3, (0x0u64) as u64);
    rax = 0x34;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158e610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931328; const P1: u64 = 0x1415edaf0; const P2: u64 = 0x1415eba50; const P3: u64 = 0x143931338; const P4: u64 = 0x1426472d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, (0x0u64) as u64); }
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(P3)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(P3, (0x0u8) as u8); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158e650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931338; const P1: u64 = 0x142647278; const P2: u64 = 0x1415ecaf0; const P3: u64 = 0x143931328;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rdx = 0x2; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; crate::simd_ops::wu8(P0, (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { 1u8 } else { 0u8 }) as u8); }
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(P3, rax);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158e6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415edd10; const P1: u64 = 0x142645358; const P2: u64 = 0x142645708; const P3: u64 = 0x142689008; const P4: u64 = 0x142645758; const P5: u64 = 0x14158f0f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<512, 261>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x828u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; r9 = rdx; r8 = rcx; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); rdx = 0x400; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x76; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x43; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x828u64)) as u64; return rax; }
            5 => { rax = 0x4; rsp = ((rsp).wrapping_add(0x828u64)) as u64; return rax; }
            6 => { rax = 0x42; rsp = ((rsp).wrapping_add(0x828u64)) as u64; return rax; }
            7 => { rax = 0x34; rsp = ((rsp).wrapping_add(0x828u64)) as u64; return rax; }
            8 => { rdx = P3; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x828u64)) as u64; return rax; }
            10 => { rax = 0x7; rsp = ((rsp).wrapping_add(0x828u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158e760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f20f0; const P1: u64 = 0x143931330; const P2: u64 = 0x1426455b0; const P3: u64 = 0x143931328; const P4: u64 = 0x1415edaf0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(P3)) as u64;
    crate::simd_ops::wu64(P1, (0x0u64) as u64);
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P3, (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158e7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931320; const P1: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(P0, (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158e830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931558; const P1: u64 = 0x143931560; const P2: u64 = 0x8; const P3: u64 = 0x58; const P4: u64 = 0x143815b50; const P5: u64 = 0x18;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = 0; fl_l = 0; fl_r = 0; rdi = r8; crate::simd_ops::wu64(rdx, r10); rbx = rdx; rax = (crate::simd_ops::ru64(P0)) as u64; r9 = (crate::simd_ops::ru32(P1)) as u32 as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r8 = ((r9.wrapping_add(rax)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = r8; rdx = ((rax.wrapping_add(P2)) as u64) as u64; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r10 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rcx = P4; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(rax)) as u64; rdx = rdi; rcx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158e8d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931568; const P1: u64 = 0x143931570; const P2: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rax = (crate::simd_ops::ru64(P0)) as u64; r11 = ((rdx as u32)) as u32 as u64; r9 = (crate::simd_ops::ru32(P1)) as u32 as u64; r10 = ((r9.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; r9 = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; rbx = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = rax; fl_r = rbx; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r8 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rdx)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            7 => { rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158e9f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931558; const P1: u64 = 0x143931560; const P2: u64 = 0x8; const P3: u64 = 0x58;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; r11 = 0; fl_l = 0; fl_r = 0; r8 = rcx; crate::simd_ops::wu64(rbx, r11); rax = (crate::simd_ops::ru64(P0)) as u64; r9 = (crate::simd_ops::ru32(P1)) as u32 as u64; r9 = ((r9).wrapping_shl(5u32)) as u64; r10 = ((r9.wrapping_add(rax)) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = r10; rcx = ((rax.wrapping_add(P2)) as u64) as u64; rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { r11 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rcx.wrapping_add(P3)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rcx = r8; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158ea70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffe5; const P1: u64 = 0x141524330; const P2: u64 = 0x141606570;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = 0x20;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158eaa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = rdx; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x5; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158ead0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rbx = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x5; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158eb00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143931558; const P1: u64 = 0x143931560; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; r8 = ((rax.wrapping_add(rdx)) as u64) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(0x20u64)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; rax = ((rdx.wrapping_add(P2)) as u64) as u64; fl_l = rdx; fl_r = r8; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158eb40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x28; const P5: u64 = 0xffffffffffffffd8; const P6: u64 = 0xffffffffffffffe8; const P7: u64 = 0xfffffffffffffff8; const P8: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp.wrapping_mul(4)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax.wrapping_add(P4)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_add(P5))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rbx).wrapping_add(P5), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_add(P6))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rbx).wrapping_add(P6), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(rcx).wrapping_add(P7))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(rbx).wrapping_add(P7), ((xmm0 as u64)) as u64); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158ec20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x20; const P5: u64 = 0xffffffffffffffe0; const P6: u64 = 0xfffffffffffffff0; const P7: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax.wrapping_add(P4)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_add(P5))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rbx).wrapping_add(P5), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_add(P6))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rbx).wrapping_add(P6), (xmm1) as u128); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158ecf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x143931548; const P2: u64 = 0x1426469c8; const P3: u64 = 0x14341ed88; const P4: u64 = 0x142646938; const P5: u64 = 0x143814f20; const P6: u64 = 0x14158ee40; const P7: u64 = 0x143931550; const P8: u64 = 0x14158f0f0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; r12 = 0x1; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { rbx = (0xffffffffffffffffu64) as u64; rax = rbx; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x2u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; rax = rax.wrapping_add(0x1); if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbp = ((rax.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; r15 = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P1, rax); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((r14.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 17; continue; }
            5 => { rax = ((rbp.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rdx = ((rax as u32)) as u32 as u64; rsi = rax.wrapping_sub(0x1); rax = rbx; bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rsi = rbx; bb = 9; continue; }
            9 => { rsi = ((rsi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(rsi.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 10; continue; }
            10 => { fl_l = ((rsi).wrapping_add(1)) as u64; rsi = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { r9 = rsi; r8 = rdi; rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(r14.wrapping_add(rsi.wrapping_mul(2)), ((rax as u16)) as u16); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { r10 = (crate::simd_ops::ru64(P1)) as u64; rax = ((rbp.wrapping_sub(0x3u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r10.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = (0x5cu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(r10.wrapping_add(rbx.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 13; continue; } bb = 14; continue; }
            14 => { rbp = (((rbp as u32)).wrapping_sub((rbx as u32))) as u32 as u64; r8 = P3; rdx = r15; rcx = r10; rax = ((rbp.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (r12 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { r8 = P5; rdx = ((r12 as u32)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(P7)) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158ee40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931568; const P1: u64 = 0x143931570; const P2: u64 = 0x143931574; const P3: u64 = 0x14158eb40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; rdi = ((rdx as u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_shl(12u32)) as u32 as u64; rsi = r8; r8 = (crate::simd_ops::ru32(P1)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rcx as u32))) as u32 as u64; rdi = (((rdi as u32)).wrapping_shl(4u32)) as u32 as u64; rax = r9; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; r14 = ((r8 as u32)) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; rdx = ((rcx.wrapping_mul(8)) as u64) as u64; rcx = ((rdx.wrapping_add(r9)) as u64) as u64; rdx = ((rdx).wrapping_add(r9)) as u64; fl_l = r9; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbp = 0; fl_l = 0; fl_r = 0; rdx = rax.wrapping_add(0x8); fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 20; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rsi)) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = r9; fl_l = r9; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 10; continue; }
            10 => { fl_l = rax; fl_r = rcx; rbx = rax.wrapping_add(0x8); rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = r14; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; r8 = (crate::simd_ops::ru32(P1)) as u32 as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; bb = 14; continue; }
            14 => { rax = ((rcx as u32)) as u32 as u64; rbx = rbp; fl_l = r14; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { rax = ((r8 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P1, ((r8 as u32)) as u32); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((r9.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); rbx = rax.wrapping_add(0x8); bb = 17; continue; }
            17 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm1) as u128); bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = rbx; rax = 0x34; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = 0x1; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; bb = 20; continue; }
            19 => { rax = 0x1f; bb = 20; continue; }
            20 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158ef90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931560; const P1: u64 = 0x143931558; const P2: u64 = 0x143931564; const P3: u64 = 0x14158ec20;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rdi = ((r8 as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rdi = (((rdi as u32)).wrapping_shl(12u32)) as u32 as u64; r10 = r9; r9 = (crate::simd_ops::ru64(P1)) as u64; rdi = (((rdi as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdi = (((rdi as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = r9; rsi = ((r8 as u32)) as u32 as u64; rcx = ((rdx.wrapping_add(r9)) as u64) as u64; rdx = ((rdx).wrapping_add(r9)) as u64; fl_l = r9; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbp = 0; fl_l = 0; fl_r = 0; rdx = rax.wrapping_add(0x8); fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10); fl_l = r9; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rcx; rbx = rax.wrapping_add(0x8); rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = P1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; r8 = (crate::simd_ops::ru32(P0)) as u32 as u64; r9 = (crate::simd_ops::ru64(P1)) as u64; bb = 13; continue; }
            13 => { rax = ((rcx as u32)) as u32 as u64; rbx = rbp; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = ((r8 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(P0, ((r8 as u32)) as u32); fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); rbx = rax.wrapping_add(0x8); bb = 16; continue; }
            16 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); bb = 17; continue; }
            17 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rbx; fl_r = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0x34; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = 0x1; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158f0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142688cd8; const P1: u64 = 0x14158ee40; const P2: u64 = 0x143931560; const P3: u64 = 0x143931558; const P4: u64 = 0x143931564; const P5: u64 = 0x14158ec20;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rdi = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r12); r12 = P0; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); r14 = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; r8 = rdi.wrapping_add(0x50); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            4 => { rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rbp = (crate::simd_ops::ru32(P2)) as u32 as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; rdx = ((rbp as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; r10 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rsi = (((rsi as u32)).wrapping_shl(12u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_add(crate::simd_ops::ru32(rdi.wrapping_add(0xcu64)))) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rsi = (((rsi as u32)).wrapping_shl(4u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(P3)) as u64; rcx = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rcx; rdx = rax.wrapping_add(0x8); rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r15 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 13; continue; }
            13 => { fl_l = rax; fl_r = rcx; rbx = rax.wrapping_add(0x8); rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { r15 } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(P4)) as u32 as u64; rbx = r15; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(P2)) as u32 as u64; crate::simd_ops::lk_add_u32(P2, (1) as u32); rax = ((rax).wrapping_shl(5u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(P3))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rax, ((rsi as u32)) as u32); rbx = rax.wrapping_add(0x8); bb = 19; continue; }
            19 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); bb = 20; continue; }
            20 => { fl_l = rbx; fl_r = rbx; rax = 0x34; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { (r14 as u32) } else { (rax as u32) }) as u32 as u64; bb = 21; continue; }
            21 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rcx = rsp.wrapping_add(0x70); rdx = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(r9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 25; continue; }
            25 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((r14 as u32)) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x58u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x58u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158f2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931558; const P1: u64 = 0x143931560; const P2: u64 = 0x1415242e0; const P3: u64 = 0x143931564; const P4: u64 = 0x143931568; const P5: u64 = 0x143931570; const P6: u64 = 0x143931574; const P7: u64 = 0x143931548;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(P0)) as u64;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(P1, ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rbx); crate::simd_ops::wu32(P3, ((rbx as u32)) as u32); }
    rdx = (crate::simd_ops::ru64(P4)) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(P5, ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P4, rbx); crate::simd_ops::wu32(P6, ((rbx as u32)) as u32); }
    rdx = (crate::simd_ops::ru64(P7)) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P7, rbx); }
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158f340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; fl_r = 0x7e601;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = 0; fl_l = 0; fl_r = 0; rcx = 0xa; fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rax = (if (((fl_l as u8)) != ((fl_r as u8))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; return rax; }
    rax = 0x2c;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158f360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143814f40; const P1: u64 = 0x80;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = P0; rdx = 0; fl_l = 0; fl_r = 0; r9 = r11; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r9)) as u32 as u64; r8 = ((rax.wrapping_add(P1)) as u32) as u32 as u64; r8 = ((((r8 as u32)) | ((rdx as u32)))) as u32 as u64; r8 = ((((r8 as u32)) | (0x80000000u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r8 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(r11.wrapping_add(rdx.wrapping_mul(4)), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; r9 = ((r9).wrapping_add(0x4u64)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x8; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rax = 0xffffffff; return rax; }
            5 => { crate::simd_ops::wu32(rcx, ((r8 as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158f3b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143814f40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((rcx as u32)) as u32 as u64;
    rdx = P0;
    rax = ((((rax as u32)) & (0x7fu32))) as u32 as u64;
    rdx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = ((rax as u32)) as u32 as u64; fl_l = ((((rcx as u32)) >> (31u32)) & 1) as u64; rcx = (((rcx as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158f430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143814f40; const P1: u64 = 0x14397d1e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    r9 = ((rcx as u32)) as u32 as u64;
    rax = P0;
    r9 = ((((r9 as u32)) & (0x7fu32))) as u32 as u64;
    r11 = rdx;
    r10 = (crate::simd_ops::ru32(rax.wrapping_add(r9.wrapping_mul(4)))) as u32 as u64;
    fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = ((crate::exe::tls_pointer() as u64)) as u64; r8 = (crate::simd_ops::ru32(P1)) as u32 as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)))) as u64; rax = 0x10; rax = ((rax).wrapping_add(rdx)) as u64; crate::simd_ops::wu32(rax.wrapping_add(r9.wrapping_mul(4)), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rax = 0x30; rcx = (if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { r11 } else { rcx }) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(r9.wrapping_mul(8)), rcx); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158f480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x142689140; const P2: u64 = 0x1439315b0; const P3: u64 = 0x1439315b8; const P4: u64 = 0x1439315a8; const P5: u64 = 0x142689150; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rsi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rdx = ((rax.wrapping_add(0x50u64)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(0x4u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdi); rax = P1; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); fl_l = (crate::simd_ops::ru64(P3)) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(P3, rbx); bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); bb = 5; continue; }
            5 => { crate::simd_ops::lk_add_u32(P4, (1) as u32); rax = P5; crate::simd_ops::wu64(P2, rbx); rcx = rbx; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rdi); crate::simd_ops::wu16(rbx.wrapping_add(0x48u64), ((rdi as u16)) as u16); crate::simd_ops::wu8(rbx.wrapping_add(0x4au64), (0x1u8) as u8); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbx); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rsi); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rdi); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158f5a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x70; const P2: u64 = 0x90; const P3: u64 = 0x98; const P4: u64 = 0xa8; const P5: u64 = 0xb0; const P6: u64 = 0xa0; const P7: u64 = 0x9c; const P8: u64 = 0xa4; const P9: u64 = 0x10; const P10: u64 = 0x20; const P11: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = rcx; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), r8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P1), r8); xmm0 = 0u128; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P6), r8); rcx = ((r8 as u32)) as u32 as u64; fl_l = (((rcx) >> (3u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (3u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (4u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (4u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (25u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (25u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (26u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (26u32))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; fl_l = (((rcx) >> (28u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (28u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (29u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (29u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (30u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (30u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (31u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (31u32))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; fl_l = (((rcx) >> (0u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (0u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (1u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (1u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (2u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (2u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (3u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (3u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (4u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (4u32))) as u64; rcx = ((rcx as u32)) as u32 as u64; fl_l = (((rcx) >> (5u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (5u32))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rcx as u32)) as u32); crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(P9), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(P10), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(P11), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rax); rax = rdx; crate::simd_ops::wu32(rdx.wrapping_add(0x70u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), r8); crate::simd_ops::wu32(rdx.wrapping_add(0xb0u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x7cu64), r8); crate::simd_ops::wu64(rdx.wrapping_add(0x84u64), r8); fl_l = (crate::simd_ops::lk_and_u8(rdx.wrapping_add(0xb7u64), (0xf8u8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu32(rdx.wrapping_add(0x8cu64), ((r8 as u32)) as u32); crate::simd_ops::wu16(rdx.wrapping_add(0xb4u64), ((r8 as u16)) as u16); crate::simd_ops::wu32(rdx.wrapping_add(0x78u64), (0x3f800000u32) as u32); crate::simd_ops::wu8(rdx.wrapping_add(0xb6u64), ((r8 as u8)) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158f6c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415ae300; const P1: u64 = 0x142689080; const P2: u64 = 0x142689110; const P3: u64 = 0x1411ffc90; const P4: u64 = 0x14158f5a0; const P5: u64 = 0x141607960; const P6: u64 = 0x143815af0; const P7: u64 = 0x141591e90; const P8: u64 = 0x1415af330;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), rdx); rdx = rax.wrapping_sub(0x38); crate::simd_ops::wu32(rax.wrapping_sub(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_sub(0x28u64), ((rbp as u32)) as u32); rsi = (((r9 as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), rbp); rdi = r8; rbx = rcx; crate::simd_ops::wu8(rax.wrapping_sub(0x20u64), (0xffu8) as u8); crate::simd_ops::wu8(rax.wrapping_sub(0x18u64), (0xffu8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P1; crate::simd_ops::wu64(rbx, rax); rcx = rbx.wrapping_add(0x78); rax = P2; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x88); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; rcx = rbx.wrapping_add(0x178); crate::simd_ops::wu128(rbx.wrapping_add(0x140u64), (xmm0) as u128); rax = 0; fl_l = 0; fl_r = 0; xmm1 = 0u128; crate::simd_ops::wu128(rbx.wrapping_add(0x150u64), (xmm1) as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x160u64), ((rax as u32)) as u32); crate::simd_ops::wu128(rbx.wrapping_add(0x164u64), (xmm0) as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x174u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x198u64), rbp); rsi = (rsi & !0xffu64) | ((((((rsi as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu64(rbx.wrapping_add(0x1a0u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x1a8u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x1b0u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x1b8u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x1c0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x1c8u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x1d0u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x1d8u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x1e0u64), ((rbp as u32)) as u32); rcx = (crate::simd_ops::ru32(P6)) as u32 as u64; rsi = (rsi & !0xffu64) | (((((rsi as u8)).wrapping_shl(6u32)) as u8 as u64)); rsi = (rsi & !0xffu64) | ((((((rsi as u8)) | (0x10u8))) as u8 as u64)); rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(P6, ((rax as u32)) as u32); crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x1f9u64), (0xfcu8) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x1e8u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x1f0u64), rbp); crate::simd_ops::wu8(rbx.wrapping_add(0x1f8u64), ((rsi as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x1e4u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x1fcu64), (0x3f7fc105u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x148u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x98u64))) as u32 as u64; fl_l = rax; fl_r = (0xe0000u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shl(7u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rsi as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x1f8u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            12 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rax as u32)) as u32); bb = 13; continue; }
            13 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x24u64), (1) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = rbx; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x164u64), (0x42ca0000u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158f890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142689080; const P1: u64 = 0x142689110; const P2: u64 = 0x141592c70; const P3: u64 = 0x1401f53e0; const P4: u64 = 0x141591580; const P5: u64 = 0x1415aef20; const P6: u64 = 0x1415ccba0; const P7: u64 = 0x1415242e0; const P8: u64 = 0x141607970; const P9: u64 = 0x1415c20b0; const P10: u64 = 0x1426712e0; const P11: u64 = 0x1415ae370;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); crate::simd_ops::wu64(rcx, rax); rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rax = P1; rsi = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; rax = P3; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 3; continue; }
            2 => { rcx = rsi; rax = crate::simd_ops::call_stk(rdx, rax, rcx, rdx, r8, r9, rsp); rbx = (((rax as u8)) as u32) as u32 as u64; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = P4; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (0x7ffffe3fffe60bdu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = rsp.wrapping_add(0x30); bb = 6; continue; }
            5 => { rdx = rsp.wrapping_add(0x30); rcx = rsi; rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { r8 = (((rbx as u8)) as u32) as u32 as u64; rdx = rax; rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x198u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x1a0u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x198u64), rbp); crate::simd_ops::wu32(rsi.wrapping_add(0x1a4u64), ((rbp as u32)) as u32); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x1a8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x1b0u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x1a8u64), rbp); crate::simd_ops::wu32(rsi.wrapping_add(0x1b4u64), ((rbp as u32)) as u32); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x1e8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1f0u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x1e8u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x1f0u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x1e8u64), rbp); crate::simd_ops::wu32(rsi.wrapping_add(0x1f4u64), ((rbp as u32)) as u32); bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x1d0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x1d8u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x1d0u64), rbp); crate::simd_ops::wu32(rsi.wrapping_add(0x1dcu64), ((rbp as u32)) as u32); bb = 17; continue; }
            17 => { rcx = rsi.wrapping_add(0x178); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0xf8u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0xf0u64), rbp); crate::simd_ops::wu32(rsi.wrapping_add(0xfcu64), ((rbp as u32)) as u32); bb = 19; continue; }
            19 => { rcx = rsi.wrapping_add(0x78); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = P10; rcx = rsi; crate::simd_ops::wu64(rsi.wrapping_add(0x68u64), rax); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158fa60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142689048; const P1: u64 = 0x142689070; const P2: u64 = 0x8; const P3: u64 = 0x14160a090; const P4: u64 = 0x78; const P5: u64 = 0x80; const P6: u64 = 0x1415242e0; const P7: u64 = 0x84; const P8: u64 = 0x1415cc8f0; const P9: u64 = 0x142689028;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P5), (0x0u32) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P7), (0x0u32) as u32); }
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rax = P9;
    crate::simd_ops::wu64(rbx, rax);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158fb70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158f890; const P1: u64 = 0x141834d30;
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x208; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158fc60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143930fa0; const P2: u64 = 0x1415c4320; const P3: u64 = 0x1415c2db0; const P4: u64 = 0xd; const P5: u64 = 0x1415cca50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = ((r9 as u32)) as u32 as u64; rbx = r8; rdi = rdx; rbp = rcx; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu32(r14, (0x0u32) as u32); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = rbp.wrapping_add(0x8); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r14))).to_bits() as u128); rcx = rbp.wrapping_add(0x1d0); crate::simd_ops::wu32(r14, ((xmm0 as u32)) as u32); rdx = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rbp.wrapping_add(0x58); rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u32) as u32 as u64; rcx = rbp; crate::simd_ops::wu8(rsp.wrapping_add(0x5cu64), ((rax as u8)) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rdi, rax); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415907b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c8; const P1: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141590920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; r10 = rdx; rdi = rcx; r9 = ((r11 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; bb = 2; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r9 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = rcx; fl_r = r10; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_add(P2)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rdx } else { r11 }) as u64; rax = r11; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rdx } else { r11 }) as u64; rax = r11; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415909c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1f0; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0x18; const P5: u64 = 0x50; const P6: u64 = 0xffffffffffffffe0; const P7: u64 = 0xffffffffffffffe8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbp = rcx; rsi = ((r14 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((rdx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1e8u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x1e8u64))) as u64; rbx = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdi).wrapping_add(P5))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P6), rcx); crate::simd_ops::wu32(rbx.wrapping_add(P7), ((r14 as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x1f0u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141590a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1f8; const P1: u64 = 0x150; const P2: u64 = 0x178; const P3: u64 = 0x8; const P4: u64 = 0x1c0; const P5: u64 = 0x14159f2d0; const P6: u64 = 0x160; const P7: u64 = 0x18c; const P8: u64 = 0x1b8; const P9: u64 = 0x14159e8e0; const P10: u64 = 0x10; const P11: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; rax = ((rcx.wrapping_add(P1)) as u64) as u64; rbx = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r9 = ((rcx.wrapping_add(P2)) as u64) as u64; r8 = rax; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P7))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = ((rbx.wrapping_add(P8)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
            5 => { fl_l = (crate::simd_ops::lk_and_u8(rax.wrapping_add(P10), (0xf0u8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu32(rax, (0x3f000000u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P11), (0x3f800000u64) as u64); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x42c80000u32) as u32); bb = 6; continue; }
            6 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141590ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141605340; const P1: u64 = 0x143426564; const P2: u64 = 0x142673708; const P3: u64 = 0x1426736fc; const P4: u64 = 0x142673700; const P5: u64 = 0x1426736ec; const P6: u64 = 0x1426736e8; const P7: u64 = 0x1426736f0; const P8: u64 = 0x143426d10; const P9: u64 = 0x142689160; const P10: u64 = 0x14158c580; const P11: u64 = 0x14158d2f0; const P12: u64 = 0x14158c3d0; const P13: u64 = 0x141603910; const P14: u64 = 0x1426891cc; const P15: u64 = 0x143815b3c; const P16: u64 = 0x14158ec20; const P17: u64 = 0x142689270; const P18: u64 = 0x142689280;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xf0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm6) as u128); rbp = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = rdx; crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x88u64), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x98u64), (xmm11) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xa8u64), (xmm12) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm13) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm14) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm15) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x13eu64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; xmm0 = 0u128; xmm12 = ((crate::simd_ops::ru32(P1)) as u32) as u128; rdi = rax; xmm13 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm11 = 0u128; xmm15 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm7 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm8 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm9 = ((crate::simd_ops::ru32(P6)) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(P7)) as u32) as u128; xmm6 = ((crate::simd_ops::ru32(P8)) as u32) as u128; xmm14 = ((crate::simd_ops::ru32(P9)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x140u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x130u64), ((xmm0 as u32)) as u32); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { xmm2 = ((crate::simd_ops::ru32(rbp.wrapping_add(0xb0u64))) as u32) as u128; xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm13 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x130u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x130u64))) as u32) as u128; xmm1 = (xmm0) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x140u64), ((rax as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x140u64)))).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm2 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x140u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x130u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x130u64))) as u32) as u128; xmm11 = (xmm0) as u128; xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) + f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x140u64)))).to_bits() as u128); bb = 5; continue; }
            4 => { xmm11 = (xmm6) as u128; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xe0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0xe4u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x140u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0xe8u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x130u64), ((xmm0 as u32)) as u32); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rsi = rdi.wrapping_add(0x20); bb = 8; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xe1u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = rdi.wrapping_add(0x70); bb = 11; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x130u64))) as u32) as u128; rcx = rsi.wrapping_add(0x30); xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x140u64))) as u32) as u128; xmm2 = (xmm11) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rdx = rbx; rcx = rax; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x160u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; r13 = 0x1; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xe0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = rdi.wrapping_add(0x20); bb = 16; continue; }
            15 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; r13 = (crate::simd_ops::ru32(rax.wrapping_add(0x48u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); bb = 17; continue; }
            17 => { rsi = 0; fl_l = 0; fl_r = 0; r12 = rbp.wrapping_add(0x104); bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(r12)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 19; continue; }
            19 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 33; continue; } bb = 20; continue; }
            20 => { r14 = rcx; r15 = ((r13 as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru32(P14)) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(P15)) as u32) as u128; rcx = (((rcx as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x88u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 32; continue; } bb = 22; continue; }
            22 => { rdi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 24; continue; }
            24 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 32; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 32; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rdx, rax); rax = (crate::simd_ops::ru32(r12)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru32(P14)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rsi as u32))) as u32 as u64; xmm2 = ((crate::simd_ops::ru32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x88u64))) as u32) as u128; xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm13 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { xmm1 = 0u128; bb = 31; continue; }
            28 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x148u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x148u64))) as u32) as u128; xmm1 = (xmm0) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x30u64)))).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm2 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x148u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x148u64))) as u32) as u128; xmm1 = (xmm0) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x30u64)))).to_bits() as u128); bb = 31; continue; }
            30 => { xmm1 = (xmm6) as u128; bb = 31; continue; }
            31 => { crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((xmm1 as u32)) as u32); rcx = (crate::simd_ops::ru32(P17)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rsi as u32))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(P18)) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0x2u32) as u32); bb = 32; continue; }
            32 => { r14 = ((r14).wrapping_add(0x8u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; r12 = ((r12).wrapping_add(0x4u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 18; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1c8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x130u64))) as u32) as u128; xmm2 = (xmm11) as u128; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x140u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            36 => { xmm14 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r11 = rsp.wrapping_add(0xf0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x60u64))) as u128; xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x70u64))) as u128; xmm13 = (crate::simd_ops::ru128(r11.wrapping_sub(0x80u64))) as u128; xmm15 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141590fb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1f8; const P1: u64 = 0x1c0; const P2: u64 = 0x14159ed70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x40u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141590fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141605350; const P2: u64 = 0x141590920; const P3: u64 = 0x1415d7950; const P4: u64 = 0x1415427d0; const P5: u64 = 0x38; const P6: u64 = 0x20; const P7: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = r8; rdi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(P5))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) <= ((fl_r as u16))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P7))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P7), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P6))) as u128; rax = 0x1; crate::simd_ops::wu128(rbx.wrapping_add(P6), (xmm0) as u128); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14158f480; const P2: u64 = 0x28; const P3: u64 = 0x20; const P4: u64 = 0x10; const P5: u64 = 0x30; const P6: u64 = 0x40; const P7: u64 = 0x50; const P8: u64 = 0x60; const P9: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = r8; rdi = ((rdx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rdx = (((rdi) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(rax).wrapping_add(P4))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P4), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax).wrapping_add(P3))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P3), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(rax).wrapping_add(P5))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P5), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax).wrapping_add(P6))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P6), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(rax).wrapping_add(P7))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P7), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax).wrapping_add(P8))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P8), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(rax).wrapping_add(P9))) as u64) as u128; rax = 0x1; crate::simd_ops::wu64(rbx.wrapping_add(P9), ((xmm1 as u64)) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x38; const P2: u64 = 0x20; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru16(r9.wrapping_add(P1))) as u32) as u32 as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((fl_l as u32)) >= ((fl_r as u32)))) { rax = ((rdx as u32)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P3))) as u128; crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P2))) as u128; rax = 0x1; crate::simd_ops::wu128(r8.wrapping_add(P2), (xmm0) as u128); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0x2;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbx = r9; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; r11 = ((r8 as u32)) as u32 as u64; r10 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = rax.wrapping_add(0x8); fl_l = rax; fl_r = rcx; r8 = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { r8 }) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r9 = ((r11 as u32)) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { rcx = ((r9 as u32)) as u32 as u64; r10 = ((r9 as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r8)) as u64; rdx = rdx.wrapping_add(0x10); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax).wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(rbx).wrapping_sub(0x10u64), (xmm0) as u128); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 11; continue; }
            11 => { rcx = ((rcx).wrapping_shl(4u32)) as u64; r10 = ((r9.wrapping_sub(0x1u64)) as u32) as u32 as u64; r10 = ((r10).wrapping_add(r10)) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; r11 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rdx = ((r11 as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = rcx.wrapping_add(0x10); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r10.wrapping_mul(8)))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x10u64), (xmm0) as u128); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = rbx.wrapping_add(0x8); rcx = r11; bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), rdx); crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), (0x42c80000u32) as u32); rax = rax.wrapping_add(0x10); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141605350; const P2: u64 = 0x141590920; const P3: u64 = 0x1415d7950; const P4: u64 = 0x60; const P5: u64 = 0x70; const P6: u64 = 0x10; const P7: u64 = 0x80; const P8: u64 = 0x20; const P9: u64 = 0x90; const P10: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = r8; rdi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P4))) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P6), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P7))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P8), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64) as u128; rax = 0x1; crate::simd_ops::wu64(rbx.wrapping_add(P10), ((xmm1 as u64)) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415912b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x2c;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (((r8 as u8)) as u32) as u32 as u64;
    rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64;
    rax = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415912d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x198; const P1: u64 = 0x1a0; const P2: u64 = 0x70; const P3: u64 = 0x141590920; const P4: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = ((r8 as u32)) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rbp = rdx; r14 = ((r14).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = ((rsi.wrapping_add(P4)) as u32) as u32 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rsi as u32) } else { (rcx as u32) }) as u32 as u64; rbx = ((rbx).wrapping_add(0x88u64)) as u64; rsi = ((rcx as u32)) as u32 as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 8; continue; }
            8 => { rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 9; continue; }
            9 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1f8; const P1: u64 = 0x1c0; const P2: u64 = 0x100;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x40u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r14 = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 29; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x210u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x218); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x258u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x250u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rsi = rbx.wrapping_add(0x218); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 25; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x210u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x210u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 19; continue; } bb = 21; continue; }
            21 => { bb = 26; continue; }
            22 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x240u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x248u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            25 => { rsi = rbx.wrapping_add(0x218); bb = 26; continue; }
            26 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 28; continue; }
            28 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 29; continue; }
            29 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415914d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x198; const P1: u64 = 0x1a0; const P2: u64 = 0x70; const P3: u64 = 0x141590920;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = ((r9 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rsi = ((r8 as u32)) as u32 as u64; r14 = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rbp = rdx; r14 = ((r14).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { bb = 9; continue; }
            5 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (((rdi as u32)).wrapping_sub(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rax = rbx; bb = 11; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (0x7ffffe3fffe60bdu64) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415915a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x38; const P5: u64 = 0xffffffffffffffc8; const P6: u64 = 0xffffffffffffffd8; const P7: u64 = 0xffffffffffffffe8; const P8: u64 = 0xfffffffffffffff8; const P9: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rbp as u32)) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 11; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx.wrapping_add(P4)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P5))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdi).wrapping_add(P5), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P6))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdi).wrapping_add(P6), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P7))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdi).wrapping_add(P7), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(P8))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdi).wrapping_add(P8), ((xmm1 as u64)) as u64); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 13; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 14; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = rdi; r8 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(0x4u64), (0x8u8) as u8); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r8); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rax; rdx = ((rdx).wrapping_sub(rdi)) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x142689048; const P3: u64 = 0x142689070; const P4: u64 = 0x1401de2f0; const P5: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r12 = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rsi = rcx; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r12 = (((r12 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((r12 as u32)) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((r12 as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 14; continue; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 7; continue; }
            7 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); r12 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rbx = rbp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = (0xffffffffffffffd0u64) as u64; r15 = ((r15).wrapping_sub(rbp)) as u64; rbp = P3; r13 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rcx = rbx.wrapping_sub(0x30); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx.wrapping_sub(0x20u64), r13); crate::simd_ops::wu32(rbx.wrapping_sub(0x18u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_sub(0x10u64), ((r13 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_sub(0x8u64), (0xffu8) as u8); crate::simd_ops::wu8(rbx, (0xffu8) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r13); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r13); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), r13); crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x28u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), r13); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r13); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), r13); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), r13); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), r13); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), r13); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), r13); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rdi = ((r15.wrapping_add(rbx)) as u64) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(rdi)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x88u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 12; continue; }
            12 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsi, rbp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((r12 as u32)) as u32); rsp = ((rsp).wrapping_add(0x38u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415918c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((rbp as u32)) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 13; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 14; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { r9 = (0xffffffffffffffccu64) as u64; rdx = rdi.wrapping_add(0x34); r9 = ((r9).wrapping_sub(rdi)) as u64; r10 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { r8 = rdx.wrapping_sub(0x34); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdx.wrapping_sub(0x4u64), r10); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((r10 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), r10); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r10); crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), (0x42c80000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x30u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x34u64), ((r10 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x3cu64), r10); crate::simd_ops::wu64(rdx.wrapping_add(0x44u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x4cu64), ((r10 as u32)) as u32); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((r9.wrapping_add(rdx)) as u64) as u64; rdx = ((rdx).wrapping_add(0x88u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x70u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x80u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x80u64), ((xmm0 as u64)) as u64); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c0; const P1: u64 = 0x70; const P2: u64 = 0x18; const P3: u64 = 0x38; const P4: u64 = 0x30; const P5: u64 = 0x1415ae930; const P6: u64 = 0x141591ae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx.wrapping_add(P2), rsi); crate::simd_ops::wu64(rsi.wrapping_add(P1), rax); bb = 4; continue; }
            3 => { crate::simd_ops::wu64(rsi.wrapping_add(P1), rax); crate::simd_ops::wu64(rcx.wrapping_add(P2), rsi); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x30); rcx = rsi; rbx = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = (((rbx as u8)) as u32) as u32 as u64; rdx = rdi; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159eb80; const P1: u64 = 0x14159f1e0; const P2: u64 = 0x141591750; const P3: u64 = 0x142689048; const P4: u64 = 0x142689070; const P5: u64 = 0x1416087b0; const P6: u64 = 0x14183acc2; const P7: u64 = 0x14159f0c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), rbx); rdx = rbp; r13 = rcx; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rsi); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1c0u64))) as u64; r14 = 0x1; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rbp, r15); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x1c0u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r12 = ((r14.wrapping_sub(0x2u64)) as u32) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 38; continue; } bb = 2; continue; }
            2 => { rdx = ((rsi as u32)) as u32 as u64; bb = 4; continue; }
            3 => { rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 4; continue; }
            4 => { rbx = r13.wrapping_add(0x1e8); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x34; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r14 as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r14 = 0x2; bb = 38; continue; }
            7 => { rdx = P3; r8 = P4; fl_l = (crate::simd_ops::ru64(rbp)) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = ((rax as u32)) as u32 as u64; rcx = rbx; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r14 as u32) } else { (rdx as u32) }) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rdx = P3; r8 = P4; bb = 11; continue; }
            11 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r15); crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x28u64), (0xffu8) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x30u64), (0xffu8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), r15); crate::simd_ops::wu64(rcx, rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r8); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), r15); bb = 15; continue; }
            14 => { rcx = r15; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbp)) as u64; r8 = r13; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { r14 = ((rax.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 38; continue; }
            17 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 18; continue; }
            18 => { rax = rsi; r15 = rsi; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = (0xffffffffffffff0u64) as u64; bb = 20; continue; }
            20 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rsp = ((rsp).wrapping_sub(rcx)) as u64; r8 = ((rsi as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rdi = rsp.wrapping_add(0x20); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x1c0u64))) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 22; continue; }
            22 => { rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rdx = ((rax as u32)) as u32 as u64; rcx = rbx; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r14 as u32) } else { (rdx as u32) }) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; bb = 25; continue; }
            25 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 29; continue; } bb = 26; continue; }
            26 => { rax = ((rdx as u32)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rcx.wrapping_add(0x28u64), (0xffu8) as u8); rax = P3; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rcx, rax); rax = P4; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x30u64), (0xffu8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rsi); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), rsi); bb = 30; continue; }
            28 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            29 => { rsi = 0; fl_l = 0; fl_r = 0; rcx = ((rsi as u32)) as u32 as u64; bb = 30; continue; }
            30 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = r13; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(0xffffffffffffff78u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), (0x2u32) as u32); bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 33; continue; }
            33 => { rax = ((r12 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rdi, rsi); bb = 36; continue; }
            36 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = r14; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 37; continue; }
            37 => { r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) as u32 as u64; bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru64(rbp)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 39; continue; }
            39 => { let xa_src: u32 = (r12 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r12 = (xa_old) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 41; continue; }
            41 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rax = ((r14 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rsp = rbp.wrapping_add(0x10); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591df0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160ba50; const P1: u64 = 0x1c8;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x13e; const P1: u64 = 0x104; const P2: u64 = 0x1c8; const P3: u64 = 0x8; const P4: u64 = 0x38; const P5: u64 = 0xce; const P6: u64 = 0x28; const P7: u64 = 0x64; const P8: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rcx.wrapping_add(P1)) as u64) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rdx; fl_r = (0x4u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { xmm0 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x80; const P2: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = rax; fl_r = (0xe0000u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rax; bb = 5; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = rax; fl_r = (0xe0000u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591ed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1f8; const P1: u64 = 0x68; const P2: u64 = 0x70; const P3: u64 = 0x78; const P4: u64 = 0x88; const P5: u64 = 0x143426564; const P6: u64 = 0x142673708; const P7: u64 = 0x1426736fc; const P8: u64 = 0x142673700; const P9: u64 = 0x1426736ec; const P10: u64 = 0x1426736e8; const P11: u64 = 0x1426736f0; const P12: u64 = 0x143815b38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P6)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm1 = 0u128; bb = 7; continue; }
            6 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(P8))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; xmm1 = (xmm0) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P9))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P10))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P11))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x40u64)))).to_bits() as u128); bb = 7; continue; }
            7 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x100u64)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(P12)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c8; const P1: u64 = 0x30; const P2: u64 = 0x8; const P3: u64 = 0x3a;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0x2u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1f8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(P0), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141591ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160a240;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r9 as u8)) as u8);
    r9 = ((rdx as u32)) as u32 as u64;
    rdx = 0x1;
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141592010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c0; const P1: u64 = 0x20; const P2: u64 = 0x1416074e0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = rdx;
    rcx = r8;
    rdx = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141592030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1f8; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0xc; const P4: u64 = 0x4; const P5: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x40u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32) as u128; r8 = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x35u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), (0x1u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141592080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140; const P1: u64 = 0x143930fa8; const P2: u64 = 0x1415f80b0; const P3: u64 = 0x1415f8370;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (((rdx as u8)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            2 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415920b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x141592830; const P2: u64 = 0x141592510; const P3: u64 = 0x141607dd0;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; rdi = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rbx = (rcx) as u64; xmm6 = (xmm1) as u128; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 36; continue; } bb = 1; continue; }
            1 => { rax = ((r8.wrapping_sub(0x12u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); xmm7 = 0u128; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x34u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 28; continue; } bb = 2; continue; }
            2 => { rcx = ((P0) as u64) as u64; rax = (rax as u32 as i32 as i64) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rax).wrapping_add(0x1592324u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x15922e0u64))) as u32 as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; j_tgt = (rdx) as u64; bb = 0xffff_fffe; continue; }
            3 => { crate::simd_ops::wu32(rbx.wrapping_add(0x180u64), ((xmm6 as u32)) as u32); bb = 28; continue; }
            4 => { crate::simd_ops::wu32(rbx.wrapping_add(0x184u64), ((xmm6 as u32)) as u32); bb = 28; continue; }
            5 => { crate::simd_ops::wu32(rbx.wrapping_add(0x188u64), ((xmm6 as u32)) as u32); bb = 28; continue; }
            6 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(core::ptr::read_unaligned((rbx.wrapping_add(0x150u64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x150u64), ((xmm6 as u32)) as u32); bb = 35; continue; }
            7 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(core::ptr::read_unaligned((rbx.wrapping_add(0x154u64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x154u64), ((xmm6 as u32)) as u32); bb = 35; continue; }
            8 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(core::ptr::read_unaligned((rbx.wrapping_add(0x158u64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x158u64), ((xmm6 as u32)) as u32); bb = 35; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x1b8u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 10; continue; }
            10 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x4u64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); bb = 35; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x1b8u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 12; continue; }
            12 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0xcu64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); bb = 35; continue; }
            13 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x1b8u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 14; continue; }
            14 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rdi.wrapping_add(0x8u64)) as *const u32))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); bb = 35; continue; }
            15 => { crate::simd_ops::wu32(rbx.wrapping_add(0x15cu64), ((xmm6 as u32)) as u32); bb = 35; continue; }
            16 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm6 as u32)))) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(rbx.wrapping_add(0x160u64))))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = (crate::simd_ops::lk_xor_u8(rbx.wrapping_add(0x160u64), ((rax as u8)) as u8)) as u8 as u64; fl_r = fl_l; bb = 35; continue; }
            17 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (0x8u32) as u32 as u64; xmm0 = 0u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; rcx = (if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x160u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf7u8))) as u8 as u64)); fl_l = ((((rcx as u8)) | ((rax as u8)))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x160u64), ((rcx as u8)) as u8); bb = 35; continue; }
            18 => { crate::simd_ops::wu32(rbx.wrapping_add(0x178u64), ((xmm6 as u32)) as u32); bb = 35; continue; }
            19 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = (rcx & !0xffu64) | (((0x80u8) as u8 as u64)); bb = 22; continue; }
            21 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 22; continue; }
            22 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18cu64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7fu8))) as u8 as u64)); fl_l = ((((rax as u8)) | ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x18cu64), ((rax as u8)) as u8); bb = 28; continue; }
            23 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = (rcx & !0xffu64) | (((0x40u8) as u8 as u64)); bb = 26; continue; }
            25 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 26; continue; }
            26 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18cu64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xbfu8))) as u8 as u64)); fl_l = ((((rax as u8)) | ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x18cu64), ((rax as u8)) as u8); bb = 28; continue; }
            27 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 28; continue; }
            28 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x1b8u64))) as u64; fl_l = (r8) as u64 as u64; fl_r = (r8) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 29; continue; }
            29 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x42u32) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 30; continue; }
            30 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 31; continue; }
            31 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm6 as u32)))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 35; continue; }
            33 => { rdx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm6 as u32)))) as u32 as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 35; continue; }
            34 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (0x80000000u32) as u32 as u64; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; rcx = (if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x30u64), ((rax as u32)) as u32); bb = 35; continue; }
            35 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 36; continue; }
            36 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1415920b0u64) { 0x25 => { bb = 1; continue; } 0x3a => { bb = 2; continue; } 0x57 => { bb = 3; continue; } 0x64 => { bb = 4; continue; } 0x71 => { bb = 5; continue; } 0x7e => { bb = 6; continue; } 0x93 => { bb = 7; continue; } 0xa8 => { bb = 8; continue; } 0xbd => { bb = 9; continue; } 0xcd => { bb = 10; continue; } 0xe4 => { bb = 11; continue; } 0xf4 => { bb = 12; continue; } 0x10b => { bb = 13; continue; } 0x11b => { bb = 14; continue; } 0x132 => { bb = 15; continue; } 0x13f => { bb = 16; continue; } 0x156 => { bb = 17; continue; } 0x17c => { bb = 18; continue; } 0x189 => { bb = 19; continue; } 0x18e => { bb = 20; continue; } 0x192 => { bb = 21; continue; } 0x194 => { bb = 22; continue; } 0x1a7 => { bb = 23; continue; } 0x1ac => { bb = 24; continue; } 0x1b0 => { bb = 25; continue; } 0x1b2 => { bb = 26; continue; } 0x1c5 => { bb = 27; continue; } 0x1cd => { bb = 28; continue; } 0x1d9 => { bb = 29; continue; } 0x1de => { bb = 30; continue; } 0x1e3 => { bb = 31; continue; } 0x1e8 => { bb = 32; continue; } 0x1f2 => { bb = 33; continue; } 0x200 => { bb = 34; continue; } 0x21b => { bb = 35; continue; } 0x220 => { bb = 36; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141592360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141590a50; const P1: u64 = 0x141591390; const P2: u64 = 0x141592d20; const P3: u64 = 0x1415928a0; const P4: u64 = 0x143426564; const P5: u64 = 0x142673708; const P6: u64 = 0x1426736fc; const P7: u64 = 0x142673700; const P8: u64 = 0x1426736ec; const P9: u64 = 0x1426736e8; const P10: u64 = 0x1426736f0; const P11: u64 = 0x143815b38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x1e0u64), ((xmm1 as u32)) as u32); rbp = (((r9 as u8)) as u32) as u32 as u64; rsi = r8; rbx = rcx; rdi = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x178); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rax; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x1b8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P5)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm1 = 0u128; bb = 11; continue; }
            10 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32) as u128; xmm1 = (xmm0) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P8))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P9))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P10))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x40u64)))).to_bits() as u128); bb = 11; continue; }
            11 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x100u64)))).to_bits() as u128); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; xmm0 = ((crate::simd_ops::ru32(P11)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rax, ((rcx as u8)) as u8); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = 0x3; crate::simd_ops::wu32(rax, (0x29u32) as u32); bb = 16; continue; }
            16 => { rax = ((rdi as u32)) as u32 as u64; bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415924d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1f8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64));
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { fl_l = ((((rax as u8)) | (0x4u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415924f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x1415c2af0; const P2: u64 = 0x80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141592510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141592c70; const P1: u64 = 0x178; const P2: u64 = 0x141591390; const P3: u64 = 0x141607970; const P4: u64 = 0x1b8; const P5: u64 = 0x1401de410; const P6: u64 = 0x1415242e0; const P7: u64 = 0x1f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64));
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), (0x0u64) as u64); }
    crate::simd_ops::lk_or_u8(rbx.wrapping_add(P7), (0x20u8) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141592580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415918c0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = rcx; rsi = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rsi as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = ((rsi as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { r8 = rsi; r10 = rdi; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 6; continue; }
            6 => { rax = rsi; r11 = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_sub(rdi)) as u64; fl_l = rax; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 7; continue; }
            7 => { rdx = (((r10) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; r9 = rsi; r9 = ((r9).wrapping_sub(rdi)) as u64; r9 = ((r9).wrapping_sub(0x4u64)) as u64; r9 = ((r9).wrapping_shr(2u32)) as u64; fl_l = ((r9).wrapping_add(1)) as u64; r9 = ((fl_l as u64)) as u64; r10 = ((rdi.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r11); crate::simd_ops::wu32(rcx.wrapping_add(0x38u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x44u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x4cu64), r11); crate::simd_ops::wu64(rcx.wrapping_add(0x54u64), r11); crate::simd_ops::wu32(rcx.wrapping_add(0x5cu64), (0x42c80000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x68u64), ((r11 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), r11); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x80u64), ((r11 as u32)) as u32); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(0x88u64)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r11); crate::simd_ops::wu32(rcx.wrapping_add(0x38u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x44u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x4cu64), r11); crate::simd_ops::wu64(rcx.wrapping_add(0x54u64), r11); crate::simd_ops::wu32(rcx.wrapping_add(0x5cu64), (0x42c80000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x68u64), ((r11 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), r11); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x80u64), ((r11 as u32)) as u32); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(0x110u64)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r11); crate::simd_ops::wu32(rcx.wrapping_add(0x38u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x44u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x4cu64), r11); crate::simd_ops::wu64(rcx.wrapping_add(0x54u64), r11); crate::simd_ops::wu32(rcx.wrapping_add(0x5cu64), (0x42c80000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x68u64), ((r11 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), r11); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x80u64), ((r11 as u32)) as u32); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(0x198u64)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r11); crate::simd_ops::wu32(rcx.wrapping_add(0x38u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x44u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x4cu64), r11); crate::simd_ops::wu64(rcx.wrapping_add(0x54u64), r11); crate::simd_ops::wu32(rcx.wrapping_add(0x5cu64), (0x42c80000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x68u64), ((r11 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), r11); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x80u64), ((r11 as u32)) as u32); bb = 16; continue; }
            16 => { rdx = ((rdx).wrapping_add(0x220u64)) as u64; fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 17; continue; }
            17 => { fl_l = r10; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 18; continue; }
            18 => { rdx = (((r10) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = r8; fl_r = r10; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r11); crate::simd_ops::wu32(rcx.wrapping_add(0x38u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x44u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x4cu64), r11); crate::simd_ops::wu64(rcx.wrapping_add(0x54u64), r11); crate::simd_ops::wu32(rcx.wrapping_add(0x5cu64), (0x42c80000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x68u64), ((r11 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), r11); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x80u64), ((r11 as u32)) as u32); bb = 21; continue; }
            21 => { rdx = ((rdx).wrapping_add(0x88u64)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rsi as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141592830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141591390; const P1: u64 = 0x1f8; const P2: u64 = 0x8; const P3: u64 = 0x1434264e4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128);
    rcx = ((rcx).wrapping_add(0x178u64)) as u64;
    xmm6 = (xmm1) as u128;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = ((crate::simd_ops::ru8(rax.wrapping_add(P1))) as u32) as u32 as u64;
    fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = ((rcx).wrapping_add(0x2u64)) as u64;
    rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64;
    r8 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64;
    rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rcx.wrapping_mul(4))))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); }
    xmm0 = (xmm6) as u128;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415928a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930fa8; const P1: u64 = 0x1415f7e10; const P2: u64 = 0x14159f2a0; const P3: u64 = 0x141607d90; const P4: u64 = 0x1415f8150; const P5: u64 = 0x141605350; const P6: u64 = 0x141606b80; const P7: u64 = 0x1415f7ea0; const P8: u64 = 0x1415f83f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x140u64))) as u64; fl_r = (0x0u64) as u64; rbp = (((r8 as u8)) as u32) as u32 as u64; rbx = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x18cu64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(1)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 21; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1b8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru32(rcx)) as u32 as u64; rdi = 0x1; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x148u64), ((rax as u32)) as u32); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x7fffffff; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1c0u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; r8 = rax; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1b8u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), (0x0u64) as u64); bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rbx.wrapping_add(0x148u64), ((rsi as u32)) as u32); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18du64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; rdx = rax; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 20; continue; }
            20 => { bb = 22; continue; }
            21 => { rdi = 0x1; bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rsi = ((rax as u32)) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), (0x0u64) as u64); bb = 29; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 26; continue; }
            26 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1c0u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; r8 = rax; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141592aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140; const P1: u64 = 0x1c0; const P2: u64 = 0x14159f2a0; const P3: u64 = 0x143930fa8; const P4: u64 = 0x1415f83f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    rbx = rcx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; r8 = rax; rcx = (crate::simd_ops::ru64(P3)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141592ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x14159f2a0; const P2: u64 = 0x143930fa8; const P3: u64 = 0x1415f81d0; const P4: u64 = 0x1415aef20; const P5: u64 = 0x141592510; const P6: u64 = 0x1415af340;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rbp = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1c8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x1c8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(0x1c8u64), rbp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(0x28u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1c0u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            12 => { fl_l = (((rax as u16)).wrapping_sub(1)) as u16 as u64; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rdi.wrapping_add(0x10u64), ((rax as u16)) as u16); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rbp); crate::simd_ops::wu32(rsi, ((rbp as u32)) as u32); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; r8 = rbx; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x148u64), ((rbp as u32)) as u32); bb = 15; continue; }
            15 => { rsi = (0xffffffffffffffffu64) as u64; rdx = rsp.wrapping_add(0x30); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 19; continue; }
            18 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x60u64), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 20; continue; }
            20 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x64u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x4; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1c0u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141592c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1d8; const P1: u64 = 0x1d0; const P2: u64 = 0x10; const P3: u64 = 0x58; const P4: u64 = 0x1415c3470; const P5: u64 = 0x1415ccc10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rbp = (((rdx as u8)) as u32) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x1u8) as u8); rax = ((rbx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; r8 = rsp.wrapping_add(0x38); rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(P0), (0x0u32) as u32); bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141592d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141592c70; const P1: u64 = 0x141591670; const P2: u64 = 0x14158fc60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rbx = rdx; rdi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); rbp = rcx.wrapping_add(0x188); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); r14 = rcx.wrapping_add(0x180); crate::simd_ops::wu32(r14, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); r15 = rcx.wrapping_add(0x184); crate::simd_ops::wu32(r15, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x28u64))) as u32 as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbp, ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; r12 = 0x1; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x1d8u64))) as u32 as u64; rcx = rdi.wrapping_add(0x58); rdx = (((rdx as u32)).wrapping_add(crate::simd_ops::ru32(rdi.wrapping_add(0x60u64)))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); rsi = ((r12.wrapping_add(0x33u64)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x218u64))) as u32 as u64; rcx = rdi.wrapping_add(0x1d0); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rsi = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r12 as u32) } else { (rsi as u32) }) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x210u64))) as u64; rdx = rsp.wrapping_add(0x68); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x218u64))) as u32 as u64; r9 = 0x1a; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rbx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = rdi; r8 = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = 0x1b; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r8 = rbx; rdx = rsp.wrapping_add(0x68); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = 0x1c; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); r8 = rbx; rdx = rsp.wrapping_add(0x68); rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 8; continue; }
            8 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((r12 as u32)) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x17cu64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141592e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1415920b0; const P2: u64 = 0x143930f44; const P3: u64 = 0x1415317a0;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; rbx = rcx; rsi = ((r8 as u32)) as u32 as u64; r8 = rdx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 31; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); rax = ((rsi.wrapping_sub(0x12u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); rdx = P0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x28; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(0x15930ecu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x15930dcu64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = rcx; bb = 0xffff_fffe; continue; }
            3 => { xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32) as u128; r8 = ((rsi as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            4 => { crate::simd_ops::wu32(rbx.wrapping_add(0x1a0u64), (0x0u32) as u32); bb = 5; continue; }
            5 => { xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32) as u128; r8 = ((rsi as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); rdi = (((crate::simd_ops::ru32(rdx.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x26895f0u64))) as i32 as i64) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x4a; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); r15 = (crate::simd_ops::ru32(rdx.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x2689180u64))) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1a; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r14); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 9; continue; }
            9 => { r14 = ((rdi as u32)) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; r14 = (((r14 as u32)).wrapping_shr(5u32)) as u32 as u64; rcx = ((r14 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(5u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x124u64))) as u32 as u64; rdx = (((rax as u8)) as u32) as u32 as u64; fl_l = (((rcx) >> (((rdx) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 23; continue; } bb = 10; continue; }
            10 => { xmm8 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32) as u128; xmm7 = 0u128; fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 24; continue; } bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = rbx.wrapping_add(0xf0); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu8(rsp.wrapping_add(0x21u64), ((rax as u8)) as u8); xmm6 = 0u128; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x5u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x22u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rdi as u8)) as u8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r8 = rsp.wrapping_add(0x80); crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x0u32) as u32); bb = 14; continue; }
            14 => { xmm7 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; xmm6 = (xmm7) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm8 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x130u64))) as u32 as u64; rax = ((r14 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(5u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = (((rdi as u8)) as u32) as u32 as u64; fl_l = (((rcx) >> (((rdx) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x13fu64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 24; continue; }
            17 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(r15.wrapping_mul(4)).wrapping_add(0x88u64))) as u32) as u128; fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x13fu64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 24; continue; }
            20 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 21; continue; }
            21 => { if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rbx.wrapping_add(r15.wrapping_mul(4)).wrapping_add(0x88u64), ((xmm6 as u32)) as u32); bb = 24; continue; }
            23 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(r15.wrapping_mul(4)).wrapping_add(0x88u64)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(r15.wrapping_mul(4)).wrapping_add(0x88u64), ((xmm0 as u32)) as u32); bb = 24; continue; }
            24 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 25; continue; }
            25 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 26; continue; }
            26 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 28; continue; }
            28 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x2f; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1c8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x141592e90u64) { 0x1d => { bb = 1; continue; } 0x3c => { bb = 2; continue; } 0x50 => { bb = 3; continue; } 0x69 => { bb = 4; continue; } 0x73 => { bb = 5; continue; } 0x8c => { bb = 6; continue; } 0xa5 => { bb = 7; continue; } 0xbc => { bb = 8; continue; } 0xd1 => { bb = 9; continue; } 0xf7 => { bb = 10; continue; } 0x10e => { bb = 11; continue; } 0x16d => { bb = 12; continue; } 0x177 => { bb = 13; continue; } 0x17e => { bb = 14; continue; } 0x190 => { bb = 15; continue; } 0x1aa => { bb = 16; continue; } 0x1b3 => { bb = 17; continue; } 0x1c2 => { bb = 18; continue; } 0x1c7 => { bb = 19; continue; } 0x1d0 => { bb = 20; continue; } 0x1d3 => { bb = 21; continue; } 0x1d5 => { bb = 22; continue; } 0x1e1 => { bb = 23; continue; } 0x1fe => { bb = 24; continue; } 0x206 => { bb = 25; continue; } 0x20b => { bb = 26; continue; } 0x213 => { bb = 27; continue; } 0x22c => { bb = 28; continue; } 0x231 => { bb = 29; continue; } 0x23d => { bb = 30; continue; } 0x243 => { bb = 31; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141593120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160a240;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r9 as u8)) as u8);
    r9 = ((rdx as u32)) as u32 as u64;
    rdx = 0x1;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141593150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c8; const P1: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141593180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142676f40; const P1: u64 = 0x142676f50; const P2: u64 = 0x14158f6c0; const P3: u64 = 0x1426892a8; const P4: u64 = 0x142677048; const P5: u64 = 0x142689318; const P6: u64 = 0x1426893a8; const P7: u64 = 0x143814f60; const P8: u64 = 0x143930fa0; const P9: u64 = 0x1415c4f90; const P10: u64 = 0x14160d040; const P11: u64 = 0x1415c0b00; const P12: u64 = 0x141524330;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbp = r9; rax = P0; crate::simd_ops::wu64(rcx, rax); rsi = rdx; rax = P1; rdi = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r14 as u32)) as u32); rcx = ((rcx).wrapping_add(0x18u64)) as u64; r9 = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2f8u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; r9 = (r9 & !0xffu64) | (((((r9 as u8)).wrapping_shr(2u32)) as u8 as u64)); r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & (0x1u8))) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x220u64), r14); rax = P3; crate::simd_ops::wu64(rdi, rax); rax = P4; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rax); rax = P5; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); rax = P6; crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x228u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0x238u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0x240u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0x248u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0x250u64), r14); crate::simd_ops::wu32(rdi.wrapping_add(0x258u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x25cu64), r14); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x270u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x264u64), rax); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x26cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax, (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdi.wrapping_add(0x25cu64), rax); rbx = 0x1; crate::simd_ops::wu64(rdi.wrapping_add(0x278u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0x280u64), r14); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_and_u8(rdi.wrapping_add(0x302u64), (0x80u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x288u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x2a4u64), ((r14 as u8)) as u8); crate::simd_ops::lk_and_u32(rdi.wrapping_add(0x2a4u64), (0xffu32) as u32); rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x306u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x2fcu64), ((rax as u32)) as u32); fl_l = 0; fl_r = ((rax as u32)) as u32 as u64; rax = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x290u64), rbp); rax = (rax & !0xffu64) | (((if (((fl_l as u32)) < ((fl_r as u32))) { 0xffu8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu64(rdi.wrapping_add(0x298u64), r14); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x40u8))) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0x2acu64), (0x3f800000u32) as u32); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x22u8))) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0x2b0u64), (0x3f800000u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x305u64), ((rax as u8)) as u8); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xf8u8))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x306u64), ((rcx as u8)) as u8); rax = (((rcx as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x2b4u64), (0x3f800000u64) as u64); crate::simd_ops::wu16(rdi.wrapping_add(0x300u64), ((rbx as u16)) as u16); crate::simd_ops::wu16(rdi.wrapping_add(0x303u64), ((r14 as u16)) as u16); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(rsi.wrapping_add(0x2f8u64))))) as u8 as u64)); crate::simd_ops::lk_and_u32(rdi.wrapping_add(0x308u64), (0xfffffff8u32) as u32); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x8u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ ((rcx as u8)))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x306u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x2f8u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(1u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(rdi.wrapping_add(0x30cu64))))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & ((rbx as u8)))) as u8 as u64)); crate::simd_ops::lk_xor_u8(rdi.wrapping_add(0x30cu64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x310u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x320u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x324u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x84u64))) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x330u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0x338u64), r14); crate::simd_ops::wu32(rdi.wrapping_add(0x340u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x328u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x98u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x348u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x350u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x358u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0x368u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0x370u64), r14); crate::simd_ops::wu32(rdi.wrapping_add(0x378u64), (0xffffffffu32) as u32); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x90u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(P7, ((rax as u32)) as u32); bb = 5; continue; }
            4 => { fl_l = (crate::simd_ops::lk_or_u8(rdi.wrapping_add(0x306u64), (0x2u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rdi.wrapping_add(0x318u64), ((rcx as u32)) as u32); rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1fcu64))) as u32 as u64; rcx = 0x4; crate::simd_ops::wu32(rdi.wrapping_add(0x314u64), ((rax as u32)) as u32); bb = 6; continue; }
            6 => { rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add(1)) as u8 as u64)); rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((((rcx as u32)) & ((rax as u32)))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu8(rdi.wrapping_add(0x2a4u64), ((rdx as u8)) as u8); r8 = rdi.wrapping_add(0x20); crate::simd_ops::lk_and_u32(rdi.wrapping_add(0x2a4u64), (0xffu32) as u32); crate::simd_ops::lk_or_u32(rdi.wrapping_add(0x2a4u64), (0x4100u32) as u32); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x2b8u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x2c8u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x58u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x2d8u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x68u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x2e8u64), (xmm1) as u128); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x2f8u64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x348u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x288u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { r9 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; bb = 11; continue; }
            10 => { r9 = (crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32 as u64; bb = 11; continue; }
            11 => { r10 = (crate::simd_ops::ru32(rdi.wrapping_add(0x270u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x80u64)) as u32) as u32 as u64; r11 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x349u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xcfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x34cu64))) as u32) as u32 as u64; bb = 15; continue; }
            14 => { rdx = (rdx & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r10 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r9 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((r11 as u8)) as u8); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), ((rdx as u8)) as u8); crate::simd_ops::wu128(r8, (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); bb = 17; continue; }
            17 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x34du64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x34du64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(P8)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((r14 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), (0x81u32) as u32); xmm3 = (((rax as u32)) as u32) as u128; xmm3 = crate::simd_ops::vcvtdq2ps(xmm3); crate::simd_ops::wu8(rsp.wrapping_add(0xa4u64), ((rbx as u8)) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x358u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 21; continue; }
            21 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1d8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x358u64))) as u64; rsi = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { rdx = ((rax.wrapping_add(0x18u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rax); bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rax); bb = 27; continue; }
            26 => { crate::simd_ops::wu64(rcx, rax); bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rcx, r14); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x6cu64), (1) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu32(rax, ((rsi as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdi); bb = 29; continue; }
            29 => { r11 = rsp.wrapping_add(0x70); rax = rdi; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415935e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426892a8; const P1: u64 = 0x360; const P2: u64 = 0x142677048; const P3: u64 = 0x10; const P4: u64 = 0x142689318; const P5: u64 = 0x18; const P6: u64 = 0x1426893a8; const P7: u64 = 0x80; const P8: u64 = 0x8; const P9: u64 = 0x1415242e0; const P10: u64 = 0x14; const P11: u64 = 0x25c; const P12: u64 = 0x1415864b0; const P13: u64 = 0x14158f890; const P14: u64 = 0x142676f40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); rdi = ((rcx.wrapping_add(P1)) as u64) as u64; rax = P2; rbx = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); rax = P4; crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); rax = P6; crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rbp as u32)) as u32); fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P8), rbp); crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rbp as u32)) as u32); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x220u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x228u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x220u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x22cu64), ((rbp as u32)) as u32); bb = 8; continue; }
            8 => { rcx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = P14; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx, rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415936b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x141524330; const P2: u64 = 0x1415317a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx.wrapping_add(0x68); rsi = rdx; rdx = (crate::simd_ops::ru64(rdi)) as u64; rbx = 0; fl_l = 0; fl_r = 0; r13 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbx as u32)) as u32); bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu128(r13, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r13.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r13.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(r13.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(r13.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(r13.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(r13.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(r13.wrapping_add(0x70u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x80u64))) as u128; crate::simd_ops::wu128(r13.wrapping_add(0x80u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x90u64))) as u128; crate::simd_ops::wu128(r13.wrapping_add(0x90u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0xa0u64))) as u128; xmm0 = 0u128; crate::simd_ops::wu128(r13.wrapping_add(0xa0u64), (xmm1) as u128); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(r13.wrapping_add(0xb0u64), rax); crate::simd_ops::wu128(rdi, (xmm0) as u128); r15 = (crate::simd_ops::ru32(rsi.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((r15 as u32)) as u32); fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 32; continue; } bb = 3; continue; }
            3 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 27; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((r15 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r14 = ((rcx as u32)) as u32 as u64; fl_l = rbx; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); r11 = rbx.wrapping_sub(0x4); r12 = 0x8; r11 = ((r11).wrapping_shr(2u32)) as u64; r15 = 0x18; rdx = rax.wrapping_add(0x18); r13 = (0xffffffffffffffe8u64) as u64; r12 = ((r12).wrapping_sub(rax)) as u64; r15 = ((r15).wrapping_sub(rax)) as u64; r13 = ((r13).wrapping_sub(rax)) as u64; fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; r14 = ((r11.wrapping_mul(4)) as u64) as u64; bb = 9; continue; }
            9 => { r9 = rdx.wrapping_sub(0x18); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(r9, ((rcx as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_sub(0x14u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; r10 = ((rdx.wrapping_add(r13)) as u64) as u64; r8 = rdx.wrapping_sub(0x8); rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx))) as u32 as u64; crate::simd_ops::wu32(r9, ((rax as u32)) as u32); r9 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0xcu64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(r8, ((r9 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_sub(0x4u64), r9); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); r8 = rdx.wrapping_add(0x8); rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(rcx).wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(r8, ((r9 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xcu64), (0x0u64) as u64); bb = 15; continue; }
            15 => { rcx = ((r12.wrapping_add(rdx)) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); r8 = rdx.wrapping_add(0x18); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(r8, ((r9 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x1cu64), (0x0u64) as u64); bb = 17; continue; }
            17 => { rcx = ((r15.wrapping_add(rdx)) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(r8, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rcx = 0x0; crate::simd_ops::wu32(rdx.wrapping_add(0x24u64), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(0x40u64)) as u64; fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r15 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 19; continue; }
            19 => { fl_l = r14; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rdx = r14; r10 = (0xfffffffffffffff8u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_sub(rbp)) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; fl_l = rbx; fl_r = r14; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 21; continue; }
            21 => { r9 = rdx.wrapping_sub(0x8); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(r9, ((rcx as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_sub(0x4u64), (0x0u64) as u64); bb = 23; continue; }
            23 => { rcx = ((r10.wrapping_add(rdx)) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(r9, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_sub(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rcx = 0x0; crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r15 as u32)) as u32); bb = 26; continue; }
            26 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x68u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x70u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 28; continue; }
            28 => { bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; r8 = rsp.wrapping_add(0x70); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rcx as u32)) as u32); rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rcx as u32)) as u32); bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x70u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x68u64)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 32; continue; }
            32 => { rax = r13; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141593a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426892a8; const P1: u64 = 0x360; const P2: u64 = 0x142677048; const P3: u64 = 0x10; const P4: u64 = 0x142689318; const P5: u64 = 0x18; const P6: u64 = 0x1426893a8; const P7: u64 = 0x80; const P8: u64 = 0x8; const P9: u64 = 0x1415242e0; const P10: u64 = 0x14; const P11: u64 = 0x25c; const P12: u64 = 0x1415864b0; const P13: u64 = 0x14158f890; const P14: u64 = 0x142676f40; const P15: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbp = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rcx, rax); rdi = ((rcx.wrapping_add(P1)) as u64) as u64; rax = P2; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); rbx = rcx; rax = P4; crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); rax = P6; crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rdi.wrapping_add(P3), ((r14 as u32)) as u32); fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P8), r14); crate::simd_ops::wu32(rdi.wrapping_add(P10), ((r14 as u32)) as u32); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x220u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x228u64), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x220u64), r14); crate::simd_ops::wu32(rbx.wrapping_add(0x22cu64), ((r14 as u32)) as u32); bb = 8; continue; }
            8 => { rcx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rax = P14; crate::simd_ops::wu64(rbx, rax); fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = 0x380; rcx = rbx; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141593b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x228; const P1: u64 = 0x220; const P2: u64 = 0x1434268d0; const P3: u64 = 0x10; const P4: u64 = 0x298; const P5: u64 = 0x29c; const P6: u64 = 0x1f8; const P7: u64 = 0x100;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(P3)))).to_bits() as u128); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P4)))).to_bits() as u128); xmm1 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P5)))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm0 = 0u128; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(P6), (0xefu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu32(rcx.wrapping_add(P7), ((xmm0 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141593b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x30; const P3: u64 = 0x40; const P4: u64 = 0x50; const P5: u64 = 0x60; const P6: u64 = 0x70; const P7: u64 = 0xa8; const P8: u64 = 0xb0; const P9: u64 = 0xb6; const P10: u64 = 0x7c; const P11: u64 = 0x84;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = rcx;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = 0u128;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu128(rcx.wrapping_add(P0), (xmm0) as u128);
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128);
    crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128);
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((r8 as u32)) as u32);
    xmm1 = 0u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P7), r8);
    crate::simd_ops::wu32(rcx.wrapping_add(P8), ((r8 as u32)) as u32);
    rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P9))) as u32) as u32 as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P10), r8);
    rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x31u8))) as u8 as u64));
    crate::simd_ops::wu64(rdx.wrapping_add(P11), r8);
    rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xcfu8))) as u8 as u64));
    crate::simd_ops::lk_and_u8(rdx.wrapping_add(0xb7u64), (0xf8u8) as u8);
    fl_l = ((((rcx as u8)) & (0xfeu8))) as u8 as u64; fl_r = fl_l;
    rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    crate::simd_ops::wu8(rdx.wrapping_add(0xb6u64), ((rcx as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(0x8cu64), ((r8 as u32)) as u32);
    crate::simd_ops::wu16(rdx.wrapping_add(0xb4u64), ((r8 as u16)) as u16);
    crate::simd_ops::wu32(rdx.wrapping_add(0x78u64), (0x3f800000u32) as u32);
    crate::simd_ops::wu128(rdx.wrapping_add(0xc8u64), (xmm1) as u128);
    crate::simd_ops::wu32(rdx.wrapping_add(0xc0u64), ((r8 as u32)) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0xd8u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(0x230u64), r8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x238u64), r8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141593c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x368; const P1: u64 = 0x370; const P2: u64 = 0x1415a1690; const P3: u64 = 0x306; const P4: u64 = 0x2a;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = 0xffff; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::lk_add_u16(rax.wrapping_add(P4), ((rsi as u16)) as u16)) as u16 as u64; fl_r = fl_l; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P1), (0x0u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141593cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x31c; const P1: u64 = 0x141591390; const P2: u64 = 0x1f8; const P3: u64 = 0x2; const P4: u64 = 0x320; const P5: u64 = 0x8; const P6: u64 = 0x143426ea4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rbx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); xmm6 = (xmm1) as u128; xmm7 = ((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32) as u128; rcx = ((rcx).wrapping_add(0x190u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru8(rax.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 2; continue; }
            2 => { rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32) as u128; xmm0 = 0u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm6 = 0u128; bb = 10; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x2u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(rcx.wrapping_mul(4)))) as u32) as u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 10; continue; }
            9 => { xmm6 = (xmm1) as u128; bb = 10; continue; }
            10 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm7 = (xmm6) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((if f32::from_bits(xmm7 as u32) > f32::from_bits(xmm0 as u32) { f32::from_bits(xmm7 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((if f32::from_bits(xmm7 as u32) < f32::from_bits(crate::simd_ops::ru32(P6)) { f32::from_bits(xmm7 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P6)) }).to_bits() as u128); bb = 11; continue; }
            11 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm7 as u32)))) as u32 as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141593d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143930f80; const P2: u64 = 0x1415ad8b0; const P3: u64 = 0x1415ada30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rbx = r9; rbp = r8; r15 = (((rdx as u8)) as u32) as u32 as u64; rdi = rcx; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x278u64))) as u64; bb = 3; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x280u64))) as u64; bb = 3; continue; }
            3 => { xmm3 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rax = ((r8.wrapping_sub(0x4000000u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfbffffff; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0x1c; bb = 6; continue; }
            5 => { rcx = 0x1d; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x4000000; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x2000000; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm3 = 0u128; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rsi); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x2b0u64))) as u32) as u128; bb = 12; continue; }
            11 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x2b4u64))) as u32) as u128; bb = 12; continue; }
            12 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(P1)) as u64; rax = rdi.wrapping_add(0x10); fl_l = rdi; fl_r = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm3 as u32)) as u32); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsi } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); r8 = 0; fl_l = 0; fl_r = 0; rax = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rbx as u32)) as u32); rax = ((rax).wrapping_shr(32u32)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((rax as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x64u64), (0x100u16) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x66u64), ((rsi as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x306u64))) as u32) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1000000; crate::simd_ops::wu64(rdi.wrapping_add(0x278u64), rax); rax = 0x8; rsi = (if (((fl_l as u32)) == ((fl_r as u32))) { (rax as u32) } else { (rsi as u32) }) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xf7u8))) as u8 as u64)); fl_l = ((((rsi as u8)) | ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rsi = (rsi & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x306u64), ((rsi as u8)) as u8); bb = 15; continue; }
            14 => { crate::simd_ops::wu64(rdi.wrapping_add(0x280u64), r14); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = ((rbx as u32)) as u32 as u64; rdx = 0xe; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r14; fl_r = r14; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = rdi.wrapping_add(0x10); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32) as u128; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 18; continue; }
            18 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x78u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xbcu64))) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141593f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c0bf0; const P1: u64 = 0x1415975d0; const P2: u64 = 0x1415975e0; const P3: u64 = 0x1415a1690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x306u64))) as u32) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; rdi = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfbu8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x306u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x368u64))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x370u64))) as u32 as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = 0xffff; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::lk_add_u16(rcx.wrapping_add(0x2au64), ((r8 as u16)) as u16); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x370u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x368u64))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x303u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1d8u64))) as u64; rdx = rsp.wrapping_add(0x70); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x303u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1d8u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x288u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x240u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = ((rbp as u32)) as u32 as u64; rdx = (rdx & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; rcx = (((rdx as u8)) as u32) as u32 as u64; crate::simd_ops::lk_and_u8(rdi.wrapping_add(0x303u64), (0x80u8) as u8); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(3u32)) as u8 as u64)); rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); fl_l = (crate::simd_ops::lk_or_u8(rdi.wrapping_add(0x303u64), ((rcx as u8)) as u8)) as u8 as u64; fl_r = fl_l; bb = 9; continue; }
            8 => { rdx = ((rax as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x7u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((r8 as u32)) as u32); fl_l = ((((rdx as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 9; continue; }
            9 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x305u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x10u8))) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x368u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x305u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x370u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x370u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x368u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rdx = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1d8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x370u64), ((rbp as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x1u32) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x48u64), (0x100u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x110u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415940e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x298; const P1: u64 = 0x1415976c0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64);
    rdx = 0x4;
    xmm3 = 0u128;
    return crate::simd_ops::tail_fwd_xmm(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c0bf0; const P1: u64 = 0x1415975d0; const P2: u64 = 0x1415975e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x306u64))) as u32) as u32 as u64; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfbu8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x306u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x368u64))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x370u64))) as u32 as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = 0xffff; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::lk_add_u16(rcx.wrapping_add(0x2au64), ((r8 as u16)) as u16); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x368u64))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x303u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1d8u64))) as u64; rdx = rsp.wrapping_add(0x30); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x303u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1d8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x288u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x240u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = (rdx & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rcx = (((rdx as u8)) as u32) as u32 as u64; crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x303u64), (0x80u8) as u8); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(3u32)) as u8 as u64)); rax = (((r8 as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x303u64), ((rcx as u8)) as u8)) as u8 as u64; fl_r = fl_l; bb = 9; continue; }
            8 => { rdx = ((rax as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; r8 = ((((r8 as u32)) & (0x7u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r8 as u32)) as u32); fl_l = ((((rdx as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 9; continue; }
            9 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594210(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c61c0; const P1: u64 = 0x1415c6480;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x298u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { rsi = 0; fl_l = 0; fl_r = 0; rax = ((rsi as u32)) as u32 as u64; fl_l = ((rcx).wrapping_add(0xe8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x290u64))) as u64; rdx = rsp.wrapping_add(0x20); xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x340u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x338u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xc0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xc8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x298u64))) as u64; rax = rdi.wrapping_add(0x18); fl_l = rdi; fl_r = rdi; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsi } else { rax }) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x338u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xd0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rdi.wrapping_add(0x338u64)))) as u64; fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (rax) as u64)) as u64; fl_r = fl_l; bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x290u64))) as u64; rdx = rdi.wrapping_add(0x330); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x330u64), (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0x340u64), ((xmm1 as u64)) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x290u64))) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415943a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c0bf0; const P1: u64 = 0x1415ad830; const P2: u64 = 0x143930f80; const P3: u64 = 0x1415ada30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x303u64))) as u32) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; rdi = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); rdx = rsp.wrapping_add(0x50); crate::simd_ops::wu8(rcx.wrapping_add(0x303u64), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1d8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1d8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x288u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x240u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = ((rsi as u32)) as u32 as u64; rdx = (rdx & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u32) as u32 as u64; rcx = (((rdx as u8)) as u32) as u32 as u64; crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x303u64), (0x80u8) as u8); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(3u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x303u64), ((rcx as u8)) as u8)) as u8 as u64; fl_r = fl_l; bb = 5; continue; }
            4 => { rdx = ((rax as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((((rdx as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 5; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x211u64), ((rdx as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x305u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdi = (rdi & !0xffu64) | (((((rdi as u8)).wrapping_add((rdi as u8))) as u8 as u64)); rdi = (rdi & !0xffu64) | ((((((rdi as u8)) ^ ((rax as u8)))) as u8 as u64)); rdi = (rdi & !0xffu64) | ((((((rdi as u8)) & (0xeu8))) as u8 as u64)); rdi = (rdi & !0xffu64) | ((((((rdi as u8)) ^ ((rax as u8)))) as u8 as u64)); fl_l = ((((rdi as u8)) | (0x1u8))) as u8 as u64; fl_r = fl_l; rdi = (rdi & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x305u64), ((rdi as u8)) as u8); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x304u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x280u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x304u64), (0x20u8) as u8); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x302u64))) as u8 as u64; fl_r = (0x78u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 20; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x278u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(P2)) as u64; xmm3 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); r8 = 0x2000000; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x305u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x2b0u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x118u64), ((rsi as u32)) as u32); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x1u8))) as u8 as u64)); fl_l = ((((rax as u8)) & (0xf1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x305u64), ((rax as u8)) as u8); bb = 19; continue; }
            19 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x378;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0xffffffffu32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x210; const P1: u64 = 0x1d8; const P2: u64 = 0x20; const P3: u64 = 0x1b0; const P4: u64 = 0x14; const P5: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x40u8) as u8 as u64;
    r9 = rcx;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rcx = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(r8, (0x0u64) as u64);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rax = 0xffffffff;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415945a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1d8; const P1: u64 = 0x1b8;
    let (mut rax, mut r10) = (crate::simd_ops::get_rax_in(), 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r10 = (crate::simd_ops::ru64(rcx)) as u64;
    return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415945c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415945e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x330; const P1: u64 = 0x8; const P2: u64 = 0x338; const P3: u64 = 0x10; const P4: u64 = 0x340; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x310; const P8: u64 = 0x28; const P9: u64 = 0x258; const P10: u64 = 0x24; const P11: u64 = 0xffffffffffffffe8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P9))) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P10), ((rax as u32)) as u32);
    rax = ((rcx.wrapping_add(P11)) as u64) as u64;
    crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), rax);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x2f4u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (((r8 as u8)) as u32) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), ((rax as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), (0x0u32) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x303; const P1: u64 = 0x1d8; const P2: u64 = 0x1415c0bf0; const P3: u64 = 0x288; const P4: u64 = 0x240;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; rdi = rdx; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = (rdx & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64; rcx = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x80u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(3u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); rax = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P0), ((rcx as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x18; const P5: u64 = 0xffffffffffffffe8; const P6: u64 = 0xfffffffffffffff8; const P7: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 11; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx.wrapping_add(P4)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P5))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdi).wrapping_add(P5), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(P6))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdi).wrapping_add(P6), ((xmm1 as u64)) as u64); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415947d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141591a50; const P1: u64 = 0x141591df0; const P2: u64 = 0x141535e10; const P3: u64 = 0x143931600; const P4: u64 = 0x14159c7c0; const P5: u64 = 0x143930f90; const P6: u64 = 0x14159b5e0; const P7: u64 = 0x1415c61c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 1; continue; }
            1 => { rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rbx = rdi.wrapping_add(0x348); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = P3; crate::simd_ops::wu64(rdx, rax); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x350u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x358u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi.wrapping_sub(0x18); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x358u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x350u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x258u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = rdi.wrapping_sub(0x10); rcx = (crate::simd_ops::ru64(P5)) as u64; r8 = rdi.wrapping_sub(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x278u64))) as u64; rdx = rdi.wrapping_add(0x318); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rax = 0x2; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 16; continue; }
            16 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415948f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x305; const P1: u64 = 0x304; const P2: u64 = 0x280; const P3: u64 = 0x1415ad830; const P4: u64 = 0x302; const P5: u64 = 0x278; const P6: u64 = 0x143930f80; const P7: u64 = 0x1415ada30; const P8: u64 = 0x2b0; const P9: u64 = 0x118;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rdx as u8))) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) ^ ((rax as u8)))) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0xeu8))) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) ^ ((rax as u8)))) as u8 as u64)); fl_l = ((((rdx as u8)) | (0x1u8))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rdx as u8)) as u8); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::lk_or_u8(rbx.wrapping_add(P1), (0x20u8) as u8); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u8 as u64; fl_r = (0x78u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P6)) as u64; xmm3 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); r8 = 0x2000000; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x1u8))) as u8 as u64)); fl_l = ((((rax as u8)) & (0xf1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P0), ((rax as u8)) as u8); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415949d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415962e0; const P1: u64 = 0x1434268d0; const P2: u64 = 0x14183acc8; const P3: u64 = 0x141595c20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((xmm1 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); xmm6 = (xmm1) as u128; rsi = (((r9 as u8)) as u32) as u32 as u64; rdi = r8; rbx = rcx; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r11 = (crate::simd_ops::ru64(r8)) as u64; rax = ((((rax as u32)) & (0xfffffffcu32))) as u32 as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r10 = ((r8 as u32)) as u32 as u64; r10 = (!((r10 as u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); r10 = ((((r10 as u32)) & (0x1u32))) as u32 as u64; r10 = ((((r10 as u32)) | ((rax as u32)))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r10 as u32)) as u32); rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r11; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((r10 as u32)) as u32 as u64; rdx = ((((rdx as u32)) ^ (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = rax.wrapping_add(0x10); fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r9 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 23; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rdx = rsp.wrapping_add(0x20); rcx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P1)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 21; continue; } bb = 10; continue; }
            10 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 12; continue; }
            12 => { r10 = (crate::simd_ops::ru64(rbx)) as u64; r11 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = r10; xmm0 = (crate::simd_ops::ru128(rdi)) as u128; rax = ((r11.wrapping_add(r11.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = ((r10.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r10; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = ((r9 as u32)) as u32 as u64; rdx = ((((rdx as u32)) ^ (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 17; continue; }
            17 => { fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 18; continue; }
            18 => { rax = ((r11.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((r10.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rcx.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 22; continue; }
            21 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; r8 = rsp.wrapping_add(0x58); rcx = rbx; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x1415949d0; const P2: u64 = 0x141591570; const P3: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rcx = ((rcx).wrapping_add(0x238u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x302;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x9fu8))) as u8 as u64));
    fl_l = ((((rax as u8)) | (0x18u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x298; const P1: u64 = 0x141601720; const P2: u64 = 0x154;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = ((r8 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = ((rdx as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffef; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = (((rbx as u32)).wrapping_shr(4u32)) as u32 as u64; fl_l = ((((rbx as u16)) & (0x1u16))) as u16 as u64; fl_r = fl_l; rbx = (rbx & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rsi.wrapping_add(P2), ((rbx as u16)) as u16); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x338; const P1: u64 = 0x141594210; const P2: u64 = 0x298; const P3: u64 = 0x141601180;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (((r8 as u8)) as u32) as u32 as u64; rsi = rdx; rbx = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = (((rdi as u8)) as u32) as u32 as u64; rdx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x330; const P1: u64 = 0x141594210; const P2: u64 = 0x298; const P3: u64 = 0x141601240;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c0; const P1: u64 = 0x20; const P2: u64 = 0x1416074e0; const P3: u64 = 0x258; const P4: u64 = 0x1416074d0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rdx;
    rbx = rcx;
    rcx = rdi;
    rdx = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x302;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x8fu8))) as u8 as u64));
    fl_l = ((((rax as u8)) | (0x8u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x302; const P1: u64 = 0x3; const P2: u64 = 0x1415976c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x9fu8))) as u8 as u64));
    xmm3 = (xmm1) as u128;
    fl_l = ((((rax as u8)) | (0x18u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    rdx = ((r8.wrapping_add(P1)) as u32) as u32 as u64;
    return crate::simd_ops::tail_fwd_xmm(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x300; const P1: u64 = 0x3; const P2: u64 = 0x14160c240;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = 0x1;
    rdx = rcx;
    crate::simd_ops::wu16(rcx.wrapping_add(P0), ((rax as u16)) as u16);
    rcx = ((rax.wrapping_add(P1)) as u32) as u32 as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594db0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159ed70; const P1: u64 = 0x141593b70; const P2: u64 = 0x1415ca360;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; rsi = rdx; rbx = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x2e0u64))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x98u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdi); crate::simd_ops::wu8(rax.wrapping_sub(0x60u64), (0xffu8) as u8); crate::simd_ops::wu8(rax.wrapping_sub(0x58u64), (0xffu8) as u8); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r14); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), r15); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_sub(0x68u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_sub(0x50u64), r15); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x34u64))) as u32 as u64; rcx = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x2e8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbx.wrapping_add(0xb0); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            5 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x167u64), (0xfdu8) as u8); rcx = rbx.wrapping_add(0x190); crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x166u64), (0x21u8) as u8); rdx = rbx.wrapping_add(0x178); crate::simd_ops::wu32(rbx.wrapping_add(0x140u64), (0x5du32) as u32); r8 = rbx.wrapping_add(0x168); crate::simd_ops::wu64(rbx.wrapping_add(0x144u64), (0x2u64) as u64); r9 = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x2e0u64))) as u64; r10 = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rdx = rbx.wrapping_add(0xb0); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x140u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x2e0u64), rsi); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x2e8u64))) as u64; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r15); rdx = rbx.wrapping_add(0xb0); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((r15 as u32)) as u32); rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x90u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r15); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x2e8u64), rdi); bb = 9; continue; }
            9 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0xa8u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141594f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415924f0; const P1: u64 = 0x141591570; const P2: u64 = 0x143930f80; const P3: u64 = 0x1434268d0; const P4: u64 = 0x1415ada30; const P5: u64 = 0x1415ad8b0; const P6: u64 = 0x1415970c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = rdx; rsi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x270u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 35; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), r15); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 9; continue; }
            9 => { rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            11 => { rbx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x1eu64))) as u32) as u32 as u64; rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x324u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x324u64), ((rax as u32)) as u32); bb = 30; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r14 = 0; fl_l = 0; fl_r = 0; rcx = rsi; crate::simd_ops::wu32(rsi.wrapping_add(0x324u64), ((r14 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x280u64))) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x14u64))) as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x2b4u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (0x8000000u64) as u64); bb = 24; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2cu64))) as u32 as u64; r8 = 0x8000000; xmm3 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            18 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { xmm3 = ((crate::simd_ops::ru32(P3)) as u32) as u128; r8 = 0x8000000; rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r14 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            20 => { rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rcx = rsi.wrapping_add(0x18); crate::simd_ops::wu32(rsi.wrapping_add(0x2b4u64), (0x3f800000u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            21 => { rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x324u64), (1) as u32); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x280u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x14u64))) as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x2b4u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((xmm0 as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), (0x4000000u64) as u64); bb = 24; continue; }
            24 => { fl_l = rsi; fl_r = rsi; crate::simd_ops::wu32(rbp.wrapping_sub(0x1cu64), ((xmm0 as u32)) as u32); rax = rsi.wrapping_add(0x10); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), ((rbx as u32)) as u32); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rax }) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), (0x1du32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); rdx = rbp.wrapping_sub(0x30); rax = rbx; crate::simd_ops::wu16(rbp.wrapping_sub(0xcu64), (0x100u16) as u16); rax = ((rax).wrapping_shr(32u32)) as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x14u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0xau64), ((r14 as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsi)) as u64; r8 = ((rbx as u32)) as u32 as u64; rdx = 0xe; crate::simd_ops::wu64(rsi.wrapping_add(0x280u64), rax); rcx = rsi; rdi = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = rsi.wrapping_add(0x10); xmm2 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x1cu64))) as u32) as u128; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            26 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2cu64))) as u32 as u64; xmm3 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); r8 = 0x4000000; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x302u64))) as u8 as u64; fl_r = (0x78u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            29 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; bb = 31; continue; }
            31 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            32 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x280u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(P2)) as u64; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); xmm3 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); r8 = 0x4000000; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r14 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            34 => { rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 29; continue; }
            35 => { rdx = ((rdx).wrapping_add(0x14u64)) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rsi; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141595250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x304; const P1: u64 = 0x50; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = (xmm3) as u128; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x3; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x8u8))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::lk_or_u8(rcx.wrapping_add(P0), (0x1u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; xmm1 = (xmm6) as u128; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415952b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143930f44; const P1: u64 = 0x142689180;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = ((crate::simd_ops::ru8(rcx.wrapping_add(0x13fu64))) as u32) as u32 as u64; r8 = rcx; fl_l = ((((rdx as u8)) & (0x4u8))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xb8u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xc4u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xccu64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xd4u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xdcu64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0xe4u64), rax); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xf0u64))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0xf8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { r9 = P1; bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; rax = ((rax).wrapping_add(0x10u64)) as u64; rdx = (crate::simd_ops::ru32(r9.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x88u64))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x88u64))) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x88u64), ((xmm0 as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(0xf8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(0xf0u64)))) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_and_u8(r8.wrapping_add(0x13fu64), (0xfbu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 8; continue; }
            7 => { crate::simd_ops::lk_and_u8(r8.wrapping_add(0x13fu64), (0xfbu8) as u8); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(0x23cu64))) as u32) as u128; xmm2 = 0u128; xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x90u64)))).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(0x90u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x124u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x130u64))) as u32 as u64; fl_l = (((rax) >> (3u32)) & 1) as u64; rax = ((rax) | ((1u64) << (3u32))) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x130u64), ((rax as u32)) as u32); bb = 12; continue; }
            12 => { xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(0x240u64))) as u32) as u128; xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x94u64)))).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(0x94u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x124u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x130u64))) as u32 as u64; fl_l = (((rax) >> (4u32)) & 1) as u64; rax = ((rax) | ((1u64) << (4u32))) as u64; crate::simd_ops::wu32(r8.wrapping_add(0x130u64), ((rax as u32)) as u32); bb = 16; continue; }
            16 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141595410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141590fb0; const P1: u64 = 0x1415936b0; const P2: u64 = 0x141594710; const P3: u64 = 0x1415ca360; const P4: u64 = 0x1415ca5a0; const P5: u64 = 0x1415a3d00; const P6: u64 = 0x141601360; const P7: u64 = 0x143930f44; const P8: u64 = 0x142689180; const P9: u64 = 0x1434268d0; const P10: u64 = 0x14159f4e0; const P11: u64 = 0x1415964b0; const P12: u64 = 0x1415c2440; const P13: u64 = 0x143930f70; const P14: u64 = 0x1415d99c0; const P15: u64 = 0x14159f2d0; const P16: u64 = 0x1415242e0; const P17: u64 = 0x141592a80;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 73>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x108); rsp = ((rsp).wrapping_sub(0x208u64)) as u64; r15 = 0; fl_l = 0; fl_r = 0; rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x160u64), r15); crate::simd_ops::wu32(rbp.wrapping_add(0x158u64), ((rbx as u32)) as u32); rsi = rdx; r14 = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = rax; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x238u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x230u64))) as u64; fl_l = rax; fl_r = rax; rbx = (((rbx as u8)) as u32) as u32 as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { (r15 as u32) } else { (rbx as u32) }) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x160u64), rdx); crate::simd_ops::wu32(rbp.wrapping_add(0x158u64), ((rbx as u32)) as u32); rdx = rsi; rbx = r14.wrapping_add(0x88); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0xc8u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x230u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xd8u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x240u64), ((rax as u32)) as u32); rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0xc0u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x228u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x22cu64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rdi as u32)) as u32 as u64; rcx = r14.wrapping_add(0x220); rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = ((rdi as u32)) as u32 as u64; r8 = rdi; crate::simd_ops::wu32(r14.wrapping_add(0x228u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; rdx = ((rax as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xb8u64))) as u64; rdx = rdx.wrapping_add(0x18); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x220u64))) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax).wrapping_sub(0x18u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(rcx).wrapping_sub(0x18u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(rax).wrapping_sub(0x8u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(rcx).wrapping_sub(0x8u64), ((xmm1 as u64)) as u64); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x160u64))) as u64; bb = 20; continue; }
            10 => { rbx = r14.wrapping_add(0x88); xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rbx.wrapping_add(0x70u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0xb0u64), ((r15 as u32)) as u32); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0xb6u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x7cu64), r15); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x31u8))) as u8 as u64)); crate::simd_ops::wu64(rbx.wrapping_add(0x84u64), r15); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xcfu8))) as u8 as u64)); crate::simd_ops::lk_and_u8(rbx.wrapping_add(0xb7u64), (0xf8u8) as u8); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfeu8))) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0xb6u64), ((rcx as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x8cu64), ((r15 as u32)) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0xb4u64), ((r15 as u16)) as u16); crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), (0x3f800000u32) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x220u64))) as u64; r9 = (crate::simd_ops::ru32(r14.wrapping_add(0x228u64))) as u32 as u64; rcx = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 11; continue; }
            11 => { r8 = rax; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rcx = ((r9 as u32)) as u32 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = rcx.wrapping_sub(0x3); rcx = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x220u64))) as u64; bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::lk_sub_u32(r14.wrapping_add(0x228u64), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x228u64))) as u32 as u64; bb = 17; continue; }
            16 => { fl_l = ((rax).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r9 = ((rcx as u32)) as u32 as u64; rcx = ((r8.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 18; continue; }
            18 => { fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { r8 = rsp.wrapping_add(0x78); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r15); rdx = rbx; crate::simd_ops::wu32(rbp.wrapping_sub(0x80u64), ((r15 as u32)) as u32); rcx = r13; crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x70u64), (0xffu8) as u8); crate::simd_ops::wu8(rbp.wrapping_sub(0x68u64), (0xffu8) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x1f8u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 74; continue; } bb = 21; continue; }
            21 => { fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { xmm0 = ((crate::simd_ops::ru32(r14.wrapping_add(0xb4u64))) as u32) as u128; r8 = r14.wrapping_add(0x150); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r14.wrapping_add(0x98u64)))).to_bits() as u128); rdx = r14.wrapping_add(0x8); crate::simd_ops::wu32(r14.wrapping_add(0xb4u64), ((xmm0 as u32)) as u32); r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x1b8u64))) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x1c0u64))) as u64; crate::simd_ops::wu32(r14.wrapping_add(0x98u64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x280u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { rax = 0x2; crate::simd_ops::wu32(r14.wrapping_add(0x118u64), (0x5du32) as u32); crate::simd_ops::wu64(r14.wrapping_add(0x11cu64), rax); rdx = ((crate::simd_ops::ru8(r14.wrapping_add(0x1f8u64))) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x164u64), (0x42ca0000u32) as u32); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = (((rax) >> (1u32)) & 1) as u64; rax = ((rax) & !((1u64) << (1u32))) as u64; crate::simd_ops::wu32(r14.wrapping_add(0x11cu64), ((rax as u32)) as u32); bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x1c0u64))) as u64; rax = rsi.wrapping_add(0xe0); fl_l = rsi; fl_r = rsi; crate::simd_ops::wu64(rbp.wrapping_add(0xf0u64), r15); crate::simd_ops::wu64(rbp.wrapping_add(0xf8u64), (0x0u64) as u64); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rax }) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; r15 = rbp.wrapping_sub(0x50); r15 = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { r15 }) as u64; fl_l = rdi; fl_r = rcx; rdi = r14.wrapping_add(0x230); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::lk_and_u8(r14.wrapping_add(0x13fu64), (0xfdu8) as u8); r8 = r14.wrapping_add(0x220); crate::simd_ops::lk_and_u8(r14.wrapping_add(0x13eu64), (0x21u8) as u8); r9 = r14.wrapping_add(0x8); r10 = (crate::simd_ops::ru64(rcx)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x160u64))) as u64; rax = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); rsi = 0x0; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x158u64))) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rsi } else { rax }) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rdx as u8)) as u8); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x140u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xb7u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x3cu64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x44u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x4cu64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x54u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x5cu64), (0x0u64) as u64); bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x70u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 33; continue; }
            33 => { r8 = P8; bb = 34; continue; }
            34 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; rax = ((rax).wrapping_add(0x10u64)) as u64; rdx = (crate::simd_ops::ru32(r8.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(rdx.wrapping_mul(4)))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(rdx.wrapping_mul(4)))) }).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(rdx.wrapping_mul(4)), ((xmm0 as u32)) as u32); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x70u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x68u64)))) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 35; continue; }
            35 => { rdi = r14.wrapping_add(0x230); bb = 36; continue; }
            36 => { crate::simd_ops::lk_and_u8(rbx.wrapping_add(0xb7u64), (0xfbu8) as u8); xmm2 = 0u128; xmm0 = ((crate::simd_ops::ru32(rdi)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx))).to_bits() as u128); crate::simd_ops::wu32(rbx, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(r14.wrapping_add(0x238u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(r14.wrapping_add(0x8cu64)))).to_bits() as u128); crate::simd_ops::wu32(r14.wrapping_add(0x8cu64), ((xmm1 as u32)) as u32); xmm3 = ((crate::simd_ops::ru32(r14.wrapping_add(0x23cu64))) as u32) as u128; xmm0 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x8u64)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x9cu64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 38; continue; }
            38 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 40; continue; } bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xa8u64))) as u32 as u64; fl_l = (((rax) >> (3u32)) & 1) as u64; rax = ((rax) | ((1u64) << (3u32))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xa8u64), ((rax as u32)) as u32); bb = 40; continue; }
            40 => { xmm1 = ((crate::simd_ops::ru32(r14.wrapping_add(0x240u64))) as u32) as u128; xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::ru32(P7)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 44; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x9cu64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 44; continue; } bb = 42; continue; }
            42 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 44; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xa8u64))) as u32 as u64; fl_l = (((rax) >> (4u32)) & 1) as u64; rax = ((rax) | ((1u64) << (4u32))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xa8u64), ((rax as u32)) as u32); bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x228u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x220u64))) as u64; xmm0 = ((crate::simd_ops::ru32(P9)) as u32) as u128; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 46; continue; } bb = 45; continue; }
            45 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(0x10u64)))).to_bits() as u128); rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 45; continue; } bb = 46; continue; }
            46 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r14.wrapping_add(0x298u64)))).to_bits() as u128); rdx = rbp.wrapping_add(0x168); crate::simd_ops::lk_and_u8(r14.wrapping_add(0x1f8u64), (0xefu8) as u8); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r14.wrapping_add(0x29cu64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm2 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); crate::simd_ops::wu32(r14.wrapping_add(0x100u64), ((xmm0 as u32)) as u32); r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x1c0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x168u64))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x304u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 49; continue; } bb = 47; continue; }
            47 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x16cu64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x308u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 49; continue; } bb = 48; continue; }
            48 => { rdi = r14.wrapping_sub(0x18); bb = 50; continue; }
            49 => { xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x168u64))) as u64) as u128; rdi = r14.wrapping_sub(0x18); rcx = rdi; crate::simd_ops::wu64(r14.wrapping_add(0x304u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 50; continue; }
            50 => { rbx = ((crate::simd_ops::ru8(r14.wrapping_add(0x1f8u64))) as u32) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 59; continue; } bb = 51; continue; }
            51 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; rbx = (rbx & !0xffu64) | (((((rbx as u8)).wrapping_shr(1u32)) as u8 as u64)); rbx = (rbx & !0xffu64) | (((!((rbx as u8))) as u8 as u64)); rbx = (rbx & !0xffu64) | ((((((rbx as u8)) & (0x1u8))) as u8 as u64)); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 52; continue; }
            52 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 53; continue; }
            53 => { fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x148u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 58; continue; } bb = 54; continue; }
            54 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x2f4u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x338u64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(r14.wrapping_add(0x330u64))) as u64) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x340u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x310u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x258u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 56; continue; } bb = 55; continue; }
            55 => { rax = (((rbx as u8)) as u32) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 57; continue; }
            56 => { rax = 0; fl_l = 0; fl_r = 0; bb = 57; continue; }
            57 => { rcx = (crate::simd_ops::ru64(P13)) as u64; r9 = rdi.wrapping_add(0x90); r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((rax as u32)) as u32); rdx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 58; continue; }
            58 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x1c0u64))) as u64; r9 = r14.wrapping_add(0x178); r8 = r14.wrapping_add(0x150); rdx = r14.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_and_u8(r14.wrapping_add(0x1f8u64), (0xfbu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 59; continue; }
            59 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 74; continue; } bb = 60; continue; }
            60 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xf8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rbx = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 72; continue; } bb = 61; continue; }
            61 => { rsi = rdx.wrapping_add(0x20); r14 = 0; fl_l = 0; fl_r = 0; bb = 62; continue; }
            62 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_sub(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 68; continue; } bb = 63; continue; }
            63 => { bb = 64; continue; }
            64 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 66; continue; } bb = 65; continue; }
            65 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 66; continue; }
            66 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_sub(0x8u64))) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 64; continue; } bb = 67; continue; }
            67 => { rdi = rdx; bb = 68; continue; }
            68 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 70; continue; } bb = 69; continue; }
            69 => { rdx = rdi; crate::simd_ops::wu32(rsi, ((r14 as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_sub(0x8u64), r14); crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((r14 as u32)) as u32); bb = 70; continue; }
            70 => { rsi = ((rsi).wrapping_add(0x28u64)) as u64; rax = rsi.wrapping_sub(0x20); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 62; continue; } bb = 71; continue; }
            71 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x150u64))) as u64; bb = 72; continue; }
            72 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0xf8u64), ((rax as u32)) as u32); rax = rbp.wrapping_sub(0x50); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 74; continue; } bb = 73; continue; }
            73 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 74; continue; }
            74 => { crate::simd_ops::lk_or_u8(r14.wrapping_add(0x1f8u64), (0x3u8) as u8); rcx = r14; rsp = ((rsp).wrapping_add(0x208u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141595b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x306; const P1: u64 = 0x305; const P2: u64 = 0x2fc; const P3: u64 = 0x14160c240;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = rcx;
    crate::simd_ops::lk_or_u8(rcx.wrapping_add(P1), (0xc0u8) as u8);
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64));
    fl_l = ((((rax as u8)) | ((r8 as u8)))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm1 as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    rcx = 0x5;
    return crate::simd_ops::tail_fwd_xmm(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141595b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x305; const P1: u64 = 0x306; const P2: u64 = 0x2fc; const P3: u64 = 0x14160c240;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::lk_and_u8(rcx.wrapping_add(P1), (0xfeu8) as u8);
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3fu8))) as u8 as u64));
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdx as u32)) as u32);
    fl_l = ((((rax as u8)) | (0x40u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    rdx = rcx;
    rcx = 0x5;
    return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141595bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438152ac; const P1: u64 = 0x306; const P2: u64 = 0x2fc; const P3: u64 = 0x305; const P4: u64 = 0x14160c240;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rax = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    r9 = (((r9) as i64).wrapping_mul((rax) as i64) as u64) as u64;
    rax = (0x20c49ba5e353f7cfu64) as u64;
    let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128;
    rax = prod as u64; rdx = (prod >> 64) as u64;
    rdx = ((((rdx) as i64).wrapping_shr(7u32) as u64)) as u64;
    rax = rdx;
    rax = ((rax).wrapping_shr(63u32)) as u64;
    rdx = ((rdx).wrapping_add(rax)) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64));
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdx as u32)) as u32);
    rax = (rax & !0xffu64) | ((((((rax as u8)) | ((r8 as u8)))) as u8 as u64));
    rdx = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3fu8))) as u8 as u64));
    fl_l = ((((rax as u8)) | (0x40u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8);
    rcx = 0x5;
    return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141595c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141594710;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r14 = r8; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbp = rdx; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rdi = rcx; r9 = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r9 = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((r8 as u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r15 = 0; fl_l = 0; fl_r = 0; rbx = rax.wrapping_add(0x10); fl_l = rax; fl_r = r9; rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { r15 } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); bb = 7; continue; }
            7 => { rax = rbx; bb = 14; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rax as u32)) as u32 as u64; rcx = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); rcx = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((rax).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 14; continue; }
            13 => { rax = r15; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141595d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x141595c20; const P2: u64 = 0x238; const P3: u64 = 0x240; const P4: u64 = 0x8; const P5: u64 = 0x18; const P6: u64 = 0x14183acc8; const P7: u64 = 0x78;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((xmm2 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; rdi = rcx; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = rsp.wrapping_add(0x50); rcx = ((rcx).wrapping_add(0x238u64)) as u64; rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; fl_l = (((rbx as u32)).wrapping_add(0x2u32)) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; bb = 12; continue; }
            2 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r11 = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rcx = r10; rax = ((r11.wrapping_add(r11.wrapping_mul(2))) as u64) as u64; rax = ((r10.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r10; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((r9 as u32)) as u32 as u64; rdx = ((((rdx as u32)) ^ (crate::simd_ops::ru32(rcx.wrapping_add(P4))))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = ((r11.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((r10.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = ((rcx.wrapping_add(P5)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(P3), (1) as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rbx = 0x1; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rcx = ((rdi.wrapping_add(P5)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141595e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x303; const P1: u64 = 0x1d8; const P2: u64 = 0x1415c0bf0; const P3: u64 = 0x288; const P4: u64 = 0x240;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; rbx = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x80u8))) as u8 as u64)); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = (rdx & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u8(rbx.wrapping_add(P0), (0x80u8) as u8); rcx = (((rdx as u8)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfu8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shl(3u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::lk_or_u8(rbx.wrapping_add(P0), ((rcx as u8)) as u8); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = ((rax as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141595eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x302;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(P0), (0x87u8) as u8)) as u8 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141595ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141593f10; const P1: u64 = 0x143930f80; const P2: u64 = 0x1415adef0; const P3: u64 = 0x1415242e0; const P4: u64 = 0x14159dbb0; const P5: u64 = 0x1415c6480; const P6: u64 = 0x14160d400; const P7: u64 = 0x1415c6110; const P8: u64 = 0x141592ae0; const P9: u64 = 0x143930f90; const P10: u64 = 0x14159b2e0; const P11: u64 = 0x14160c070;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx.wrapping_sub(0x18); rbx = rcx; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x260u64))) as u64; rdi = rbx.wrapping_sub(0x8); rbp = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rsi; fl_r = rsi; r8 = rdi; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x268u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rsi; fl_r = rsi; rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rdi }) as u64; r8 = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x2ecu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((((rax as u8)) & (0xfeu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x2ecu64), ((rax as u8)) as u8); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x220u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x228u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x220u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x22cu64), ((rbp as u32)) as u32); bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x1c0u64))) as u64; fl_l = rsi; fl_r = rsi; rcx = rbx; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rcx }) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x70u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 9; continue; }
            9 => { r10 = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; rdx = rbp; rax = r10; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rdx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { bb = 18; continue; }
            14 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(r8.wrapping_add(0x18u64), rcx); bb = 18; continue; }
            17 => { crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), rcx); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x70u64))) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd0u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 25; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xd4u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if ((((fl_l as u16)) as i16) > (((fl_r as u16)) as i16)) { bb = 25; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xb8u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 25; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0xbau64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 25; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = r9; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x278u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdx = rbx.wrapping_add(0x318); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x278u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x340u64))) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32 as u64; rdi = (((rdi as u32)).wrapping_shr(8u32)) as u32 as u64; rdi = (rdi & !0xffu64) | ((((((rdi as u8)) & (0x1u8))) as u8 as u64)); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 28; continue; }
            28 => { r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; r8 = rbp; rcx = r9; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 29; continue; }
            29 => { bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 31; continue; }
            31 => { r8 = rcx; rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 32; continue; }
            32 => { bb = 39; continue; }
            33 => { fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu64(rdx.wrapping_add(0x48u64), rax); bb = 36; continue; }
            35 => { crate::simd_ops::wu64(r8, rax); bb = 36; continue; }
            36 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 37; continue; }
            37 => { crate::simd_ops::wu64(rdx.wrapping_add(0x50u64), r8); bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rdx.wrapping_add(0x6cu64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x58u64), rcx); bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x6cu64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 43; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x94u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 43; continue; }
            43 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x340u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x340u64), rbp); bb = 44; continue; }
            44 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 47; continue; } bb = 45; continue; }
            45 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x278u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 47; continue; }
            47 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x258u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { rcx = (crate::simd_ops::ru64(P9)) as u64; r8 = rsi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 49; continue; }
            49 => { rdx = rbx.wrapping_add(0x8); rcx = rbx.wrapping_add(0x208); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141596120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141591570; const P1: u64 = 0x8; const P2: u64 = 0xffffffffff000000; const P3: u64 = 0xfffffffffc000000; const P4: u64 = 0x270; const P5: u64 = 0xfffffffffffffff0; const P6: u64 = 0x18; const P7: u64 = 0x2a4; const P8: u64 = 0x268; const P9: u64 = 0x2a0; const P10: u64 = 0x200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rcx = ((rcx).wrapping_add(0x8u64)) as u64; rbp = (((r9 as u8)) as u32) as u32 as u64; xmm6 = (xmm2) as u128; rsi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rdi.wrapping_add(P1)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rsi.wrapping_add(P2)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfeffffff; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = ((rsi.wrapping_add(P3)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfbffffff; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P4), (0x0u64) as u64); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x4000000; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rdi.wrapping_add(P7), ((xmm6 as u32)) as u32); bb = 10; continue; }
            6 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdi.wrapping_add(P8), (0x0u64) as u64); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x2000000; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rcx = ((rdi.wrapping_add(P5)) as u64) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(P9), ((xmm6 as u32)) as u32); bb = 10; continue; }
            10 => { crate::simd_ops::lk_or_u8(rdi.wrapping_add(P10), (0x10u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141596200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x1415c2440; const P2: u64 = 0x148; const P3: u64 = 0x30c; const P4: u64 = 0x350; const P5: u64 = 0x348; const P6: u64 = 0x358; const P7: u64 = 0x30; const P8: u64 = 0x20; const P9: u64 = 0x328; const P10: u64 = 0x270; const P11: u64 = 0x143930f70; const P12: u64 = 0x1415d99c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = rcx; rbp = (((r8 as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rbp as u32)) as u32 as u64; rax = ((((rax as u32)) ^ (0x1u32))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((rax as u32)) as u32); bb = 6; continue; }
            5 => { crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x0u32) as u32); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P11)) as u64; r9 = rbx.wrapping_add(0x90); r8 = rsp.wrapping_add(0x20); rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415962e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rcx = r10; r11 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = ((r11.wrapping_add(r11.wrapping_mul(2))) as u64) as u64; rax = ((r10.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r10; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((r9 as u32)) as u32 as u64; rdx = ((((rdx as u32)) ^ (crate::simd_ops::ru32(rcx.wrapping_add(P0))))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = ((r11.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((r10.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = ((rcx.wrapping_add(P1)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141596380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x210; const P1: u64 = 0x298; const P2: u64 = 0x141602c20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } bb = 2; continue; }
            2 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415963a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141535e10; const P1: u64 = 0x143931600; const P2: u64 = 0x14159c7c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x370u64))) as u32 as u64; fl_r = 0x0; rdi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x210u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rdx = rsp.wrapping_add(0x20); xmm0 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rbx = rcx.wrapping_add(0x360); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1d8u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x1u32) as u32); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rsp.wrapping_add(0x48u64), (0x100u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x108u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = P1; crate::simd_ops::wu64(rdx, rax); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x368u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x370u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x306u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::lk_add_u16(rax.wrapping_add(0x2au64), (1) as u16)) as u16 as u64; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x370u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x368u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415964b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ea4; const P1: u64 = 0x310; const P2: u64 = 0x370; const P3: u64 = 0x368; const P4: u64 = 0x1415a3c90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); rdi = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm6 = (xmm1) as u128; xmm7 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm8 = 0u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) > f32::from_bits(xmm8 as u32) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(xmm8 as u32) }).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) < f32::from_bits(xmm7 as u32) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rdi; xmm1 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) > f32::from_bits(xmm8 as u32) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(xmm8 as u32) }).to_bits() as u128); fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm7 = (xmm6) as u128; bb = 6; continue; }
            6 => { rax = ((xmm7 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(P1), ((xmm7 as u32)) as u32); bb = 8; continue; }
            8 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141596580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1f8; const P1: u64 = 0x308; const P2: u64 = 0x8; const P3: u64 = 0x304; const P4: u64 = 0xffffffffffffffe8; const P5: u64 = 0x1415964b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; xmm3 = (xmm2) as u128; r8 = rcx; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = ((rax).wrapping_add(0x2u64)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32) as u128; xmm1 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x2u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)))) as u32) as u128; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 8; continue; }
            7 => { xmm1 = (xmm0) as u128; bb = 8; continue; }
            8 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(P3)))).to_bits() as u128); rcx = ((r8.wrapping_add(P4)) as u64) as u64; return crate::simd_ops::tail_fwd_xmm(P5, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); }
            9 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141596600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141591570; const P1: u64 = 0x140000000; const P2: u64 = 0x1415964b0; const P3: u64 = 0x141602c00; const P4: u64 = 0x143930f44; const P5: u64 = 0x1415317a0; const P6: u64 = 0x141592e90;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rsi = ((r9 as u32)) as u32 as u64; rdi = ((r8 as u32)) as u32 as u64; rbp = rdx; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rdi.wrapping_sub(0x5u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x36; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 25; continue; } bb = 1; continue; }
            1 => { rcx = P1; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rax).wrapping_add(0x159682cu64))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1596818u64))) as u32 as u64; fl_l = ((r9).wrapping_add(rcx)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; j_tgt = r9; bb = 0xffff_fffe; continue; }
            2 => { xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x304u64))) as u32) as u128; rcx = rbx.wrapping_sub(0x18); xmm6 = (xmm1) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x2f8u64)))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((xmm6 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbx.wrapping_add(0x304u64), ((xmm6 as u32)) as u32); bb = 20; continue; }
            4 => { rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64)))))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x21; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x280u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 6; continue; }
            6 => { rdx = ((rdi.wrapping_sub(0x1du64)) as u32) as u32 as u64; r8 = (((rax as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u16(rbx.wrapping_add(0x13cu64), ((rax as u16)) as u16)) as u16 as u64; fl_r = fl_l; bb = 20; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x124u64))) as u32 as u64; fl_r = 0x1000000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); xmm7 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32) as u128; fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 11; continue; }
            11 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0xa0u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); rcx = rbx.wrapping_add(0xf0); crate::simd_ops::wu16(rsp.wrapping_add(0x30u64), (0x18u16) as u16); r8 = rsp.wrapping_add(0xa0); crate::simd_ops::wu8(rsp.wrapping_add(0x32u64), ((rdi as u8)) as u8); rdx = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; xmm6 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x34u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xa0u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rdi as u32)) as u32); bb = 14; continue; }
            14 => { xmm7 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; xmm6 = (xmm7) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm8 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x130u64))) as u32 as u64; fl_r = 0x1000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xb4u64))) as u32) as u128; fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 21; continue; } bb = 17; continue; }
            17 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x13fu64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 19; continue; }
            19 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            21 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 22; continue; }
            22 => { if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 19; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu32(rbx.wrapping_add(0xb4u64), ((xmm6 as u32)) as u32); bb = 19; continue; }
            24 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0xb4u64)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0xb4u64), ((xmm0 as u32)) as u32); bb = 20; continue; }
            25 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32) as u128; r9 = ((rsi as u32)) as u32 as u64; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32) as u128; r8 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); rdx = rbp; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x141596600u64) { 0x2e => { bb = 1; continue; } 0x4b => { bb = 2; continue; } 0x87 => { bb = 3; continue; } 0x94 => { bb = 4; continue; } 0xa2 => { bb = 5; continue; } 0xb2 => { bb = 6; continue; } 0xc3 => { bb = 7; continue; } 0xcf => { bb = 8; continue; } 0xdc => { bb = 9; continue; } 0xec => { bb = 10; continue; } 0x10e => { bb = 11; continue; } 0x156 => { bb = 12; continue; } 0x160 => { bb = 13; continue; } 0x163 => { bb = 14; continue; } 0x175 => { bb = 15; continue; } 0x181 => { bb = 16; continue; } 0x18e => { bb = 17; continue; } 0x193 => { bb = 18; continue; } 0x19a => { bb = 19; continue; } 0x1a5 => { bb = 20; continue; } 0x1ba => { bb = 21; continue; } 0x1bd => { bb = 22; continue; } 0x1bf => { bb = 23; continue; } 0x1c9 => { bb = 24; continue; } 0x1e4 => { bb = 25; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141596870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141591390; const P1: u64 = 0x141605350; const P2: u64 = 0x141597e90; const P3: u64 = 0x1416072d0; const P4: u64 = 0x141606ea0; const P5: u64 = 0x141592aa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; crate::simd_ops::wu32(rcx.wrapping_add(0x17cu64), (0x42ca0000u32) as u32); rcx = ((rcx).wrapping_add(0x190u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x1a4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x210u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1c0u64))) as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x1c8u64))) as u32 as u64; bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; rsi = ((crate::simd_ops::ru16(rdx.wrapping_add(0x38u64))) as u32) as u32 as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; xmm6 = ((crate::simd_ops::ru32(rax.wrapping_add(0x78u64))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdx = ((rsi as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = rdi.wrapping_add(0x18); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm6 as u32)).to_bits() as u128); rdx = rbp; xmm2 = (xmm0) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x88u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x1d0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x158u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru32(rax.wrapping_add(0x30u64))) as u32 as u64; rbx = (((rbx as u32)).wrapping_shr(31u32)) as u32 as u64; rdx = (((rbx as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x158u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}
